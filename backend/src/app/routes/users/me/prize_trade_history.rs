use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::user::UserRepository;
use rouillle::{Request, Response};
use serde::Deserialize;
use uuid::Uuid;

pub fn post(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(infra, req)?;
    let repo = UserRepository::from(infra.get_postgres()?);
    let user = match repo.query_user(&access_token.user_id)? {
        None => {
            return Err(FailureResponse::Unauthorized);
        }
        Some(user) => user.with_prize_trade_history()?,
    };
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqPrizeTradeRecord {
    prize_id: Uuid,
}
