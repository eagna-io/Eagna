pub mod orders;

use crate::{
    app::{validate_bearer_header, FailureResponse},
    domain::models::{lmsr, market::*},
    domain::services::{
        market_store::{NewMarket, NewToken},
        AccessTokenStore, MarketStore, UserStore,
    },
};
use chrono::{DateTime, Utc};
use rouille::{input::json::json_input, Request, Response};

pub fn get<S>(
    mut store: S,
    _req: &Request,
    market_id: MarketId,
) -> Result<Response, FailureResponse>
where
    S: MarketStore,
{
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
        #[serde(skip_serializing_if = "Option::is_none")]
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

    let market = match store.query_market(&market_id)? {
        Some(market) => market,
        None => return Err(FailureResponse::ResourceNotFound),
    };
    Ok(Response::json(&RespData::from(market)))
}

pub fn post<S>(mut store: S, req: &Request) -> Result<Response, FailureResponse>
where
    S: MarketStore + UserStore + AccessTokenStore,
{
    #[derive(Deserialize)]
    struct ReqData {
        title: MarketTitle,
        organizer: MarketOrganizer,
        short_desc: MarketShortDesc,
        description: MarketDesc,
        lmsr_b: lmsr::B,
        open_time: DateTime<Utc>,
        close_time: DateTime<Utc>,
        tokens: Vec<ReqTokenData>,
    }

    #[derive(Deserialize)]
    struct ReqTokenData {
        name: TokenName,
        description: TokenDesc,
    }

    impl Into<NewMarket> for ReqData {
        fn into(self) -> NewMarket {
            let tokens: Vec<NewToken> = self
                .tokens
                .into_iter()
                .map(|token| NewToken {
                    name: token.name,
                    description: token.description,
                })
                .collect();
            NewMarket {
                title: self.title,
                organizer: self.organizer,
                short_desc: self.short_desc,
                description: self.description,
                lmsr_b: self.lmsr_b,
                open_time: self.open_time,
                close_time: self.close_time,
                tokens,
            }
        }
    }

    let access_token = validate_bearer_header(&mut store, req)?;
    let user = match store.query_user(&access_token.user_id)? {
        Some(user) => user,
        None => {
            println!("User does not exists, but AccessToken exists");
            return Err(FailureResponse::ServerError);
        }
    };
    if !user.is_admin {
        return Err(FailureResponse::Unauthorized);
    }

    let req_data = json_input::<ReqData>(req).map_err(|e| {
        dbg!(e);
        FailureResponse::InvalidPayload
    })?;
    let market_id = store.insert_market(req_data.into())?;
    store.commit()?;

    Ok(Response::json(&market_id).with_status_code(201))
}

pub fn put<S>(mut store: S, req: &Request, market_id: MarketId) -> Result<Response, FailureResponse>
where
    S: AccessTokenStore + UserStore + MarketStore,
{
    #[derive(Deserialize)]
    struct ReqData {
        status: MarketStatus,
        settle_token_id: TokenId,
    }

    let access_token = validate_bearer_header(&mut store, req)?;
    match store.query_user(&access_token.user_id)? {
        Some(ref user) if user.is_admin => {}
        Some(_) => return Err(FailureResponse::Unauthorized),
        None => {
            println!("User does not exists, but AccessToken exists");
            return Err(FailureResponse::ServerError);
        }
    };

    let req_data = json_input::<ReqData>(req).map_err(|e| {
        dbg!(e);
        FailureResponse::InvalidPayload
    })?;
    if req_data.status != MarketStatus::Settled {
        return Err(FailureResponse::InvalidPayload);
    }

    {
        let mut locked_store = store.lock_market(&market_id)?;
        let closed_market = match locked_store.query_market(&market_id)? {
            Some(Market::Closed(m)) => m,
            Some(_) => return Err(FailureResponse::ResourceNotFound),
            None => return Err(FailureResponse::ResourceNotFound),
        };
        let settled_market = closed_market.settle(req_data.settle_token_id);
        locked_store.update_market_status_to_settle(&settled_market)?;
    }
    store.commit()?;

    Ok(Response::json(&market_id).with_status_code(201))
}
