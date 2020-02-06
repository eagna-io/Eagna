use crate::user::models::UserId;
use chrono::{DateTime, Duration, Utc};

/// トークンは30日有効
pub const ACCESS_TOKEN_EXPIRE_DAYS: i64 = 30;

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub struct AccessToken {
    pub user_id: UserId,
    pub expire_at: DateTime<Utc>,
}

impl AccessToken {
    pub fn new(user_id: &UserId) -> AccessToken {
        AccessToken {
            user_id: *user_id,
            expire_at: Utc::now() + Duration::days(ACCESS_TOKEN_EXPIRE_DAYS),
        }
    }
}
