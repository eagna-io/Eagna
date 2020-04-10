use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use crop_infra::jwt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Adminモデルを表現するトレイト
/// このトレイトは最も基本的な要求しかしない
pub trait Admin {
    fn id(&self) -> &AdminId;

    fn gen_access_token(&self) -> AccessToken {
        AccessToken::new(self.id())
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct AdminId(pub Uuid);

/*
 * ===========
 * AccessToken
 * ===========
 */
const ACCESS_TOKEN_EXPIRE_DAYS: i64 = 30;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
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

    pub fn encode(&self) -> String {
        let claim = JwtClaim {
            admin_id: self.admin_id,
            exp: self.expire_at.timestamp() as usize,
        };
        jwt::encode(&claim).unwrap()
    }

    pub fn decode(raw: &str) -> anyhow::Result<Self> {
        let claim = jwt::decode::<JwtClaim>(raw)?;
        Ok(AccessToken {
            admin_id: claim.admin_id,
            expire_at: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(claim.exp as i64, 0),
                Utc,
            ),
        })
    }
}

impl std::str::FromStr for AccessToken {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        AccessToken::decode(s)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaim {
    admin_id: AdminId,
    exp: usize,
}

/*
 * ==================
 * AuthenticatedAdmin
 * ==================
 */
pub struct AuthenticatedAdmin {
    pub id: AdminId,
}

impl Admin for AuthenticatedAdmin {
    fn id(&self) -> &AdminId {
        &self.id
    }
}

impl From<AccessToken> for AuthenticatedAdmin {
    fn from(token: AccessToken) -> AuthenticatedAdmin {
        AuthenticatedAdmin { id: token.admin_id }
    }
}
