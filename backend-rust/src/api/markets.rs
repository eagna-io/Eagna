pub mod orders;

use crate::{api::FailureResponse, postgres::MarketStatus, Server};
use chrono::{DateTime, Utc};
use diesel::{pg::PgConnection as PgConn, result::Error as PgError};
use rouille::{Request, Response};

pub fn get(server: &Server, _req: &Request, market_id: i32) -> Result<Response, FailureResponse> {
    let pg_conn = server.get_new_pg_conn()?;
    let market = query_market(&pg_conn, market_id)?;
    Ok(Response::json(&market))
}

fn get<MS>(market_store: &MS, market_id: MarketId) -> Result<Response, FailureResponse> {
    let market = market_store.find_market(market_id).map_err(|e| match e {
        FindMarketError::NotFound => FailureResponse::ResourceNotFound,
        FindMarketError::InternalError => FailureResponse::ServerError,
    })?;
    Ok(Response::json(&RespData::from(market)))
}

#[derive(Debug, Serialize, Queryable)]
struct RespData {
    title: String,
    organizer: String,
    short_desc: String,
    description: String,
    open_time: DateTime<Utc>,
    close_time: DateTime<Utc>,
    status: MarketStatus,
    settle_token_id: Option<i32>,
}

fn query_market(conn: &PgConn, market_id: i32) -> Result<Market, FailureResponse> {
    use crate::postgres::schema::markets::{columns as market, table as markets};
    use diesel::prelude::*;

    markets
        .select((
            market::title,
            market::organizer,
            market::short_desc,
            market::description,
            market::open_time,
            market::close_time,
            market::status,
            market::settle_token_id,
        ))
        .filter(market::id.eq(market_id))
        .first::<Market>(conn)
        .map_err(|e| match e {
            PgError::NotFound => FailureResponse::ResourceNotFound,
            _ => FailureResponse::ServerError,
        })
}
