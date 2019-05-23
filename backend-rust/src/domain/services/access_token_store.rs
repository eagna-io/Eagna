use crate::domain::{
    models::access_token::{AccessToken, AccessTokenId},
    services::Store,
};

pub trait AccessTokenStore: Store {
    fn query_access_token(
        &mut self,
        access_token_id: &AccessTokenId,
    ) -> Result<Option<AccessToken>, Self::Error>;
}
