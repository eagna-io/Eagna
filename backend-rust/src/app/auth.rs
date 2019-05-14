use crate::{
    app::FailureResponse,
    domain::models::access_token::{AccessToken, AccessTokenId, TOKEN_LENGTH},
    domain::services::AccessTokenStore,
};
use rouille::Request;

pub fn validate_bearer_header<S>(store: &S, req: &Request) -> Result<AccessToken, FailureResponse>
where
    S: AccessTokenStore,
{
    let header_val = req
        .header("Authorization")
        .ok_or(FailureResponse::Unauthorized)?;
    let token_id = extract_token(header_val)?;
    match store.query(&token_id) {
        Ok(Some(token)) => Ok(token),
        Ok(None) => Err(FailureResponse::Unauthorized),
        Err(e) => {
            dbg!(e);
            Err(FailureResponse::ServerError)
        }
    }
}

fn extract_token(header_val: &str) -> Result<AccessTokenId, FailureResponse> {
    // トークンはbase64エンコードされた64文字の文字列
    // base64の仕様上、4の倍数長の文字列には=は含まれない
    let re = regex::Regex::new(&format!(r"^Bearer ([A-Za-z0-9+/]{{{}}})$", TOKEN_LENGTH)).unwrap();
    re.captures(header_val)
        .and_then(|cap| cap.get(1))
        .ok_or(FailureResponse::Unauthorized)
        .map(|mat| AccessTokenId(mat.as_str().into()))
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
