use crate::domain::{
    market::{
        num::{AmountCoin, AmountToken},
        order::Order,
        ClosedMarket, ResolvedMarket,
    },
    user::UserId,
};
use crate::primitive::NonEmptyString;
use std::collections::HashMap;

const REWARD_COIN_PER_TOKEN: u32 = 1000;

/// ClosedMarketのメソッドとして持たせるには機能が大きすぎるので分離。
pub fn resolve_market_uncheck(
    mut market: ClosedMarket,
    resolved_token_name: NonEmptyString,
) -> ResolvedMarket {
    assert!(market.attrs.is_valid_token(&resolved_token_name));

    let reward_records = todo!();

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
        .filter(|order| order.token_name() == token_name)
        .for_each(|order| {
            let cur_token_amount = user_token_map
                .entry(*order.user_id())
                .or_insert(AmountToken(0));
            *cur_token_amount += *order.amount_token();
        });

    user_token_map
}
