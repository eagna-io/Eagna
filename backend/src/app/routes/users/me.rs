use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::user::*;
use chrono::{DateTime, Utc};
use rouille::{Request, Response};
use serde::Serialize;
use uuid::Uuid;

pub fn get_me(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(infra, req)?;
    let repo = UserRepository::from(infra.get_postgres()?);
    let user = match repo.query_user(&access_token.user_id)? {
        None => return Err(FailureResponse::Unauthorized),
        Some(user) => user
            .with_prize_trade_history()?
            .with_market_reward_history()?
            .compute_point(),
    };
    Ok(Response::json(&ResUser::from(&user)))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResUser<'a> {
    id: &'a str,
    name: &'a str,
    email: &'a str,
    is_admin: bool,
    point: u32,
    prize_trade_history: Vec<ResUserPrizeTradeRecord>,
    market_reward_history: Vec<ResUserMarketRewardRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResUserPrizeTradeRecord {
    id: Uuid,
    prize_id: Uuid,
    point: u32,
    time: DateTime<Utc>,
    status: ResUserPrizeTradeStatus,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum ResUserPrizeTradeStatus {
    Requested,
    Processed,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResUserMarketRewardRecord {
    market_id: Uuid,
    point: u32,
}

impl<'a, U> From<&'a U> for ResUser<'a>
where
    U: UserWithPoint + UserWithPrizeTradeHistory + UserWithMarketRewardHistory,
{
    fn from(user: &'a U) -> ResUser<'a> {
        ResUser {
            id: user.id().as_str(),
            name: user.name().as_str(),
            email: user.email().as_str(),
            is_admin: user.is_admin(),
            point: user.point().as_u32(),
            prize_trade_history: user
                .prize_trade_history()
                .iter()
                .map(ResUserPrizeTradeRecord::from)
                .collect(),
            market_reward_history: user
                .market_reward_history()
                .iter()
                .map(ResUserMarketRewardRecord::from)
                .collect(),
        }
    }
}

impl<'a> From<&'a PrizeTradeRecord> for ResUserPrizeTradeRecord {
    fn from(record: &'a PrizeTradeRecord) -> ResUserPrizeTradeRecord {
        ResUserPrizeTradeRecord {
            id: *record.id(),
            prize_id: *record.prize_id().as_uuid(),
            point: record.point().as_u32(),
            time: *record.time(),
            status: ResUserPrizeTradeStatus::from(*record.status()),
        }
    }
}

impl From<PrizeTradeStatus> for ResUserPrizeTradeStatus {
    fn from(status: PrizeTradeStatus) -> ResUserPrizeTradeStatus {
        match status {
            PrizeTradeStatus::Requested => ResUserPrizeTradeStatus::Requested,
            PrizeTradeStatus::Processed => ResUserPrizeTradeStatus::Processed,
        }
    }
}

impl<'a> From<&'a MarketRewardRecord> for ResUserMarketRewardRecord {
    fn from(record: &'a MarketRewardRecord) -> ResUserMarketRewardRecord {
        ResUserMarketRewardRecord {
            market_id: *record.market_id().as_uuid(),
            point: record.point().as_u32(),
        }
    }
}
