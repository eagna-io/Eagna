use crate::{
    domain::models::{
        lmsr,
        market::*,
        num::{AmountCoin, AmountToken},
        user::UserId,
    },
    infra::postgres::types::{MarketStatus, OrderType},
};
use chrono::{DateTime, Utc};
use diesel::{pg::PgConnection, prelude::*, result::Error as PgError};
use std::sync::Arc;

pub fn query_market(conn: &PgConnection, market_id: &MarketId) -> Result<Option<Market>, PgError> {
    let raw_market = match query_raw_market(conn, market_id) {
        Ok(Some(raw_market)) => raw_market,
        Ok(None) => return Ok(None),
        Err(e) => return Err(e),
    };
    let tokens = query_tokens(conn, market_id)?;

    let base = BaseInfos {
        id: MarketId(raw_market.id),
        title: MarketTitle(Arc::new(raw_market.title)),
        organizer: MarketOrganizer(Arc::new(raw_market.organizer)),
        short_desc: MarketShortDesc(Arc::new(raw_market.short_desc)),
        description: MarketDesc(Arc::new(raw_market.description)),
        lmsr_b: lmsr::B(raw_market.lmsr_b as u32),
        open_time: raw_market.open_time,
        close_time: raw_market.close_time,
        tokens: tokens,
    };

    let m = match raw_market.status {
        MarketStatus::Preparing => Market::Preparing(PreparingMarket { base }),
        MarketStatus::Open => {
            let orders = query_orders(conn, market_id)?;
            Market::Open(OpenMarket { base, orders })
        }
        MarketStatus::Closed => {
            let orders = query_orders(conn, market_id)?;
            Market::Closed(ClosedMarket { base, orders })
        }
        MarketStatus::Settled => {
            let token_id = TokenId(raw_market.settle_token_id.unwrap());
            let token = base
                .tokens
                .iter()
                .find(|t| t.id == token_id)
                .unwrap()
                .clone();
            let orders = query_orders(conn, market_id)?;
            Market::Settled(SettledMarket {
                base,
                orders,
                settle_token: token,
            })
        }
    };

    Ok(Some(m))
}

#[derive(Debug, Queryable)]
struct QueryableMarket {
    id: i32,
    title: String,
    organizer: String,
    short_desc: String,
    description: String,
    lmsr_b: i32,
    open_time: DateTime<Utc>,
    close_time: DateTime<Utc>,
    status: MarketStatus,
    settle_token_id: Option<i32>,
}

#[derive(Debug, Queryable)]
struct QueryableToken {
    id: i32,
    name: String,
    description: String,
}

#[derive(Debug, Queryable)]
struct QueryableOrder {
    id: i32,
    market_id: i32,
    serial_num: i32,
    user_id: i32,
    token_id: Option<i32>,
    amount_token: i32,
    amount_coin: i32,
    type_: OrderType,
    time: DateTime<Utc>,
}

impl Into<Token> for QueryableToken {
    fn into(self) -> Token {
        Token {
            id: TokenId(self.id),
            name: TokenName(Arc::new(self.name)),
            description: TokenDesc(Arc::new(self.description)),
        }
    }
}

impl Into<Order> for QueryableOrder {
    fn into(self) -> Order {
        match self.type_ {
            OrderType::Normal => Order::Normal(NormalOrder {
                user_id: UserId(self.user_id),
                token_id: TokenId(self.token_id.unwrap()),
                amount_token: AmountToken(self.amount_token),
                amount_coin: AmountCoin(self.amount_coin),
                time: self.time,
            }),
            OrderType::InitialSupply => Order::InitialSupply(InitialSupplyOrder {
                user_id: UserId(self.user_id),
                amount_coin: AmountCoin(self.amount_coin),
                time: self.time,
            }),
            OrderType::Settle => Order::Settle(SettleOrder {
                user_id: UserId(self.user_id),
                token_id: TokenId(self.token_id.unwrap()),
                amount_token: AmountToken(self.amount_token),
                amount_coin: AmountCoin(self.amount_coin),
                time: self.time,
            }),
        }
    }
}

fn query_raw_market(
    conn: &PgConnection,
    market_id: &MarketId,
) -> Result<Option<QueryableMarket>, PgError> {
    use crate::infra::postgres::schema::markets::{columns as market, table as markets};

    let res = markets
        .filter(market::id.eq(market_id.0))
        .first::<QueryableMarket>(conn);
    match res {
        Ok(raw_market) => Ok(Some(raw_market)),
        Err(PgError::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

fn query_tokens(conn: &PgConnection, market_id: &MarketId) -> Result<MarketTokens, PgError> {
    use crate::infra::postgres::schema::market_tokens::{columns as token, table as tokens};

    let vec = tokens
        .select((token::id, token::name, token::description))
        .filter(token::market_id.eq(market_id.0))
        .load::<QueryableToken>(conn)?
        .into_iter()
        .map(QueryableToken::into)
        .collect();
    Ok(MarketTokens(Arc::new(vec)))
}

fn query_orders(conn: &PgConnection, market_id: &MarketId) -> Result<MarketOrders, PgError> {
    use crate::infra::postgres::schema::orders::{columns as order, table as orders};

    let vec = orders
        .filter(order::market_id.eq(market_id.0))
        .load::<QueryableOrder>(conn)?
        .into_iter()
        .map(QueryableOrder::into)
        .collect();
    Ok(MarketOrders { orders: vec })
}
