use failure::Error;
use redis::{Commands, Connection as RedisConn};
use diesel::pg::PgConnection;

const TOKEN_LENGTH: usize = 8;
const TOKEN_EXPIRE_SEC: usize = 60 * 60 * 24;

pub fn authenticate_user(
    conn: &PgConnection,
    email: &str,
    hashed_pass: &str,
) -> Result<i32, Error> {
    use diesel::prelude::*;
    use crate::postgres::schema::users::{self, columns};

    Ok(users::table
        .filter(columns::email.eq(email))
        .filter(columns::hashed_pass.eq(hashed_pass))
        .select(columns::id)
        .first::<i32>(conn)?)
}

pub fn create_token(conn: &RedisConn, user_id: i32) -> Result<String, Error> {
    let token = base64::encode(&rand::random::<[u8; TOKEN_LENGTH]>());
    conn.set_ex(&token, user_id, TOKEN_EXPIRE_SEC)?;
    Ok(token)
}

pub fn check_token(conn: &RedisConn, token: &str) -> Result<i32, Error> {
    let maybe_user_id: Option<i32> = conn.get(token)?;
    maybe_user_id.ok_or(Error::from(InvalidTokenError))
}

#[derive(Debug, Clone, Copy)]
pub struct InvalidTokenError;

impl ::std::fmt::Display for InvalidTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid token error")
    }
}

impl failure::Fail for InvalidTokenError {
    fn name(&self) -> Option<&str> {
        Some("invalid token error")
    }
}
