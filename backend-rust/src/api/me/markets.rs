use crate::{
    api::{validate_bearer_header, FailureResponse},
    postgres::{MarketStatus, OrderType},
    Server,
};
use chrono::{DateTime, Utc};
use diesel::pg::PgConnection as PgConn;
use rouille::{Request, Response};

pub fn get(server: &Server, req: &Request) -> Result<Response, FailureResponse> {
    let redis_conn = server.get_new_redis_conn()?;
    let user_id = validate_bearer_header(&redis_conn, req)?;
    let pg_conn = server.get_new_pg_conn()?;
    let markets = query_markets(&pg_conn, user_id)?;

    Ok(Response::json(&markets))
}

#[derive(Debug, Serialize, Queryable)]
struct QueryMarket {
    id: i32,
    title: String,
    short_desc: String,
    status: MarketStatus,
    open_time: DateTime<Utc>,
    close_time: DateTime<Utc>,
}

fn query_markets(conn: &PgConn, user_id: i32) -> Result<Vec<QueryMarket>, FailureResponse> {
    use crate::postgres::schema::markets::{columns as market, table as markets};
    use crate::postgres::schema::orders::{columns as order, table as orders};
    use diesel::expression::dsl::any;
    use diesel::prelude::*;

    let joining_market_ids = orders
        .select(order::market_id)
        .filter(
            order::type_
                .eq(OrderType::InitialSupply)
                .and(order::user_id.eq(user_id)),
        )
        .load::<i32>(conn)
        .map_err(|_| FailureResponse::ServerError)?;

    markets
        .select((
            market::id,
            market::title,
            market::short_desc,
            market::status,
            market::open_time,
            market::close_time,
        ))
        .filter(
            market::status
                .eq(MarketStatus::Preparing)
                .or(market::id.eq(any(joining_market_ids))),
        )
        .load::<QueryMarket>(conn)
        .map_err(|_| FailureResponse::ServerError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::Connection;

    #[test]
    fn query_markets_should_contain_preparing_markets() {
        let pg_conn = crate::PgConnectionFactory::new_with_env()
            .establish()
            .unwrap();
        pg_conn.begin_test_transaction().unwrap();
        let user_id = utils::user::Alice.get_id(&pg_conn);
        let res = query_markets(&pg_conn, user_id);
        assert!(res.is_ok());

        let markets = res.unwrap();
        let market_id = utils::market::preparing_market().get_id(&pg_conn);
        assert!(markets.iter().find(|m| m.id == market_id).is_some());
    }
}
