use crate::{
    api::FailureResponse,
    postgres::{MarketStatus, OrderType},
    Server,
};
use chrono::Utc;
use diesel::{pg::PgConnection as PgConn, result::Error as PgError, Connection};
use rouille::{Request, Response};

pub const INITIAL_SUPPLY_COIN: i32 = 10000;

pub fn get(server: &Server, req: &Request) -> Result<Response, FailureResponse> {
    // 特定のソースからのリクエストかチェック
    // gcp app engine によるcron jobリクエストは10.0.0.1から
    // 開発環境によるcron jobリクエストはloopbackアドレスから
    let source = req.remote_addr().ip();
    if !source.is_loopback() && source != std::net::Ipv4Addr::new(10, 0, 0, 1) {
        return Err(FailureResponse::ResourceNotFound);
    }

    let pg_conn = server.get_new_pg_conn()?;
    let market_ids = query_new_open_market_ids(&pg_conn)?;
    if market_ids.is_empty() {
        return Ok(Response::text("No market is opened"));
    }

    let user_ids = query_user_ids(&pg_conn)?;

    for market_id in market_ids.iter() {
        pg_conn
            .transaction(|| {
                distribute_initial_coin(&pg_conn, *market_id, &user_ids)?;
                open_market(&pg_conn, *market_id)
            })
            .map_err(|_| FailureResponse::ServerError)?;
    }

    Ok(Response::json(&market_ids))
}

fn query_new_open_market_ids(conn: &PgConn) -> Result<Vec<i32>, FailureResponse> {
    use crate::postgres::schema::markets::{columns as market, table as markets};
    use diesel::prelude::*;

    markets
        .select(market::id)
        .filter(
            market::status
                .eq(MarketStatus::Preparing)
                .and(market::open_time.lt(Utc::now())),
        )
        .load::<i32>(conn)
        .map_err(|_| FailureResponse::ServerError)
}

fn query_user_ids(conn: &PgConn) -> Result<Vec<i32>, FailureResponse> {
    use crate::postgres::schema::users::{columns as user, table as users};
    use diesel::prelude::*;

    users
        .select(user::id)
        .load::<i32>(conn)
        .map_err(|_| FailureResponse::ServerError)
}

fn distribute_initial_coin(
    conn: &PgConn,
    market_id: i32,
    user_ids: &Vec<i32>,
) -> Result<(), PgError> {
    use crate::postgres::schema::orders;
    use diesel::prelude::*;

    #[derive(Debug, Insertable)]
    #[table_name = "orders"]
    struct NewOrder {
        user_id: i32,
        market_id: i32,
        amount_token: i32,
        amount_coin: i32,
        type_: OrderType,
    }

    let new_orders: Vec<NewOrder> = user_ids
        .iter()
        .map(|user_id| NewOrder {
            user_id: *user_id,
            market_id,
            amount_token: 0,
            amount_coin: INITIAL_SUPPLY_COIN,
            type_: OrderType::InitialSupply,
        })
        .collect();

    diesel::insert_into(orders::table)
        .values(&new_orders)
        .execute(conn)
        .map(|_| ())
}

fn open_market(conn: &PgConn, market_id: i32) -> Result<(), PgError> {
    use crate::postgres::schema::markets::{columns as market, table as markets};
    use diesel::prelude::*;

    diesel::update(markets.filter(market::id.eq(market_id)))
        .set(market::status.eq(MarketStatus::Open))
        .execute(conn)
        .map(|_| ())
}
