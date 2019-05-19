use crate::domain::models::user::UserId;

// AccessTokenId の文字列長
pub const TOKEN_LENGTH: usize = 64;
// base64 encode したら64文字になるbyteサイズ
const TOKEN_BYTE_SIZE: usize = TOKEN_LENGTH / 4 * 3;
// Tokenの有効期限。1日
pub const TOKEN_EXPIRE_SEC: usize = 60 * 60 * 24;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessToken {
    pub id: AccessTokenId,
    pub user_id: UserId,
}

#[derive(Clone, Copy)]
pub struct AccessTokenId(pub [u8; TOKEN_LENGTH]);

impl AccessToken {
    pub fn new(user_id: UserId) -> AccessToken {
        use rand::RngCore;

        let mut rand_bytes = [0; TOKEN_BYTE_SIZE];
        rand::thread_rng().fill_bytes(&mut rand_bytes[..]);

        let mut token_bytes = [0; TOKEN_LENGTH];
        base64::encode_config_slice(&rand_bytes[..], base64::STANDARD, &mut token_bytes[..]);
        let token = AccessTokenId(token_bytes);
        AccessToken { id: token, user_id }
    }
}

impl AccessTokenId {
    /// ## Panics
    /// if the length of string is not TOKEN_LENGTH.
    pub fn from_exact_length_str(s: &str) -> AccessTokenId {
        assert!(s.len() == TOKEN_LENGTH);
        let mut token_bytes = [0; TOKEN_LENGTH];
        token_bytes.copy_from_slice(s.as_bytes());
        AccessTokenId(token_bytes)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0[..]).unwrap()
    }
}

impl serde::Serialize for AccessTokenId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0[..])
    }
}

impl std::fmt::Debug for AccessTokenId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", std::str::from_utf8(&self.0[..]))
    }
}

impl std::cmp::PartialEq for AccessTokenId {
    fn eq(&self, other: &Self) -> bool {
        &self.0[..] == &other.0[..]
    }
}

impl std::cmp::Eq for AccessTokenId {}
