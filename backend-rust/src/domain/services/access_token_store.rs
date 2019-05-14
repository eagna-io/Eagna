use crate::domain::models::access_token::{AccessToken, AccessTokenId};

pub trait AccessTokenStore {
    type Error: std::fmt::Debug;

    fn save(&self, access_token: &AccessToken) -> Result<(), Self::Error>;

    fn query(&self, access_token_id: &AccessTokenId) -> Result<Option<AccessToken>, Self::Error>;
}
