use crate::domain::{
    market::{
        num::{AmountCoin, AmountToken},
        order::{NormalOrder, Order},
        AbstractMarket, ClosedMarket, ResolvedMarket, RewardRecords,
    },
    user::UserId,
};
use crate::primitive::NonEmptyString;
use std::collections::HashMap;

const REWARD_COIN_PER_TOKEN: u32 = 1000;

/// ClosedMarketのメソッドとして持たせるには機能が大きすぎるので分離。
pub fn resolve_market_uncheck(
    market: ClosedMarket,
    resolved_token_name: NonEmptyString,
) -> ResolvedMarket {
    assert!(market.attrs.is_valid_token(&resolved_token_name));

    let point_coin_rate = match market.point_coin_rate() {
        Ok(rate) => rate,
        Err(_) => {
            return ResolvedMarket {
                id: market.id,
                attrs: market.attrs,
                orders: market.orders,
                token_distribution: market.token_distribution,
                resolved_token_name,
                reward_records: RewardRecords(HashMap::new()),
            };
        }
    };

    // 当たりトークンを持っているユーザーにReward Orderを発行する
    compute_users_token_amount(&market, &resolved_token_name)
        .into_iter()
        .for_each(|(user_id, amount_token)| {
            // 各ユーザーにReward Orderを発行
            market.orders.add_reward_order(
                user_id,
                resolved_token_name.clone(),
                AmountCoin(REWARD_COIN_PER_TOKEN as i32 * amount_token.as_i32()),
            );
        });

    // ユーザーに報酬ポイントを配布
    let mut rng = rand::thread_rng();
    let users_reward_point = compute_users_coin_amount(&market)
        .into_iter()
        .map(|(user, coin)| {
            let (int_point, fract_point) = point_coin_rate * coin;
            let point = int_point + fract_point.to_integer_with_probability(&mut rng);
            (user, point)
        })
        .collect::<HashMap<_, _>>();
    let reward_records = RewardRecords(users_reward_point);

    ResolvedMarket {
        id: market.id,
        attrs: market.attrs,
        orders: market.orders,
        token_distribution: market.token_distribution,
        resolved_token_name,
        reward_records,
    }
}

// 各ユーザーが持っている対象トークンの量を計算
fn compute_users_token_amount(
    market: &ClosedMarket,
    token_name: &NonEmptyString,
) -> HashMap<UserId, AmountToken> {
    let mut user_token_map = HashMap::new();

    market
        .orders
        .iter()
        .filter_map(|order| filter_map_related_normal_order(order, token_name))
        .for_each(|order| {
            let cur_token_amount = user_token_map
                .entry(*order.user_id())
                .or_insert(AmountToken(0));
            *cur_token_amount += *order.amount_token();
        });

    user_token_map
}

// 対象のTokenを取引したNormalOrderのみを抽出
fn filter_map_related_normal_order<'a>(
    order: &'a Order,
    token_name: &'a NonEmptyString,
) -> Option<&'a NormalOrder> {
    match order {
        Order::Normal(ref n) if n.token_name() == token_name => Some(n),
        _ => None,
    }
}

// 各ユーザーが持っているコイン量を計算
fn compute_users_coin_amount(market: &ClosedMarket) -> HashMap<UserId, AmountCoin> {
    let mut user_coin_map = HashMap::new();

    market.orders.iter().for_each(|order| {
        let cur_amount = user_coin_map
            .entry(*order.user_id())
            .or_insert(AmountCoin(0));
        *cur_amount += *order.amount_coin();
    });

    user_coin_map
}
