use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

/// Adminモデルを表現するトレイト
/// このトレイトは最も基本的な要求しかしない
pub trait Admin {
    fn id(&self) -> &AdminId;

    fn gen_access_token(&self) -> AccessToken {
        AccessToken::new(self.id())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AdminId(pub Uuid);

/*
 * ===========
 * AccessToken
 * ===========
 */
const ACCESS_TOKEN_EXPIRE_DAYS: i64 = 30;

pub struct AccessToken {
    pub admin_id: AdminId,
    pub expire_at: DateTime<Utc>,
}

impl AccessToken {
    pub fn new(admin_id: &AdminId) -> AccessToken {
        AccessToken {
            admin_id: *admin_id,
            expire_at: Utc::now() + Duration::days(ACCESS_TOKEN_EXPIRE_DAYS),
        }
    }
}
