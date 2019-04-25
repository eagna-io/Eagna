use crate::api::FailureResponse;
use redis::{Commands, Connection as RedisConn};
use rouille::Request;

pub fn validate_bearer_header(
    redis_conn: &RedisConn,
    req: &Request,
) -> Result<i32, FailureResponse> {
    req.header("Authorization")
        .ok_or(FailureResponse::Unauthorized)
        .and_then(extract_token)
        .and_then(move |token| check_token(redis_conn, token))
}

fn extract_token<'a>(header_val: &'a str) -> Result<&'a str, FailureResponse> {
    // トークンはbase64エンコードされた64文字の文字列
    // base64の仕様上、4の倍数長の文字列には=は含まれない
    let re = regex::Regex::new(r"^Bearer ([A-Za-z0-9+/]{64})$").unwrap();
    re.captures(header_val)
        .and_then(|cap| cap.get(1))
        .ok_or(FailureResponse::Unauthorized)
        .map(|mat| mat.as_str())
}

fn check_token(conn: &RedisConn, token: &str) -> Result<i32, FailureResponse> {
    let maybe_user_id: Option<i32> = conn.get(token).map_err(|_| FailureResponse::ServerError)?;
    maybe_user_id.ok_or(FailureResponse::Unauthorized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_valid_token() {
        let token = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz+/";
        let header_val = format!("Bearer {}", token);
        let res = extract_token(header_val.as_str());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), token);
    }

    #[test]
    fn try_extract_short_token_should_fail() {
        let token = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let header_val = format!("Bearer {}", token);
        let res = extract_token(header_val.as_str());
        assert!(res.is_err());
    }

    #[test]
    fn try_extract_long_token_should_fail() {
        let token = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz+/A";
        let header_val = format!("Bearer {}", token);
        let res = extract_token(header_val.as_str());
        assert!(res.is_err());
    }

    #[test]
    fn try_extract_non_base64_token_should_fail() {
        let token = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz+!";
        let header_val = format!("Bearer {}", token);
        let res = extract_token(header_val.as_str());
        assert!(res.is_err());
    }
}
