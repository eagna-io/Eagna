use crate::{
    app::FailureResponse,
    domain::models::access_token::{AccessToken, AccessTokenId},
    domain::services::AccessTokenStore,
};
use rouille::Request;

pub fn validate_bearer_header<S>(
    store: &mut S,
    req: &Request,
) -> Result<AccessToken, FailureResponse>
where
    S: AccessTokenStore,
{
    let header_val = req
        .header("Authorization")
        .ok_or(FailureResponse::Unauthorized)?;
    let token_id = extract_token(header_val)?;
    match store.validate_access_token(&token_id)? {
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
