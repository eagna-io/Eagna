use crate::domain::models::{
    access_token::{AccessToken, AccessTokenId},
    user::UserId,
};
use redis::{Commands, Connection as RedisConn};

pub fn save_access_token(
    conn: &RedisConn,
    token: &AccessToken,
    expire_sec: usize,
) -> Result<(), failure::Error> {
    Ok(conn.set_ex(token.id.as_str(), token.user_id.as_str(), expire_sec)?)
}

pub fn query_access_token(
    conn: &RedisConn,
    token_id: &AccessTokenId,
) -> Result<Option<AccessToken>, failure::Error> {
    match conn.get::<_, Option<String>>(token_id.as_str())? {
        Some(user_id) => Ok(Some(AccessToken {
            id: token_id.clone(),
            user_id: UserId::from_str(user_id.as_str()),
        })),
        None => Ok(None),
    }
}
