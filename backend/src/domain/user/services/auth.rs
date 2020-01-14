use crate::domain::user::models::{User, UserId};
use crate::infra::postgres::{user::QueryUserCredentials, PostgresInfra};
use arrayvec::ArrayString;
use data_encoding::HEXUPPER;
use getset::Getters;
use rand::{thread_rng, Rng};
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

const CRED_LEN: usize = digest::SHA512_OUTPUT_LEN; // 64
const ENCODED_CRED_LEN: usize = CRED_LEN * 2;
static ALGO: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const N_ITER: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100_000) };

#[derive(From)]
pub struct UserAuthService<'a> {
    db: &'a dyn PostgresInfra,
}

impl<'a> UserAuthService<'a> {
    /// Userをemailとattempted_passによって認証する。
    /// つまりUserのsigninに使用する。
    /// 成功した場合はOk(AuthorizedUser)を返す。
    /// 失敗した場合はErr(_) を返す。
    pub fn authenticate(
        &self,
        email: &str,
        attempted_pass: &str,
    ) -> anyhow::Result<AuthorizedUser> {
        let QueryUserCredentials { id, cred, salt } = self
            .db
            .query_user_credentials(email)?
            .ok_or(anyhow::anyhow!("Authentication failed"))?;

        Self::verify_credentials(salt.as_slice(), cred.as_slice(), attempted_pass)?;

        Ok(AuthorizedUser {
            id: UserId::from(id),
        })
    }

    fn verify_credentials(salt: &[u8], cred: &[u8], attempted_pass: &str) -> anyhow::Result<()> {
        pbkdf2::verify(ALGO, N_ITER, salt, attempted_pass.as_bytes(), cred)
            .map_err(|_| anyhow::anyhow!("Authentication failed"))
    }

    pub fn derive_credentials(raw_pass: &str) -> Credentials {
        let salt = gen_salt();

        let mut cred = [0u8; CRED_LEN];
        pbkdf2::derive(ALGO, N_ITER, &salt, raw_pass.as_bytes(), &mut cred);

        Credentials {
            salt: salt,
            cred: cred,
        }
    }
}

pub struct AuthorizedUser {
    id: UserId,
}

impl User for AuthorizedUser {
    fn id(&self) -> &UserId {
        &self.id
    }
}

#[derive(Copy, Clone, Getters)]
#[get = "pub"]
pub struct Credentials {
    salt: [u8; CRED_LEN],
    cred: [u8; CRED_LEN],
}

impl Credentials {
    pub fn salt_hex(&self) -> ArrayString<[u8; ENCODED_CRED_LEN]> {
        encode(self.salt)
    }

    pub fn cred_hex(&self) -> ArrayString<[u8; ENCODED_CRED_LEN]> {
        encode(self.cred)
    }
}

fn gen_salt() -> [u8; CRED_LEN] {
    let mut salt = [0u8; CRED_LEN];
    thread_rng().fill(&mut salt);
    salt
}

fn encode(bytes: [u8; CRED_LEN]) -> ArrayString<[u8; ENCODED_CRED_LEN]> {
    let mut encoded = [0u8; ENCODED_CRED_LEN];
    HEXUPPER.encode_mut(&bytes[..], &mut encoded[..]);
    ArrayString::from_byte_string(&encoded).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_and_verify() {
        let pass = "hogehoge";

        let Credentials { salt, cred } = UserAuthService::derive_credentials(pass);
        UserAuthService::verify_credentials(&salt[..], &cred[..], pass).unwrap();
    }
}
