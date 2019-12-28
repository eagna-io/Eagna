use super::ApiMarketStatus;
use crate::app::{get_param, get_params, validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::market::*;

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
    let market_ids = query_market_ids(infra, req)?;
    let markets =
        MarketRepository::from(infra.get_postgres()?).query_markets(market_ids.as_slice())?;
    let resp_data: Vec<_> = markets.iter().map(ResMarket::from).collect();

    Ok(Response::json(&resp_data))
}

fn query_market_ids(infra: &InfraManager, req: &Request) -> Result<Vec<MarketId>, FailureResponse> {
    if let Some("true") = get_param(req, "participated") {
        // ユーザーが参加している/参加したマーケット一覧を取得
        let access_token = validate_bearer_header(infra, req)?;
        Ok(MarketRepository::from(infra.get_postgres()?)
            .query_market_ids_participated_by_user(&access_token.user_id)?)
    } else {
        // 指定されたstatusのマーケット一覧を取得
        query_market_ids_by_status(infra, req)
    }
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
    id: &'a Uuid,
    title: &'a str,
    organizer_id: &'a Uuid,
    description: &'a str,
    lmsr_b: u32,
    total_reward_point: u32,
    open: &'a DateTime<Utc>,
    close: &'a DateTime<Utc>,
    tokens: Vec<ResMarketToken<'a>>,
    prizes: Vec<ResMarketPrize<'a>>,
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResMarketPrize<'a> {
    id: i32,
    name: &'a str,
    thumbnail_url: &'a str,
    target: &'a str,
}

impl<'a> From<&'a Market> for ResMarket<'a> {
    fn from(market: &'a Market) -> ResMarket<'a> {
        ResMarket {
            id: market.id().as_uuid(),
            title: market.attrs().title().as_str(),
            organizer_id: market.attrs().organizer_id().as_uuid(),
            description: market.attrs().description().as_str(),
            lmsr_b: market.attrs().lmsr_b().as_u32(),
            total_reward_point: market.attrs().total_reward_point().as_u32(),
            open: market.attrs().open(),
            close: market.attrs().close(),
            tokens: market
                .attrs()
                .tokens()
                .iter()
                .map(ResMarketToken::from)
                .collect(),
            prizes: market
                .attrs()
                .prizes()
                .iter()
                .map(ResMarketPrize::from)
                .collect(),
            status: ApiMarketStatus::from(&market.status()),
            token_distribution: market
                .token_distribution()
                .iter()
                .map(|(name, amount)| (name.as_str(), amount.as_i32() as u32))
                .collect(),
            resolved_token_name: match market {
                Market::Resolved(ref inner) => Some(inner.resolved_token_name()),
                _ => None,
            },
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

impl<'a> From<&'a MarketPrize> for ResMarketPrize<'a> {
    fn from(prize: &'a MarketPrize) -> ResMarketPrize<'a> {
        ResMarketPrize {
            id: *prize.id(),
            name: prize.name().as_str(),
            thumbnail_url: prize.thumbnail_url().as_str(),
            target: prize.target().as_str(),
        }
    }
}
