use crate::{
    domain::{
        models::market::*,
        services::market_store::{UpdateMarketLastOrderResult, UpdateMarketStatusResult},
    },
    infra::postgres::{MarketStatus, OrderType},
};
use chrono::{DateTime, Utc};
use diesel::{pg::PgConnection, prelude::*, result::Error as PgError};

pub fn update_market_last_order(
    conn: &PgConnection,
    target: &OpenMarket,
) -> UpdateMarketLastOrderResult<PgError> {
    use crate::infra::postgres::schema::{markets, orders};

    let (serial_num, last_order) = target.last_normal_order().unwrap();

    // 対象マーケットがDB上でもOpenであることをチェックする
    // ロックを獲得し、現在のトランザクションが終了するまではOpenであることを保証する
    match markets::table
        .select(markets::status)
        .filter(markets::id.eq(target.base.id.0))
        .filter(markets::status.eq(MarketStatus::Open))
        .for_update()
        .first::<MarketStatus>(conn)
    {
        Ok(_) => {}
        Err(PgError::NotFound) => return UpdateMarketLastOrderResult::NotOpen,
        Err(e) => return UpdateMarketLastOrderResult::Error(e),
    }

    // Orderを記録
    match diesel::insert_into(orders::table)
        .values(NewOrder::normal(target.base.id, serial_num, *last_order))
        .execute(conn)
    {
        Ok(_) => {}
        Err(e) => return UpdateMarketLastOrderResult::Error(e),
    }

    UpdateMarketLastOrderResult::Success
}

pub fn update_market_status_to_open(
    conn: &PgConnection,
    target: &OpenMarket,
) -> UpdateMarketStatusResult<PgError> {
    use crate::infra::postgres::schema::{markets, orders};

    // 対象マーケットがDB上でPreparing状態であることをチェックし、
    // status行に対するロックを獲得する。
    match markets::table
        .select(markets::status)
        .filter(markets::id.eq(target.base.id.0))
        .filter(markets::status.eq(MarketStatus::Preparing))
        .for_update()
        .first::<MarketStatus>(conn)
    {
        Ok(_) => {}
        Err(PgError::NotFound) => return UpdateMarketStatusResult::MarketNotFound,
        Err(e) => return UpdateMarketStatusResult::Error(e),
    }

    // Orderを記録
    let order_records: Vec<NewOrder> = target
        .orders
        .iter()
        .map(|(serial_num, order)| match order {
            Order::InitialSupply(o) => NewOrder::initial_supply(target.base.id, serial_num, *o),
            _ => panic!("A new opened market contains non-initial-supply order"),
        })
        .collect();
    match diesel::insert_into(orders::table)
        .values(order_records)
        .execute(conn)
    {
        Ok(_) => {}
        Err(e) => return UpdateMarketStatusResult::Error(e),
    }

    // Market の status を open に変更
    match diesel::update(markets::table.filter(markets::id.eq(target.base.id.0)))
        .set(markets::status.eq(MarketStatus::Open))
        .execute(conn)
    {
        Ok(_) => UpdateMarketStatusResult::Success,
        Err(e) => UpdateMarketStatusResult::Error(e),
    }
}

pub fn update_market_status_to_closed(
    conn: &PgConnection,
    target: &ClosedMarket,
) -> UpdateMarketStatusResult<PgError> {
    use crate::infra::postgres::schema::markets;

    // 対象マーケットがDB上でOpen状態であることをチェックし、
    // status行に対するロックを獲得する。
    match markets::table
        .select(markets::status)
        .filter(markets::id.eq(target.base.id.0))
        .filter(markets::status.eq(MarketStatus::Open))
        .for_update()
        .first::<MarketStatus>(conn)
    {
        Ok(_) => {}
        Err(PgError::NotFound) => return UpdateMarketStatusResult::MarketNotFound,
        Err(e) => return UpdateMarketStatusResult::Error(e),
    }

    // Market の status を closed に変更
    match diesel::update(markets::table.filter(markets::id.eq(target.base.id.0)))
        .set(markets::status.eq(MarketStatus::Closed))
        .execute(conn)
    {
        Ok(_) => UpdateMarketStatusResult::Success,
        Err(e) => UpdateMarketStatusResult::Error(e),
    }
}

use crate::infra::postgres::schema::orders;
#[derive(Insertable)]
#[table_name = "orders"]
struct NewOrder {
    market_id: i32,
    market_internal_serial_num: i32,
    user_id: i32,
    token_id: Option<i32>,
    amount_token: i32,
    amount_coin: i32,
    type_: OrderType,
    time: DateTime<Utc>,
}

impl NewOrder {
    fn normal(market_id: MarketId, serial_num: OrderId, order: NormalOrder) -> NewOrder {
        NewOrder {
            market_id: market_id.0,
            market_internal_serial_num: serial_num.0,
            user_id: order.user_id.0,
            token_id: Some(order.token_id.0),
            amount_token: order.amount_token.0,
            amount_coin: order.amount_coin.0,
            type_: OrderType::Normal,
            time: order.time,
        }
    }

    fn initial_supply(
        market_id: MarketId,
        serial_num: OrderId,
        order: InitialSupplyOrder,
    ) -> NewOrder {
        NewOrder {
            market_id: market_id.0,
            market_internal_serial_num: serial_num.0,
            user_id: order.user_id.0,
            token_id: None,
            amount_token: 0,
            amount_coin: order.amount_coin.0,
            type_: OrderType::InitialSupply,
            time: order.time,
        }
    }
}
