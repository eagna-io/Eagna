mod insert_market;
mod query_market;
mod query_market_ids;

pub use insert_market::insert_market;
pub use query_market::query_market;
pub use query_market_ids::{
    query_market_ids_ready_to_close, query_market_ids_ready_to_open,
    query_market_ids_related_to_user, query_market_ids_with_status,
};

use crate::{
    domain::models::market::*,
    infra::postgres::types::{MarketStatus as PgMarketStatus, OrderType},
};
use chrono::{DateTime, Utc};
use diesel::{pg::PgConnection, prelude::*};

pub fn lock_market(conn: &PgConnection, market_id: &MarketId) -> Result<(), failure::Error> {
    use crate::infra::postgres::schema::markets;
    markets::table
        .select(markets::id)
        .filter(markets::id.eq(market_id.0))
        .for_update()
        .first::<i32>(conn)?;
    Ok(())
}

pub fn update_market_status(
    conn: &PgConnection,
    market_id: &MarketId,
    status: &MarketStatus,
) -> Result<(), failure::Error> {
    use crate::infra::postgres::schema::markets;
    diesel::update(markets::table.filter(markets::id.eq(market_id.0)))
        .set(markets::status.eq(PgMarketStatus::from(*status)))
        .execute(conn)?;
    Ok(())
}

pub fn insert_market_orders<'a, I>(
    conn: &PgConnection,
    market_id: &MarketId,
    orders: I,
) -> Result<(), failure::Error>
where
    I: Iterator<Item = (OrderId, &'a Order)>,
{
    use crate::infra::postgres::schema::orders;
    #[derive(Insertable)]
    #[table_name = "orders"]
    struct NewOrder<'b> {
        market_id: i32,
        market_internal_serial_num: i32,
        user_id: &'b str,
        token_id: Option<i32>,
        amount_token: i32,
        amount_coin: i32,
        type_: OrderType,
        time: DateTime<Utc>,
    }

    let order_records: Vec<NewOrder<'a>> = orders
        .map(|(serial_num, order)| {
            let (order_type, maybe_token_id) = match order {
                Order::InitialSupply(_) => (OrderType::InitialSupply, None),
                Order::Normal(o) => (OrderType::Normal, Some(o.token_id.0)),
                Order::Settle(o) => (OrderType::Settle, Some(o.token_id.0)),
            };
            NewOrder {
                market_id: market_id.0,
                market_internal_serial_num: serial_num.0,
                user_id: order.user_id().as_str(),
                token_id: maybe_token_id,
                amount_token: order.amount_token().0,
                amount_coin: order.amount_coin().0,
                type_: order_type,
                time: *order.time(),
            }
        })
        .collect();

    diesel::insert_into(orders::table)
        .values(order_records)
        .execute(conn)?;
    Ok(())
}
