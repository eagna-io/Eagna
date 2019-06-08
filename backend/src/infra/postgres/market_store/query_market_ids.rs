use crate::{
    domain::models::{market::MarketId, user::UserId},
    infra::postgres::types::{MarketStatus, OrderType},
};
use diesel::{pg::PgConnection, prelude::*, result::Error as PgError};

/// 以下の条件を満たすMarketのIDを返す
/// 1. status が Preparing
/// 2. 対象のUserに対してInitialSupplyを配布している
pub fn query_market_ids_related_to_user(
    conn: &PgConnection,
    user_id: &UserId,
) -> Result<Vec<MarketId>, PgError> {
    let preparing_market_ids = query_preparing_market_ids(conn)?;
    let participated_market_ids = query_participated_market_ids(conn, user_id)?;

    Ok(preparing_market_ids
        .into_iter()
        .chain(participated_market_ids)
        .map(|id| MarketId(id))
        .collect())
}

fn query_preparing_market_ids(conn: &PgConnection) -> Result<Vec<i32>, PgError> {
    use crate::infra::postgres::schema::markets::{columns as market, table as markets};

    markets
        .select(market::id)
        .filter(market::status.eq(MarketStatus::Preparing))
        .load::<i32>(conn)
}

fn query_participated_market_ids(
    conn: &PgConnection,
    user_id: &UserId,
) -> Result<Vec<i32>, PgError> {
    use crate::infra::postgres::schema::orders::{columns as order, table as orders};

    orders
        .select(order::market_id)
        .filter(order::user_id.eq(user_id.as_str()))
        .filter(order::type_.eq(OrderType::InitialSupply))
        .load::<i32>(conn)
}

pub fn query_market_ids_ready_to_open(conn: &PgConnection) -> Result<Vec<MarketId>, PgError> {
    use crate::infra::postgres::schema::markets::{columns as market, table as markets};

    Ok(markets
        .select(market::id)
        .filter(market::status.eq(MarketStatus::Preparing))
        .filter(market::open_time.lt(diesel::dsl::now))
        .load::<i32>(conn)?
        .into_iter()
        .map(|id| MarketId(id))
        .collect())
}

pub fn query_market_ids_ready_to_close(conn: &PgConnection) -> Result<Vec<MarketId>, PgError> {
    use crate::infra::postgres::schema::markets::{columns as market, table as markets};

    Ok(markets
        .select(market::id)
        .filter(market::status.eq(MarketStatus::Open))
        .filter(market::close_time.lt(diesel::dsl::now))
        .load::<i32>(conn)?
        .into_iter()
        .map(|id| MarketId(id))
        .collect())
}
