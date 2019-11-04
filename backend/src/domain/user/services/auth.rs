use crate::domain::user::models::{User, UserId};
use crate::infra::postgres::{user::QueryUserCredentials, PostgresInfra};
use arrayvec::ArrayString;
use data_encoding::HEXUPPER;
use failure::{err_msg, Fallible};
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
    pub fn authenticate(&self, email: &str, attempted_pass: &str) -> Fallible<AuthorizedUser> {
        let QueryUserCredentials { id, cred, salt } = self
            .db
            .query_user_credentials(email)?
            .ok_or(err_msg("Authentication failed"))?;

        pbkdf2::verify(
            ALGO,
            N_ITER,
            salt.as_bytes(),
            attempted_pass.as_bytes(),
            cred.as_bytes(),
        )
        .map_err(|_| err_msg("Authentication failed"))?;

        Ok(AuthorizedUser {
            id: UserId::from(id),
        })
    }

    pub fn derive_credentials(raw_pass: &str) -> Credentials {
        let salt = gen_salt();

        let mut cred = [0u8; CRED_LEN];
        pbkdf2::derive(ALGO, N_ITER, &salt, raw_pass.as_bytes(), &mut cred);

        let mut encoded_salt = [0u8; ENCODED_CRED_LEN];
        HEXUPPER.encode_mut(&salt[..], &mut encoded_salt[..]);

        let mut encoded_cred = [0u8; ENCODED_CRED_LEN];
        HEXUPPER.encode_mut(&cred[..], &mut encoded_cred[..]);

        Credentials {
            salt: ArrayString::from_byte_string(&encoded_salt).unwrap(),
            credential: ArrayString::from_byte_string(&encoded_cred).unwrap(),
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

#[derive(Copy, Clone, Debug)]
pub struct Credentials {
    pub salt: ArrayString<[u8; ENCODED_CRED_LEN]>,
    pub credential: ArrayString<[u8; ENCODED_CRED_LEN]>,
}

fn gen_salt() -> [u8; CRED_LEN] {
    let mut salt = [0u8; CRED_LEN];
    thread_rng().fill(&mut salt);
    salt
}
