use crate::app::errors::Error;
use crate::app::{FailureResponse, InfraManager};
use crate::domain::user::{
    models::access_token::{AccessToken, AccessTokenId},
    repository::access_token::AccessTokenRepository,
};
use crate::infra::redis::{Redis, RedisInfra};
use http::StatusCode;
use regex::Regex;
use rouille::Request;
use warp::{reject::Rejection, Filter as _};

/// Authorizationヘッダーを読み取り、UserIdなどをExtractする
/// 失敗した場合は409を返す
///
/// ## Note
/// このFilterは内部で `warp::filters::ext::get::<Redis>` を呼び出している。
/// そのため、呼び出し元で `warp::filters::ext::set(redis)` していないとパニックする。
pub fn auth_filter() -> impl warp::Filter<Extract = (AccessToken,), Error = Rejection> {
    warp::filters::header::header::<BearerToken>("Authorization")
        .and(warp::filters::ext::get::<Redis>())
        .and_then(|BearerToken(token), redis| {
            query_user(token, redis).map_err(warp::reject::custom)
        })
}

struct BearerToken(AccessTokenId);

impl std::str::FromStr for BearerToken {
    type Err = (); // warpはこのエラーを捨てる

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref BEARER_TOKEN_REGEX: Regex = Regex::new(r"^Bearer: (.+)$").unwrap();
        }
        let access_token_id = BEARER_TOKEN_REGEX
            .captures(s)
            .and_then(|cap| cap.get(1))
            .ok_or(())
            .and_then(|mat| AccessTokenId::try_from_str(mat.as_str()).map_err(drop))?;
        Ok(BearerToken(access_token_id))
    }
}

// ## TODO
// async にする
fn query_user(token: AccessTokenId, redis: Redis) -> Result<AccessToken, Error> {
    AccessTokenRepository::from(&redis as &dyn RedisInfra)
        .query(&token)
        .map_err(Error::internal_error)?
        .ok_or(Error::from((
            StatusCode::UNAUTHORIZED,
            "Unauthorized".into(),
        )))
}

/*
 * ==========================
 * Old codes
 * ==========================
 */
pub fn validate_bearer_header(
    infra: &InfraManager,
    req: &Request,
) -> Result<AccessToken, FailureResponse> {
    let header_val = req
        .header("Authorization")
        .ok_or(FailureResponse::Unauthorized)?;

    let token_id = extract_token(header_val)?;

    let repo = AccessTokenRepository::from(infra.get_redis()?);

    match repo.query(&token_id)? {
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
