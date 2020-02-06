use jsonwebtoken::{errors::Error as JwtError, DecodingKey, EncodingKey, Header, Validation};
use lazycell::AtomicLazyCell;
use serde::{de::DeserializeOwned, Serialize};

pub struct Jwt {
    inner: AtomicLazyCell<JwtInner<'static>>,
}

impl Jwt {
    pub const GLOBAL: Self = Jwt {
        inner: AtomicLazyCell::NONE,
    };

    pub fn init(secret: &'static [u8]) {
        let jwt = JwtInner::new(secret);
        Jwt::GLOBAL.inner.fill(jwt).unwrap()
    }

    pub fn encode<T: Serialize>(claim: &T) -> Result<String, JwtError> {
        Jwt::GLOBAL.inner.borrow().unwrap().encode(claim)
    }

    pub fn decode<T: DeserializeOwned>(token: &str) -> Result<T, JwtError> {
        Jwt::GLOBAL.inner.borrow().unwrap().decode(token)
    }
}

#[derive(Debug)]
struct JwtInner<'a> {
    secret: &'a [u8],

    // cache
    encoding_key: EncodingKey,
    decoding_key: DecodingKey<'a>,
}

impl<'a> JwtInner<'a> {
    fn new(secret: &'a [u8]) -> JwtInner<'a> {
        JwtInner {
            secret,
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    fn encode<T: Serialize>(&self, claim: &T) -> Result<String, JwtError> {
        jsonwebtoken::encode(&Header::default(), claim, &self.encoding_key)
    }

    fn decode<T: DeserializeOwned>(&self, token: &str) -> Result<T, JwtError> {
        Ok(jsonwebtoken::decode(token, &self.decoding_key, &Validation::default())?.claims)
    }
}
