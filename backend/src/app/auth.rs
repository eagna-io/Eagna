use crate::{
    app::{FailureResponse, InfraManager},
    domain::access_token::{AccessToken, AccessTokenId, AccessTokenRepository},
};
use rouille::Request;

pub fn validate_bearer_header(
    infra: &InfraManager,
    req: &Request,
) -> Result<AccessToken, FailureResponse> {
    let header_val = req
        .header("Authorization")
        .ok_or(FailureResponse::Unauthorized)?;
    let token_id = extract_token(header_val)?;

    let access_token_repository =
        AccessTokenRepository::from((infra.get_firebase()?, infra.get_redis()?));

    match access_token_repository.query_access_token(&token_id)? {
        Some(token) => Ok(token),
        None => Err(FailureResponse::Unauthorized),
    }
}

fn extract_token(header_val: &str) -> Result<AccessTokenId, FailureResponse> {
    let re = regex::Regex::new(r"^Bearer: (.+)$").unwrap();
    re.captures(header_val)
        .and_then(|cap| cap.get(1))
        .ok_or(FailureResponse::Unauthorized)
        .map(|mat| AccessTokenId::from_str(mat.as_str()))
}
