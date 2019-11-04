use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::user::repository::access_token::AccessTokenRepository;
use rouille::{Request, Response};

pub fn handler(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(infra, req)?;
    AccessTokenRepository::from(infra.get_redis()?).delete(&access_token)?;

    Ok(Response::empty_204())
}
