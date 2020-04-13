use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use crop_infra::jwt;
use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod new;

pub use new::New;

pub fn new(name: String) -> New {
    New::new(name)
}

pub trait Account {
    fn id(&self) -> &AccountId;

    fn name(&self) -> &str;

    fn gen_access_token(&self) -> AccessToken {
        AccessToken::new(*self.id())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct AccountId(pub Uuid);

impl AccountId {
    fn new() -> AccountId {
        AccountId(Uuid::new_v4())
    }
}

/*
 * ============
 * AccessToken
 * ============
 */
const ACCESS_TOKEN_EXPIRE_DAYS: i64 = 30;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AccessToken {
    pub account_id: AccountId,
    pub expire_at: DateTime<Utc>,
}

impl AccessToken {
    pub fn new(account_id: AccountId) -> AccessToken {
        AccessToken {
            account_id,
            expire_at: Utc::now() + Duration::days(ACCESS_TOKEN_EXPIRE_DAYS),
        }
    }

    pub fn encode(&self) -> String {
        let claim = JwtClaim {
            account_id: self.account_id,
            exp: self.expire_at.timestamp() as usize,
        };
        jwt::encode(&claim).unwrap()
    }

    pub fn decode(raw: &str) -> anyhow::Result<Self> {
        let claim = jwt::decode::<JwtClaim>(raw)?;
        Ok(AccessToken {
            account_id: claim.account_id,
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
    account_id: AccountId,
    exp: usize,
}
