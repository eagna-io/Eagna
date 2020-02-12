use crate::FailureResponse;
use crop_domain::user::access_token::{models::AccessToken, services::AccessTokenManager};
use regex::Regex;
use rouille::Request;

pub fn validate_bearer_header(req: &Request) -> Result<AccessToken, FailureResponse> {
    let header_val = req
        .header("Authorization")
        .ok_or(FailureResponse::Unauthorized)?;

    let raw_token = extract_token(header_val)?;

    match AccessTokenManager::decode(raw_token) {
        Ok(token) => Ok(token),
        Err(e) => {
            log::warn!("Failed to decode access_token");
            log::warn!("     {:?}", e);
            Err(FailureResponse::Unauthorized)
        }
    }
}

fn extract_token<'a>(header_val: &'a str) -> Result<&'a str, FailureResponse> {
    lazy_static::lazy_static! {
        static ref BEARER_TOKEN_REGEX: Regex = Regex::new(r"^Bearer (.+)$").unwrap();
    }
    BEARER_TOKEN_REGEX
        .captures(header_val)
        .and_then(|cap| cap.get(1))
        .ok_or(FailureResponse::Unauthorized)
        .map(|mat| mat.as_str())
}
