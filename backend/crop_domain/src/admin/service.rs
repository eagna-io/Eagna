use arrayvec::ArrayString;
use data_encoding::HEXUPPER;
use rand::{thread_rng, Rng};
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

const CRED_LEN: usize = digest::SHA512_OUTPUT_LEN; // 64
const ENCODED_CRED_LEN: usize = CRED_LEN * 2;
static ALGO: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const N_ITER: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100_000) };

pub fn verify_credentials(salt: &[u8], cred: &[u8], attempted_pass: &str) -> anyhow::Result<()> {
    pbkdf2::verify(ALGO, N_ITER, salt, attempted_pass.as_bytes(), cred)
        .map_err(|_| anyhow::anyhow!("Authentication failed"))
}

pub fn derive_credentials(raw_pass: &str) -> Credentials {
    let salt = gen_salt();

    let mut cred = [0u8; CRED_LEN];
    pbkdf2::derive(ALGO, N_ITER, &salt, raw_pass.as_bytes(), &mut cred);

    Credentials { salt, cred }
}

#[derive(Copy, Clone)]
pub struct Credentials {
    pub salt: [u8; CRED_LEN],
    pub cred: [u8; CRED_LEN],
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
