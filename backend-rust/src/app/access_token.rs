use crate::{
    app::FailureResponse,
    domain::{
        models::access_token::{AccessToken, AccessTokenId},
        services::{AccessTokenStore, UserStore},
    },
};
use rouille::{input::json::json_input, Request, Response};

pub fn create<S>(store: &S, req: &Request) -> Result<Response, FailureResponse>
where
    S: UserStore + AccessTokenStore,
{
    // 認証情報のチェック
    let req_data = json_input::<ReqData>(&req).map_err(|e| {
        dbg!(e);
        FailureResponse::InvalidPayload
    })?;
    let query_res = store.query_user_by_email_and_hashed_pass(
        req_data.email.as_str(),
        req_data.hashed_pass.as_str(),
    );
    let user = match query_res {
        Ok(Some(user)) => user,
        Ok(None) => return Err(FailureResponse::Unauthorized),
        Err(e) => {
            dbg!(e);
            return Err(FailureResponse::ServerError);
        }
    };

    // 新規トークンの発行
    let access_token = AccessToken::new(user.id);
    store.save(&access_token).map_err(|e| {
        dbg!(e);
        FailureResponse::ServerError
    })?;

    let res_data = ResData {
        access_token: access_token.id,
    };
    Ok(Response::json(&res_data))
}

#[derive(Debug, Deserialize)]
struct ReqData {
    email: String,
    hashed_pass: String,
}

#[derive(Debug, Serialize)]
struct ResData {
    access_token: AccessTokenId,
}
