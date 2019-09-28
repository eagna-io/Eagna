use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::user::*;
use rouille::{Request, Response};
use serde::{Deserialize, Serialize};

pub fn me(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(infra, req)?;
    let repo = UserRepository::from(infra.get_postgres()?);
    match repo.query_user(&access_token.user_id)? {
        None => Err(FailureResponse::Unauthorized),
        Some(user) => Ok(Response::json(&ResUser::from(user))),
    }
}

fn construct_response_data(user: &User) -> ResUser {
    let point_history = user
        .point_history()
        .slice()
        .iter()
        .map(|item| match item {
            HistoryItem::MarketReward(reward_item) => ResPointHistoryItem {
                amount: reward_item.amount(),
                time: reward_item.time(),
                _type: PointHistoryItemType::MarketReward,
                market_id: Some(reward_item.market_id().as_uuid()),
                prize_id: None,
                prize_trade_status: None,
            },
            HistoryItem::PrizeTrade(trade_item) => ResPointHistoryItem {
                amount: -trade_item.price(),
                time: trade_item.time(),
                _type: PointHistoryItemType::PrizeTrade,
                market_id: None,
                prize_id: Some(trade_item.prize_id().as_uuid()),
                prize_trade_status: Some(PrizeTradeStatus::from(trade_item.status())),
            },
        })
        .collect();
    ResUser {
        id: user.id.as_str(),
        name: user.name.as_str(),
        email: user.email.as_str(),
        is_admin: user.is_admin,
        point_history: hoge,
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
struct ResPointHistoryItem {
    amount: i32,
    time: DateTime<Utc>,
    #[serde(rename = "type")]
    _type: PointHistoryItemType,
    market_id: Option<Uuid>,
    prize_id: Option<Uuid>,
    prize_trade_status: Option<PrizeTradeStatus>,
}

#[derive(Serialize)]
enum PointHistoryItemType {
    MarketReward,
    PointTrade,
}

#[derive(Serialize)]
enum PrizeTradeStatus {
    Requested,
    Processed,
}
