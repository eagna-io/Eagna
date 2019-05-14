use crate::domain::models::user::UserId;

// AccessTokenId の文字列長
pub const TOKEN_LENGTH: usize = 64;
// base64 encode したら64文字になるbyteサイズ
const TOKEN_BYTE_SIZE: usize = TOKEN_LENGTH / 4 * 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessToken {
    pub id: AccessTokenId,
    pub user_id: UserId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AccessTokenId(pub String);

impl AccessToken {
    pub fn new(user_id: UserId) -> AccessToken {
        use rand::RngCore;

        let mut buf = [0; TOKEN_BYTE_SIZE];
        rand::thread_rng().fill_bytes(&mut buf[..]);
        let token = AccessTokenId(base64::encode(&buf[..]));
        AccessToken { id: token, user_id }
    }
}
