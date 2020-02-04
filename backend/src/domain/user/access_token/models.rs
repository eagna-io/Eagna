use crate::domain::user::models::UserId;
use chrono::Utc;

/// トークンは30日有効
pub const ACCESS_TOKEN_EXPIRE_SEC: usize = 60 * 60 * 24 * 30;

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub struct AccessToken {
    pub user_id: UserId,
    pub exp: usize, // JWTの仕様のためusize
}

impl AccessToken {
    pub fn new(user_id: &UserId) -> AccessToken {
        AccessToken {
            user_id: *user_id,
            exp: Utc::now().timestamp() as usize + ACCESS_TOKEN_EXPIRE_SEC,
        }
    }
}
