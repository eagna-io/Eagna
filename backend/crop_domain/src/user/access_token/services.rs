use super::models::AccessToken;
use crate::user::models::UserId;
use chrono::{DateTime, NaiveDateTime, Utc};
use crop_infra::jwt::Jwt;
use jsonwebtoken::errors::Error as JwtError;
use uuid::Uuid;

pub struct AccessTokenManager();

impl AccessTokenManager {
    pub fn encode(token: &AccessToken) -> String {
        let claim = JwtClaim {
            user_id: *token.user_id.as_uuid(),
            exp: token.expire_at.timestamp() as usize,
        };
        Jwt::encode(&claim).unwrap()
    }

    pub fn decode(raw: &str) -> Result<AccessToken, JwtError> {
        let claim = Jwt::decode::<JwtClaim>(raw)?;
        Ok(AccessToken {
            user_id: UserId::from(claim.user_id),
            expire_at: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(claim.exp as i64, 0),
                Utc,
            ),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaim {
    user_id: Uuid,
    exp: usize,
}
