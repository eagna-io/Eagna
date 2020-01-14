use super::UserId;
use arrayvec::ArrayString;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// 1週間Tokenをcacheする
pub const ACCESS_TOKEN_EXPIRE_SEC: usize = 60 * 60 * 24 * 7;

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub struct AccessToken {
    pub id: AccessTokenId,
    pub user_id: UserId,
}

impl AccessToken {
    pub fn new(user_id: &UserId) -> AccessToken {
        AccessToken {
            id: AccessTokenId::new(),
            user_id: *user_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, From)]
pub struct AccessTokenId(ArrayString<[u8; 64]>);

impl AccessTokenId {
    fn new() -> AccessTokenId {
        let random_str = thread_rng()
            .sample_iter(Alphanumeric)
            .take(64)
            .collect::<String>();
        AccessTokenId(ArrayString::from(&random_str).unwrap())
    }

    pub fn try_from_str(s: &str) -> anyhow::Result<AccessTokenId> {
        ArrayString::from(s)
            .map_err(|e| anyhow::Error::from(e.simplify()))
            .map(|inner| AccessTokenId(inner))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
