use crate::{
    api::{validate_bearer_header, FailureResponse},
    postgres::OrderType,
    Server,
};
use chrono::{DateTime, Utc};
use diesel::pg::PgConnection as PgConn;
use rouille::{Request, Response};

pub fn get_all(
    server: &Server,
    req: &Request,
    market_id: i32,
) -> Result<Response, FailureResponse> {
    let maybe_user_id = match req.get_param("contains") {
        Some(ref s) if s.as_str() == "me" => {
            let redis_conn = server.get_new_redis_conn()?;
            Some(validate_bearer_header(&redis_conn, req)?)
        }
        _ => None,
    };
    let pg_conn = server.get_new_pg_conn()?;
    let orders = query_orders(&pg_conn, market_id)?;

    let maybe_me = maybe_user_id.map(|user_id| {
        let order_ids = orders
            .iter()
            .filter(|order| order.user_id == user_id)
            .map(|order| order.id)
            .collect();
        Me { order_ids }
    });

    let resp = RespBody {
        orders,
        me: maybe_me,
    };
    Ok(Response::json(&resp))
}

#[derive(Debug, Serialize)]
struct RespBody {
    orders: Vec<Order>,
    me: Option<Me>,
}

#[derive(Debug, Serialize)]
struct Me {
    order_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Queryable)]
struct Order {
    id: i32,
    #[serde(skip)]
    user_id: i32,
    #[serde(serialize_with = "unwrap_then_ser")]
    token_id: Option<i32>,
    amount_token: i32,
    amount_coin: i32,
    time: DateTime<Utc>,
}

fn unwrap_then_ser<S>(token_id: &Option<i32>, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    S::serialize_i32(ser, token_id.unwrap())
}

fn query_orders(conn: &PgConn, market_id: i32) -> Result<Vec<Order>, FailureResponse> {
    use crate::postgres::schema::orders::{columns as order, table as orders};
    use diesel::prelude::*;

    orders
        .select((
            order::id,
            order::user_id,
            order::token_id,
            order::amount_token,
            order::amount_coin,
            order::time,
        ))
        .filter(
            order::market_id
                .eq(market_id)
                .and(order::type_.eq(OrderType::Normal)),
        )
        .load::<Order>(conn)
        .map_err(|_| FailureResponse::ServerError)
        .and_then(|results| {
            if results.is_empty() {
                Err(FailureResponse::ResourceNotFound)
            } else {
                Ok(results)
            }
        })
}
