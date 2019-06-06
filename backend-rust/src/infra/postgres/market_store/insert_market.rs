use crate::domain::{models::market::MarketId, services::market_store::NewMarket};
use chrono::{DateTime, Utc};
use diesel::{pg::PgConnection, prelude::*, result::Error as PgError};
use crate::infra::postgres::schema::{market_tokens, markets};

pub fn insert_market(conn: &PgConnection, market: NewMarket) -> Result<MarketId, PgError> {
    let market_id = diesel::insert_into(markets::table)
        .values(InnerNewMarket {
            title: market.title.0.as_str(),
            organizer: market.organizer.0.as_str(),
            short_desc: market.short_desc.0.as_str(),
            description: market.description.0.as_str(),
            lmsr_b: market.lmsr_b.0 as i32,
            open_time: market.open_time,
            close_time: market.close_time,
        })
        .returning(markets::id)
        .get_result(conn)?;

    let tokens: Vec<InnerNewToken> = market
        .tokens
        .iter()
        .map(|token| InnerNewToken {
            name: token.name.0.as_str(),
            description: token.description.0.as_str(),
            market_id: market_id,
        })
        .collect();

    diesel::insert_into(market_tokens::table)
        .values(tokens)
        .execute(conn)
        .map(|_| MarketId(market_id))
}

#[derive(Insertable)]
#[table_name = "markets"]
struct InnerNewMarket<'a> {
    title: &'a str,
    organizer: &'a str,
    short_desc: &'a str,
    description: &'a str,
    lmsr_b: i32,
    open_time: DateTime<Utc>,
    close_time: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "market_tokens"]
struct InnerNewToken<'a> {
    name: &'a str,
    description: &'a str,
    market_id: i32,
}
