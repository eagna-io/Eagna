use crate::domain::models::user::UserId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessToken {
    pub id: AccessTokenId,
    pub user_id: UserId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AccessTokenId(pub String);

impl AccessTokenId {
    pub fn from_str(s: &str) -> AccessTokenId {
        AccessTokenId(s.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
