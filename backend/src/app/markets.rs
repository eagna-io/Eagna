pub mod orders;

use crate::{
    app::{get_params, validate_bearer_header, FailureResponse},
    domain::models::{lmsr, market::*},
    domain::services::{
        market_store::{NewMarket, NewToken},
        AccessTokenStore, MarketStore, UserStore,
    },
};
use chrono::{DateTime, Utc};
use log::info;
use rouille::{input::json::json_input, Request, Response};

pub fn get<S>(
    store: &mut S,
    _req: &Request,
    market_id: MarketId,
) -> Result<Response, FailureResponse>
where
    S: MarketStore,
{
    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct RespData {
        id: MarketId,
        title: MarketTitle,
        organizer: MarketOrganizer,
        short_desc: MarketShortDesc,
        description: MarketDesc,
        open_time: DateTime<Utc>,
        close_time: DateTime<Utc>,
        lmsr_b: lmsr::B,
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
                id: market.id,
                title: market.title.clone(),
                organizer: market.organizer.clone(),
                short_desc: market.short_desc.clone(),
                description: market.description.clone(),
                open_time: market.open_time,
                close_time: market.close_time,
                lmsr_b: market.lmsr_b,
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

pub fn get_all<S>(store: &mut S, req: &Request) -> Result<Response, FailureResponse>
where
    S: MarketStore,
{
    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct RespItem {
        id: MarketId,
        title: MarketTitle,
        organizer: MarketOrganizer,
        short_desc: MarketShortDesc,
        description: MarketDesc,
        lmsr_b: lmsr::B,
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
                lmsr_b: market.lmsr_b,
                open_time: market.open_time,
                close_time: market.close_time,
                tokens: market.tokens.clone(),
                status: market.status(),
            }
        }
    }

    let status_iter = get_params(req, "status").filter_map(|s| match s {
        "upcoming" => Some(MarketStatus::Preparing),
        "open" => Some(MarketStatus::Open),
        "closed" => Some(MarketStatus::Closed),
        "resolved" => Some(MarketStatus::Settled),
        _ => {
            log::info!("Received invalid status query : [{}]", s);
            None
        }
    });
    let market_ids = store.query_market_ids_with_status(status_iter)?;

    let mut resp_data = Vec::with_capacity(market_ids.len());
    for market_id in market_ids {
        let market = store.query_market(&market_id)?.unwrap();
        resp_data.push(RespItem::from(market));
    }

    Ok(Response::json(&resp_data))
}

pub fn post<S>(store: &mut S, req: &Request) -> Result<Response, FailureResponse>
where
    S: MarketStore + UserStore + AccessTokenStore,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
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

    let access_token = validate_bearer_header(store, req)?;
    let user = match store.query_user(&access_token.user_id)? {
        Some(user) => user,
        None => {
            log::warn!("User does not exists, but AccessToken exists");
            return Err(FailureResponse::ServerError);
        }
    };
    if !user.is_admin {
        return Err(FailureResponse::Unauthorized);
    }

    let req_data = json_input::<ReqData>(req).map_err(|e| {
        info!("{:?}", e);
        FailureResponse::InvalidPayload
    })?;
    let market_id = store.insert_market(req_data.into())?;

    Ok(Response::json(&market_id).with_status_code(201))
}

pub fn put<S>(
    store: &mut S,
    req: &Request,
    market_id: MarketId,
) -> Result<Response, FailureResponse>
where
    S: AccessTokenStore + UserStore + MarketStore,
{
    #[derive(Deserialize)]
    struct ReqData {
        status: MarketStatus,
        settle_token_id: TokenId,
    }

    let access_token = validate_bearer_header(store, req)?;
    match store.query_user(&access_token.user_id)? {
        Some(ref user) if user.is_admin => {}
        Some(_) => return Err(FailureResponse::Unauthorized),
        None => {
            log::warn!("User does not exists, but AccessToken exists");
            return Err(FailureResponse::ServerError);
        }
    };

    let req_data = json_input::<ReqData>(req).map_err(|e| {
        info!("Invalid payload : {:?}", e);
        FailureResponse::InvalidPayload
    })?;
    if req_data.status != MarketStatus::Settled {
        return Err(FailureResponse::InvalidPayload);
    }

    let mut locked_store = store.lock_market(&market_id)?;
    let closed_market = match locked_store.query_market(&market_id)? {
        Some(Market::Closed(m)) => m,
        Some(_) => return Err(FailureResponse::ResourceNotFound),
        None => return Err(FailureResponse::ResourceNotFound),
    };
    let settled_market = closed_market
        .settle(req_data.settle_token_id)
        .map_err(|_e| {
            log::info!(
                "Try to resolve market {:?} with invalid token {:?}",
                market_id,
                req_data.settle_token_id
            );
            FailureResponse::InvalidPayload
        })?;
    locked_store.update_market_status_to_settle(&settled_market)?;

    Ok(Response::json(&market_id).with_status_code(201))
}
