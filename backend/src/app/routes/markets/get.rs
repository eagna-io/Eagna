use super::ApiMarketStatus;
use crate::app::{get_param, get_params, validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::market::{
    models::{Market, MarketId, MarketStatus, MarketToken},
    repository::{MarketRepository, QueryMarket},
};
use arrayvec::ArrayVec;
use chrono::{DateTime, Utc};
use rouille::{Request, Response};
use std::collections::HashMap;
use uuid::Uuid;

pub fn get(
    infra: &InfraManager,
    _req: &Request,
    market_id: Uuid,
) -> Result<Response, FailureResponse> {
    let postgres = infra.get_postgres()?;
    let market_repo = MarketRepository::from(postgres);

    let market = match market_repo.query_market(&MarketId::from(market_id))? {
        Some(market) => market,
        None => return Err(FailureResponse::ResourceNotFound),
    };

    Ok(Response::json(&ResMarket::from(&market)))
}

pub fn get_list(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let repo = MarketRepository::from(infra.get_postgres()?);
    let market_ids = if get_param(req, "participated").is_some() {
        query_market_ids_by_user_id(infra, req)?
    } else {
        query_market_ids_by_status(infra, req)?
    };
    let markets = market_ids
        .iter()
        .map(|id| Ok(repo.query_market(id)?.unwrap()))
        .collect::<Result<Vec<_>, FailureResponse>>()?;
    let res_data = markets.iter().map(ResMarket::from).collect::<Vec<_>>();

    Ok(Response::json(&res_data))
}

fn query_market_ids_by_user_id(
    infra: &InfraManager,
    req: &Request,
) -> Result<Vec<MarketId>, FailureResponse> {
    let access_token = validate_bearer_header(infra, req)?;
    Ok(MarketRepository::from(infra.get_postgres()?)
        .query_market_ids_user_participated(&access_token.user_id)?)
}

fn query_market_ids_by_status(
    infra: &InfraManager,
    req: &Request,
) -> Result<Vec<MarketId>, FailureResponse> {
    let mut statuses = ArrayVec::<[MarketStatus; 4]>::new();
    get_params(req, "status").for_each(|s| match s {
        "upcoming" => {
            let _ = statuses.try_push(MarketStatus::Upcoming);
        }
        "open" => {
            let _ = statuses.try_push(MarketStatus::Open);
        }
        "closed" => {
            let _ = statuses.try_push(MarketStatus::Closed);
        }
        "resolved" => {
            let _ = statuses.try_push(MarketStatus::Resolved);
        }
        _ => {
            log::info!("Received invalid status query : [{}]", s);
        }
    });
    if statuses.len() == 0 {
        statuses.push(MarketStatus::Upcoming);
        statuses.push(MarketStatus::Open);
        statuses.push(MarketStatus::Closed);
        statuses.push(MarketStatus::Resolved);
    }

    Ok(MarketRepository::from(infra.get_postgres()?)
        .query_market_ids_with_status(statuses.as_slice())?)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResMarket<'a> {
    id: Uuid,
    title: &'a str,
    description: &'a str,
    lmsr_b: u32,
    open: &'a DateTime<Utc>,
    close: &'a DateTime<Utc>,
    tokens: Vec<ResMarketToken<'a>>,
    status: ApiMarketStatus,
    token_distribution: HashMap<&'a str, u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolved_token_name: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResMarketToken<'a> {
    name: &'a str,
    description: &'a str,
    thumbnail_url: &'a str,
}

impl<'a> From<&'a QueryMarket> for ResMarket<'a> {
    fn from(market: &'a QueryMarket) -> ResMarket<'a> {
        ResMarket {
            id: *market.id().as_uuid(),
            title: market.attrs().title().as_str(),
            description: market.attrs().description().as_str(),
            lmsr_b: market.attrs().lmsr_b().as_u32(),
            open: market.attrs().open(),
            close: market.attrs().close(),
            tokens: market
                .attrs()
                .tokens()
                .iter()
                .map(ResMarketToken::from)
                .collect(),
            status: ApiMarketStatus::from(&market.status()),
            token_distribution: market
                .compute_token_distribution()
                .iter()
                .map(|(name, amount)| (name.as_str(), amount.as_i32() as u32))
                .collect(),
            resolved_token_name: market.resolved_token_name().as_ref().map(|s| s.as_str()),
        }
    }
}

impl<'a> From<&'a MarketToken> for ResMarketToken<'a> {
    fn from(token: &'a MarketToken) -> ResMarketToken<'a> {
        ResMarketToken {
            name: token.name().as_str(),
            description: token.description().as_str(),
            thumbnail_url: token.thumbnail_url().as_str(),
        }
    }
}
