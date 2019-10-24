use crate::domain::{
    market::{
        num::{AmountCoin, AmountToken},
        order::{NormalOrder, Order},
        ClosedMarket, ResolvedMarket, RewardRecords, INITIAL_SUPPLY_COIN,
    },
    point::Point,
    user::UserId,
};
use crate::primitive::NonEmptyString;
use num_rational::Ratio;
use rand::Rng;
use std::{collections::HashMap, iter::FromIterator};

const REWARD_COIN_PER_TOKEN: u32 = 1000;

/// ClosedMarketのメソッドとして持たせるには機能が大きすぎるので分離。
pub fn resolve_market_uncheck(
    market: ClosedMarket,
    resolved_token_name: NonEmptyString,
) -> ResolvedMarket {
    assert!(market.attrs().is_valid_token(&resolved_token_name));

    let point_coin_ratio = match compute_point_coin_ratio(&market) {
        Ok(ratio) => ratio,
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
            market.orders_mut().add_reward_order(
                user_id,
                resolved_token_name.clone(),
                AmountCoin(REWARD_COIN_PER_TOKEN as i32 * amount_token.as_i32()),
            );
        });

    // 参加した全ユーザーの保有コイン量を計算
    let users_reward_point_iter = compute_users_coin_amount(&market)
        .into_iter()
        .map(|(user, coin)| (user, determine_user_reward_point(point_coin_ratio, coin)));

    // 各ユーザーの獲得ポイントを記録
    let mut users_reward_point = HashMap::from_iter(users_reward_point_iter);

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

// 1コインあたりのポイント比率を計算する
fn compute_point_coin_ratio(market: &ClosedMarket) -> Result<Ratio<u32>, NoUserError> {
    // マーケットで発行された総コイン量を計算する
    // 参加ユーザー数 * InitialSupplyCoin
    let user_num = market.orders().num_users();
    if user_num == 0 {
        return Err(NoUserError());
    }
    let total_issued_coin = INITIAL_SUPPLY_COIN * user_num as i32;

    let reward_point = *market.attrs().total_reward_point();

    Ok(Ratio::new(
        reward_point.as_u32(),
        total_issued_coin.as_i32() as u32,
    ))
}

struct NoUserError();

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

    market.orders().iter().for_each(|order| {
        let cur_amount = user_coin_map
            .entry(*order.user_id())
            .or_insert(AmountCoin(0));
        *cur_amount += *order.amount_coin();
    });

    user_coin_map
}

fn determine_user_reward_point(point_coin_ratio: Ratio<u32>, coin: AmountCoin) -> Point {
    let reward_point = point_coin_ratio * coin.as_i32() as u32;
    let int_reward_point = reward_point.to_integer();
    let fract_reward_point = reward_point.fract();
    let rng = rand::thread_rng();
    if rng.gen_ratio(*fract_reward_point.numer(), *fract_reward_point.denom()) {
        Point::from(int_reward_point + 1)
    } else {
        Point::from(int_reward_point)
    }
}
