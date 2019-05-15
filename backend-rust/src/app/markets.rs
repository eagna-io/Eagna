pub mod orders;

use crate::{
    app::FailureResponse,
    domain::models::market::{
        Market, MarketDesc, MarketId, MarketOrganizer, MarketShortDesc, MarketStatus, MarketTitle,
        MarketTokens, TokenId,
    },
    domain::services::MarketStore,
};
use chrono::{DateTime, Utc};
use rouille::{Request, Response};

pub fn get<S>(store: &S, _req: &Request, market_id: MarketId) -> Result<Response, FailureResponse>
where
    S: MarketStore,
{
    let market = match store.query_market(&market_id) {
        Ok(Some(market)) => market,
        Ok(None) => return Err(FailureResponse::ResourceNotFound),
        Err(e) => {
            dbg!(e);
            return Err(FailureResponse::ServerError);
        }
    };
    Ok(Response::json(&RespData::from(market)))
}

#[derive(Debug, Serialize, Queryable)]
struct RespData {
    title: MarketTitle,
    organizer: MarketOrganizer,
    short_desc: MarketShortDesc,
    description: MarketDesc,
    open_time: DateTime<Utc>,
    close_time: DateTime<Utc>,
    tokens: MarketTokens,
    status: MarketStatus,
    settle_token_id: Option<TokenId>,
}

impl From<Market> for RespData {
    fn from(market: Market) -> RespData {
        let settle_token_id = match &market {
            Market::Preparing(_) => None,
            Market::Open(_) => None,
            Market::Closed(_) => None,
            Market::Settled(m) => Some(m.settle_token.id),
        };
        RespData {
            title: market.title.clone(),
            organizer: market.organizer.clone(),
            short_desc: market.short_desc.clone(),
            description: market.description.clone(),
            open_time: market.open_time,
            close_time: market.close_time,
            tokens: market.tokens.clone(),
            status: market.status(),
            settle_token_id,
        }
    }
}
