use crate::{
    app::{FailureResponse, InfraManager},
    domain::user::{AccessToken, AccessTokenId, UserRepository},
};
use regex::Regex;
use rouille::Request;

pub fn validate_bearer_header(
    infra: &InfraManager,
    req: &Request,
) -> Result<AccessToken, FailureResponse> {
    let header_val = req
        .header("Authorization")
        .ok_or(FailureResponse::Unauthorized)?;
    let token_id = extract_token(header_val)?;

    let user_repo = UserRepository::from((infra.get_postgres()?, infra.get_redis()?));

    match user_repo.query_access_token(&token_id)? {
        Some(token) => Ok(token),
        None => Err(FailureResponse::Unauthorized),
    }
}

fn extract_token(header_val: &str) -> Result<AccessTokenId, FailureResponse> {
    lazy_static::lazy_static! {
        static ref BEARER_TOKEN_REGEX: Regex = Regex::new(r"^Bearer: (.+)$").unwrap();
    }
    BEARER_TOKEN_REGEX
        .captures(header_val)
        .and_then(|cap| cap.get(1))
        .ok_or(FailureResponse::Unauthorized)
        .and_then(|mat| AccessTokenId::try_from_str(mat.as_str()).map_err(FailureResponse::from))
}
