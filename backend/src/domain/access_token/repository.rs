use super::{AccessToken, AccessTokenId};
use crate::domain::user::UserId;
use crate::infra::{FirebaseInfra, RedisInfra};

#[derive(From)]
pub struct AccessTokenRepository<'a, 'b> {
    firebase: &'a dyn FirebaseInfra,
    redis: &'b dyn RedisInfra,
}

impl<'a, 'b> AccessTokenRepository<'a, 'b> {
    pub fn query_access_token(
        &self,
        access_token_id: &AccessTokenId,
    ) -> Result<Option<AccessToken>, failure::Error> {
        if let Some(user_id_str) = self
            .redis
            .query_user_id_by_access_token(access_token_id.as_str())?
        {
            // Cache を返すだけ
            let access_token = AccessToken {
                id: access_token_id.clone(),
                user_id: UserId::from_str(user_id_str.as_str()),
            };
            return Ok(Some(access_token));
        }

        // Firebase に問い合わせる

        // Token を Cache する時間 = 10分
        const CACHE_EXPIRE_SEC: usize = 60 * 60;

        if let Some(user_id_str) = self
            .firebase
            .query_user_by_access_token(access_token_id.as_str())?
        {
            // Redis に Cache を保存
            self.redis.save_access_token(
                access_token_id.as_str(),
                user_id_str.as_str(),
                CACHE_EXPIRE_SEC,
            )?;
            let access_token = AccessToken {
                id: access_token_id.clone(),
                user_id: UserId::from_str(user_id_str.as_str()),
            };
            return Ok(Some(access_token));
        }

        // Firebase にないトークンの場合
        Ok(None)
    }
}
