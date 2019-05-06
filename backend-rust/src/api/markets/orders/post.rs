use crate::{
    api::{validate_bearer_header, FailureResponse},
    lmsr,
    postgres::MarketStatus,
    Server,
};
use diesel::{pg::PgConnection as PgConn, result::Error as PgError};
use rouille::{input::json::json_input, Request, Response};
use std::collections::HashMap;

// スプリットの最大許容量
const MAX_SPLIT_PERCENT: f64 = 0.05;

pub fn post(server: &Server, req: &Request, market_id: i32) -> Result<Response, FailureResponse> {
    let req_data = json_input::<ReqData>(&req).map_err(|_| FailureResponse::InvalidPayload)?;
    if req_data.amount_token == AmountToken(0) || req_data.amount_coin == AmountCoin(0) {
        return Err(FailureResponse::InvalidPayload);
    }

    // 認証チェック
    let redis_conn = server.get_new_redis_conn()?;
    let user_id = validate_bearer_header(&redis_conn, req)?;

    let pg_conn = server.get_new_pg_conn()?;

    // marketがopenかチェック
    let market = query_market(&pg_conn, market_id)?;
    if market.status != MarketStatus::Open {
        return Err(FailureResponse::ResourceNotFound);
    }

    // orderのpriceが範囲内に入っているか、残高はあるかを確認
    // 確認途中で別のユーザーがorderの更新を行なった場合はretryする
    check_order(pg_conn, req_data, market, user_id)
}

fn check_order(
    pg_conn: PgConn,
    req_data: ReqData,
    market: Market,
    user_id: i32,
) -> Result<Response, FailureResponse> {
    // 現在の order リストを取得
    // 処理の間、この order リストの楽観ロックを取得している
    let orders = query_order_history(&pg_conn, market.id)?;

    // priceチェック
    let lmsr_b = lmsr::B(market.lmsr_b as u32);
    let price = check_price(&orders, lmsr_b, &req_data)?;

    // 残高チェック
    if req_data.amount_token < AmountToken(0) {
        // tokenを売却するケース
        check_balance_of_user_token(&orders, user_id, req_data.token_id, req_data.amount_token)?;
    } else {
        // tokenを購入するケース
        check_balance_of_user_coin(&orders, user_id, req_data.amount_coin)?;
    }

    // DBにsave
    let new_order = NewOrder {
        user_id,
        market_id: market.id,
        token_id: req_data.token_id.0,
        in_market_id: orders.len() as i32,
        amount_token: req_data.amount_token.0,
        amount_coin: req_data.amount_coin.0,
    };
    match save_order(&pg_conn, new_order) {
        SaveResult::Success => {
            let res_data = ResData {
                token_id: req_data.token_id,
                amount_token: req_data.amount_token,
                amount_coin: price,
            };
            Ok(Response::json(&res_data).with_status_code(201))
        },
        SaveResult::Retry => check_order(pg_conn, req_data, market, user_id),
        SaveResult::Failure(res) => Err(res),
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone)]
struct TokenId(i32);

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct AmountToken(i32);

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct AmountCoin(i32);

#[derive(Debug, Deserialize)]
struct ReqData {
    token_id: TokenId,
    amount_token: AmountToken,
    amount_coin: AmountCoin,
}

#[derive(Debug, Serialize)]
struct ResData {
    token_id: TokenId,
    amount_token: AmountToken,
    amount_coin: AmountCoin,
}

#[derive(Debug, Queryable)]
struct Market {
    id: i32,
    status: MarketStatus,
    lmsr_b: i32,
}

#[derive(Debug, Queryable)]
struct Order {
    user_id: i32,
    token_id: Option<i32>,
    amount_token: i32,
    amount_coin: i32,
}

use crate::postgres::schema::orders;
#[derive(Debug, Insertable)]
#[table_name = "orders"]
struct NewOrder {
    user_id: i32,
    market_id: i32,
    token_id: i32,
    in_market_id: i32,
    amount_token: i32,
    amount_coin: i32,
}

#[derive(Debug)]
enum SaveResult {
    Success,
    Retry,
    Failure(FailureResponse),
}

impl std::ops::Add for AmountToken {
    type Output = AmountToken;

    fn add(self, rhs: AmountToken) -> AmountToken {
        AmountToken(self.0 + rhs.0)
    }
}

