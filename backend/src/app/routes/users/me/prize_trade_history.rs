use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::{
    prize::{PrizeId, PrizeRepository},
    user::*,
};
use chrono::{DateTime, Utc};
use rouille::{input::json::json_input, Request, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn post(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(infra, req)?;
    let req_data = json_input::<ReqData>(req).map_err(|_| FailureResponse::InvalidPayload)?;

    let user_repo = UserRepository::from((infra.get_postgres()?, infra.get_redis()?));
    let user = match user_repo.query_user(&access_token.user_id)? {
        None => {
            return Err(FailureResponse::Unauthorized);
        }
        Some(user) => user.with_point()?,
    };

    let prize_repo = PrizeRepository::from(infra.get_postgres()?);
    let prize = match prize_repo.query_prize(&PrizeId::from(req_data.prize_id))? {
        None => {
            log::warn!("User request non-exist prize trade");
            return Err(FailureResponse::InvalidPayload);
        }
        Some(prize) => prize,
    };

    let user = match user.request_prize_trade(&prize) {
        Err(_) => {
            log::warn!("User request exceeded price prize trade");
            return Err(FailureResponse::InvalidPayload);
        }
        Ok(user) => user,
    };
    user_repo.save_user_prize_trade_request(&user)?;

    let created_record = user.requested_prize_trade_record();

    Ok(Response::json(&ResUserPrizeTradeRecord::from(created_record)).with_status_code(201))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReqData {
    prize_id: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResUserPrizeTradeRecord {
    id: Uuid,
    prize_id: Uuid,
    point: u32,
    time: DateTime<Utc>,
    status: ResUserPrizeTradeStatus,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ResUserPrizeTradeStatus {
    Requested,
    Processed,
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
