use chrono::Utc;
use jsonwebtoken::{errors::Error as JwtError, DecodingKey, EncodingKey, Header, Validation};

const EXPIRES_IN_SECS: usize = 60 * 60 * 24;

lazy_static::lazy_static! {
    static ref SECRET: String = std::env::var("SECRET").unwrap();
    static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(SECRET.as_ref());
    static ref DECODING_KEY: DecodingKey<'static> = DecodingKey::from_secret(SECRET.as_ref());
}

// JWTを複数箇所で使うようになったらinfra化する。
pub struct UserInviteService {}

impl UserInviteService {
    fn encoding_key() -> &'static EncodingKey {
        &ENCODING_KEY
    }

    fn decoding_key() -> &'static DecodingKey<'static> {
        &DECODING_KEY
    }

    pub fn publish_invitation_token<S>(email: S) -> InvitationToken
    where
        S: Into<String>,
    {
        let now = Utc::now().timestamp();

        let claim = JWTClaim {
            email: email.into(),
            exp: now as usize + EXPIRES_IN_SECS,
        };

        let jwt = jsonwebtoken::encode(&Header::default(), &claim, Self::encoding_key()).unwrap();
        InvitationToken(jwt)
    }

    pub fn validate_invitation_token(token: &InvitationToken) -> Result<Invitation, JwtError> {
        let email = jsonwebtoken::decode::<JWTClaim>(
            token.0.as_str(),
            Self::decoding_key(),
            &Validation::default(),
        )?
        .claims
        .email;

        Ok(Invitation { email })
    }
}

pub struct Invitation {
    pub email: String,
}

#[derive(From)]
pub struct InvitationToken(String);

impl InvitationToken {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JWTClaim {
    email: String,
    exp: usize,
}
