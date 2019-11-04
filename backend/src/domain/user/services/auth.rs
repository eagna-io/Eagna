use crate::domain::user::models::{User, UserId};
use crate::infra::postgres::{user::QueryUserCredentials, PostgresInfra};
use failure::{err_msg, Fallible};
use rand::{thread_rng, Rng};
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

const CRED_LEN: usize = digest::SHA512_OUTPUT_LEN;
static ALGO: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const N_ITER: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100_000) };

#[derive(From)]
pub struct UserAuthService<'a> {
    db: &'a dyn PostgresInfra,
}

impl<'a> UserAuthService<'a> {
    pub fn authenticate(&self, email: &str, attempted_pass: &str) -> Fallible<AuthorizedUser> {
        let QueryUserCredentials { id, cred, salt } = self
            .db
            .query_user_credentials(email)?
            .ok_or(err_msg("Authentication failed"))?;

        pbkdf2_verify(cred.as_str(), salt.as_str(), attempted_pass)?;

        Ok(AuthorizedUser {
            id: UserId::from(id),
        })
    }
}

fn pbkdf2_verify(stored_pass: &str, salt: &str, attempted_pass: &str) -> Fallible<()> {
    pbkdf2::verify(
        ALGO,
        N_ITER,
        salt.as_bytes(),
        attempted_pass.as_bytes(),
        stored_pass.as_bytes(),
    )
    .map_err(|_| err_msg("Authentication failed"))
}

fn gen_salt() -> [u8; CRED_LEN] {
    let mut salt = [0u8; CRED_LEN];
    thread_rng().fill(&mut salt);
    salt
}

pub struct AuthorizedUser {
    id: UserId,
}

impl User for AuthorizedUser {
    fn id(&self) -> &UserId {
        &self.id
    }
}
