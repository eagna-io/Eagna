use crate::infra::jwt::Jwt;
use jsonwebtoken::errors::Error as JwtError;

pub struct AccessTokenManager();

impl AccessTokenManager {
    pub fn encode(token: &AccessToken) -> String {
        let claim = JwtClaim {
            user_id: *token.user_id.as_uuid(),
        };
        Jwt::encode(&claim).unwrap()
    }

    pub fn decode(raw: &str) -> Result<AccessToken, JwtError> {
        let claim = Jwt::decode::<JwtClaim>(raw)?;
        Ok(AccessToken::from(UserId::from(claim.user_id)))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaim {
    user_id: Uuid,
}
