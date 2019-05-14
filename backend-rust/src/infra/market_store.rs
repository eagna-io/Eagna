use crate::domain::{
    infra::postgres::MarketStatus,
    models::{BaseMarket, Market, MarketId},
    services::{MarketStore, QueryMarketError},
};
use diesel::{pg::PgConnection, result::Error as PgError};

pub struct PgMarketStore {
    conn: PgConnection,
}

impl MarketStore for PgMarketStore {
    type Error = PgError;

    fn query_market(&self, market_id: MarketId) -> Result<Option<Market>, Self::Error> {
        let raw_market = match query_raw_market(&self.conn, market_id) {
            Ok(Some(raw_market)) => raw_market,
            res => return res,
        };
        let tokens = query_tokens(&self.conn, market_id)?;

        let base = BaseMarket {
            title: raw_market.title,
            organizer: raw_market.organizer,
            short_desc: raw_market.short_desc,
            description: raw_market.description,
            lmsr_b: lmsr::B(raw_market.lmsr_b),
            open_time: raw_market.open_time,
            close_time: raw_market.close_time,
            tokens: tokens,
        };

        let m = match raw_market.status {
            MarketStatus::Preparing => Market::Preparing(PreparingMarket { base }),
            MarketStatus::Open => Market::Open(OpenMarket { base }),
            MarketStatus::Closed => Market::Closed(ClosedMarket { base }),
            MarketStatus::Settled => {
                let token_id = raw_market.settle_token_id.unwrap();
                let token = base.tokens.iter().find(|t| t.id == token_id).unwrap();
                Market::Settled(SettledMarket {
                    base,
                    settle_token: token,
                })
            }
        };
    }
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

impl Into<Token> for QueryableToken {
    fn into(self) -> Token {
        Token {
            id: self.id,
            name: self.name,
            description: self.description,
        }
    }
}

fn query_raw_market(
    conn: &PgConnection,
    market_id: MarketId,
) -> Result<Option<QueryableMarket>, PgError> {
    use crate::infra::postgres::schema::market_tokens::{columns as token, table as tokens};
    use diesel::prelude::*;

    let res = markets
        .filter(market::id.eq(market_id.0))
        .first::<QueryableMarket>(conn);
    match res {
        Ok(raw_market) => Ok(Some(raw_market)),
        Err(PgError::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

fn query_tokens(conn: &PgConnection, market_id: MarketId) -> Result<Vec<Token>, PgError> {
    use crate::infra::postgres::schema::markets::{columns as market, table as markets};
    use diesel::prelude::*;

    tokens
        .select((token::id, token::name, token::description))
        .filter(token::market_id.eq(market_id.0))
        .load::<QueryableToken>(conn)?
        .into_iter()
        .map(QueryableToken::into)
        .collect()
}
