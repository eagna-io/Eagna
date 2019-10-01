use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::user::*;
use chrono::{DateTime, Utc};
use rouille::{Request, Response};
use serde::Serialize;
use uuid::Uuid;

pub fn get_me(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(infra, req)?;
    let repo = UserRepository::from(infra.get_postgres()?);
    match repo.query_user(&access_token.user_id)? {
        None => Err(FailureResponse::Unauthorized),
        Some(user) => Ok(Response::json(&construct_response_data(&user))),
    }
}

fn construct_response_data(user: &User) -> ResUser {
    let point_history = user
        .point_history()
        .iter()
        .map(|item| match item {
            PointHistoryItem::MarketReward(reward_item) => ResPointHistoryItem::MarketReward {
                point: *reward_item.point(),
                time: *reward_item.time(),
                market_id: *reward_item.market_id().as_uuid(),
            },
            PointHistoryItem::PrizeTrade(trade_item) => ResPointHistoryItem::PrizeTrade {
                point: *trade_item.point(),
                time: *trade_item.time(),
                prize_id: *trade_item.prize_id().as_uuid(),
                trade_status: match trade_item.status() {
                    PrizeTradeStatus::Requested => ResPrizeTradeStatus::Requested,
                    PrizeTradeStatus::Processed => ResPrizeTradeStatus::Processed,
                },
            },
        })
        .collect();
    ResUser {
        id: user.id().as_str(),
        name: user.name().as_str(),
        email: user.email().as_str(),
        is_admin: *user.is_admin(),
        point_history: point_history,
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResUser<'a> {
    id: &'a str,
    name: &'a str,
    email: &'a str,
    is_admin: bool,
    point_history: Vec<ResPointHistoryItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
enum ResPointHistoryItem {
    MarketReward {
        point: u32,
        time: DateTime<Utc>,
        market_id: Uuid,
    },
    PrizeTrade {
        point: u32,
        time: DateTime<Utc>,
        prize_id: Uuid,
        trade_status: ResPrizeTradeStatus,
    },
}

#[derive(Serialize)]
enum ResPrizeTradeStatus {
    Requested,
    Processed,
}
