use crate::{
    app::{validate_bearer_header, FailureResponse},
    domain::{
        models::market::{
            Market, MarketDesc, MarketId, MarketOrganizer, MarketShortDesc, MarketStatus,
            MarketTitle, MarketTokens,
        },
        services::{AccessTokenStore, MarketStore},
    },
};
use chrono::{DateTime, Utc};
use rouille::{Request, Response};

pub fn get<S>(mut store: S, req: &Request) -> Result<Response, FailureResponse>
where
    S: AccessTokenStore + MarketStore,
{
    let access_token = validate_bearer_header(&mut store, req)?;
    let markets = store
        .query_markets_related_to_user(&access_token.user_id)?;

    let resp_data: Vec<RespItem> = markets.into_iter().map(|m| RespItem::from(m)).collect();

    Ok(Response::json(&resp_data))
}

#[derive(Debug, Serialize, Queryable)]
struct RespItem {
    id: MarketId,
    title: MarketTitle,
    organizer: MarketOrganizer,
    short_desc: MarketShortDesc,
    description: MarketDesc,
    open_time: DateTime<Utc>,
    close_time: DateTime<Utc>,
    tokens: MarketTokens,
    status: MarketStatus,
}

impl From<Market> for RespItem {
    fn from(market: Market) -> RespItem {
        RespItem {
            id: market.id,
            title: market.title.clone(),
            organizer: market.organizer.clone(),
            short_desc: market.short_desc.clone(),
            description: market.description.clone(),
            open_time: market.open_time,
            close_time: market.close_time,
            tokens: market.tokens.clone(),
            status: market.status(),
        }
    }
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