impl std::ops::Add for AmountCoin {
    type Output = AmountCoin;

    fn add(self, rhs: AmountCoin) -> AmountCoin {
        AmountCoin(self.0 + rhs.0)
    }
}

fn query_market(conn: &PgConn, market_id: i32) -> Result<Market, FailureResponse> {
    use crate::postgres::schema::markets::{columns as market, table as markets};
    use diesel::prelude::*;

    markets
        .select((market::id, market::status, market::lmsr_b))
        .filter(market::id.eq(market_id))
        .first::<Market>(conn)
        .map_err(|e| match e {
            PgError::NotFound => FailureResponse::ResourceNotFound,
            _ => FailureResponse::ServerError,
        })
}

fn query_order_history(conn: &PgConn, market_id: i32) -> Result<Vec<Order>, FailureResponse> {
    use crate::postgres::schema::orders::{columns as order, table as orders};
    use diesel::prelude::*;

    orders
        .select((
            order::user_id,
            order::token_id,
            order::amount_token,
            order::amount_coin,
        ))
        .filter(order::market_id.eq(market_id))
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

fn check_price(
    orders: &Vec<Order>,
    lmsr_b: lmsr::B,
    req_data: &ReqData,
) -> Result<AmountCoin, FailureResponse> {
    let distributions = distributions(&orders);
    let cur_cost = lmsr::cost(lmsr_b, distributions.values().map(|amt| amt.0));
    let new_distri = distributions.iter().map(|(t_id, t_amt)| {
        if *t_id == req_data.token_id {
            req_data.amount_token.0 + t_amt.0
        } else {
            req_data.amount_token.0
        }
    });
    let new_cost = lmsr::cost(lmsr_b, new_distri);
    let price: i32 = new_cost - cur_cost;
    if price.signum() != req_data.amount_coin.0.signum() {
        return Err(FailureResponse::InvalidPayload);
    }
    let under_abs_price = price.abs() as f64 * (1f64 - MAX_SPLIT_PERCENT);
    let top_abs_price = price.abs() as f64 * (1f64 + MAX_SPLIT_PERCENT);
    let expect_abs_price = req_data.amount_coin.0.abs() as f64;
    if under_abs_price < expect_abs_price && expect_abs_price < top_abs_price {
        Ok(AmountCoin(price))
    } else {
        Err(FailureResponse::InvalidPayload)
    }
}

fn check_balance_of_user_token(
    orders: &Vec<Order>,
    user_id: i32,
    token_id: TokenId,
    expected_amount: AmountToken,
) -> Result<(), FailureResponse> {
    let balance: i32 = orders
        .iter()
        .filter(|order| order.user_id == user_id && order.token_id == Some(token_id.0))
        .map(|order| order.amount_token)
        .sum();
    if balance < expected_amount.0 {
        Err(FailureResponse::InvalidPayload)
    } else {
        Ok(())
    }
}

fn check_balance_of_user_coin(
    orders: &Vec<Order>,
    user_id: i32,
    expected_amount: AmountCoin,
) -> Result<(), FailureResponse> {
    let balance: i32 = orders
        .iter()
        .filter(|order| order.user_id == user_id)
        .map(|order| order.amount_coin)
        .sum();
    if balance < expected_amount.0 {
        Err(FailureResponse::InvalidPayload)
    } else {
        Ok(())
    }
}

fn distributions(orders: &Vec<Order>) -> HashMap<TokenId, AmountToken> {
    let mut distributions = HashMap::new();
    orders
        .iter()
        .filter_map(|o| {
            o.token_id
                .map(|t| (TokenId(t), AmountToken(o.amount_token)))
        })
        .for_each(|(token_id, amount_token)| {
            let cur = distributions
                .get(&token_id)
                .cloned()
                .unwrap_or(AmountToken(0));
            distributions.insert(token_id, cur + amount_token);
        });
    distributions
}

fn save_order(conn: &PgConn, new_order: NewOrder) -> SaveResult {
    use crate::postgres::schema::orders::{table as orders};
    use diesel::prelude::*;

    let res = diesel::insert_into(orders)
        .values(new_order)
        .execute(conn);
    match res {
        Ok(_) => SaveResult::Success,
        Err(PgError::DatabaseError(_, _)) => SaveResult::Retry,
        Err(_) => SaveResult::Failure(FailureResponse::ServerError),
    }
}
