use crate::domain::{
    models::access_token::{AccessToken, AccessTokenId},
    services::Store,
};

pub trait AccessTokenStore: Store {
    fn save_access_token(&mut self, access_token: &AccessToken) -> Result<(), Self::Error>;

    fn query_access_token(
        &mut self,
        access_token_id: &AccessTokenId,
    ) -> Result<Option<AccessToken>, Self::Error>;
}
