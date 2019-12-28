use crate::domain::user::models::{
    access_token::{AccessToken, AccessTokenId, ACCESS_TOKEN_EXPIRE_SEC},
    UserId,
};
use crate::infra::redis::RedisInfra;
use failure::Fallible;

#[derive(From)]
pub struct AccessTokenRepository<'a> {
    redis: &'a dyn RedisInfra,
}

impl<'a> AccessTokenRepository<'a> {
    pub fn save(&self, access_token: &AccessToken) -> Fallible<()> {
        self.redis.save_access_token(
            access_token.id.as_str(),
            access_token.user_id.as_uuid(),
            ACCESS_TOKEN_EXPIRE_SEC,
        )
    }

    pub fn query(&self, access_token_id: &AccessTokenId) -> Fallible<Option<AccessToken>> {
        Ok(self
            .redis
            .query_user_id_by_access_token(access_token_id.as_str())?
            .map(|user_id| AccessToken::from((*access_token_id, UserId::from(user_id)))))
    }

    pub fn delete(&self, access_token: &AccessToken) -> Fallible<()> {
        self.redis.delete_access_token(access_token.id.as_str())
    }
}
