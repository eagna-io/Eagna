use crate::{
    domain::models::{
        market::{MarketId, MarketStatus},
        user::UserId,
    },
    infra::postgres::types::{MarketStatus as PgMarketStatus, OrderType},
};
use diesel::{pg::PgConnection, prelude::*, result::Error as PgError};

pub fn query_market_ids_with_status<I>(
    conn: &PgConnection,
    status_iter: I,
) -> Result<Vec<MarketId>, PgError>
where
    I: Iterator<Item = MarketStatus>,
{
    use crate::infra::postgres::schema::markets::{columns as market, table as markets};

    let init_query = markets.select(market::id).into_boxed();
    Ok(status_iter
        .map(|s| match s {
            MarketStatus::Preparing => PgMarketStatus::Preparing,
            MarketStatus::Open => PgMarketStatus::Open,
            MarketStatus::Closed => PgMarketStatus::Closed,
            MarketStatus::Settled => PgMarketStatus::Settled,
        })
        .fold(init_query, |query, status| {
            query.or_filter(market::status.eq(status))
        })
        .load::<i32>(conn)?
        .into_iter()
        .map(|id| MarketId(id))
        .collect())
}

/// 以下の条件を満たすMarketのIDを返す
/// - 対象のUserに対してInitialSupplyを配布している
pub fn query_market_ids_related_to_user(
    conn: &PgConnection,
    user_id: &UserId,
) -> Result<Vec<MarketId>, PgError> {
    use crate::infra::postgres::schema::orders::{columns as order, table as orders};

    Ok(orders
        .select(order::market_id)
        .filter(order::user_id.eq(user_id.as_str()))
        .filter(order::type_.eq(OrderType::InitialSupply))
        .load::<i32>(conn)?
        .into_iter()
        .map(|id| MarketId(id))
        .collect())
}

pub fn query_market_ids_ready_to_open(conn: &PgConnection) -> Result<Vec<MarketId>, PgError> {
    use crate::infra::postgres::schema::markets::{columns as market, table as markets};

    Ok(markets
        .select(market::id)
        .filter(market::status.eq(PgMarketStatus::Preparing))
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
        .filter(market::status.eq(PgMarketStatus::Open))
        .filter(market::close_time.lt(diesel::dsl::now))
        .load::<i32>(conn)?
        .into_iter()
        .map(|id| MarketId(id))
        .collect())
}
