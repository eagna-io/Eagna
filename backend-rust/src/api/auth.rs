use failure::Error;
use redis::Connection as RedisConn;
use rouille::Request;

pub fn validate_bearer_header(redis_conn: &RedisConn, req: &Request) -> Result<i32, Error> {
    req.header("Authorization")
        .ok_or(Error::from(AuthHeaderIsNotFound))
        .and_then(extract_token)
        .and_then(move |token| validate_token(redis_conn, token))
}

fn extract_token<'a>(header_val: &'a str) -> Result<&'a str, Error> {
    // トークンはbase64エンコードされた128文字の文字列
    // base64の仕様上、4の倍数長の文字列には=は含まれない
    let re = regex::Regex::new(r"^Bearer ([A-Za-z0-9+/]{128})").unwrap();
    re.captures(header_val)
        .and_then(|cap| cap.get(1))
        .ok_or(Error::from(InvalidAuthHeader))
        .map(|mat| mat.as_str())
}

fn validate_token(redis_conn: &RedisConn, token: &str) -> Result<i32, Error> {
    crate::auth::check_token(redis_conn, token)
}

#[derive(Debug, Copy, Clone)]
pub struct AuthHeaderIsNotFound;

impl std::fmt::Display for AuthHeaderIsNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Authorization header is not presented")
    }
}

impl failure::Fail for AuthHeaderIsNotFound {
    fn name(&self) -> Option<&str> {
        Some("Authorization header is not presented")
    }
}

#[derive(Debug, Copy, Clone)]
pub struct InvalidAuthHeader;

impl std::fmt::Display for InvalidAuthHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid Authorization header")
    }
}

impl failure::Fail for InvalidAuthHeader {
    fn name(&self) -> Option<&str> {
        Some("Invalid Authorization header")
    }
}
