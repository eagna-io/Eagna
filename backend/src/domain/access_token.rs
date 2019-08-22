mod repository;
pub use repository::*;

use crate::domain::user::UserId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessToken {
    pub id: AccessTokenId,
    pub user_id: UserId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Firebase が発行するアクセストークン
/// JWTだが、とりあえず使う予定はない
/// JWTをデコードする代わりに、Firebase に正当性の問い合わせを行っている
/// なぜならJWTのvalidationだけでは、keyがrevokeされた時などに対応できないから
pub struct AccessTokenId(String);

impl AccessTokenId {
    pub fn from_str(s: &str) -> AccessTokenId {
        AccessTokenId(s.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
