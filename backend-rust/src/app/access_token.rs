use crate::{
    app::FailureResponse,
    domain::{
        models::access_token::{AccessToken, AccessTokenId},
        services::{AccessTokenStore, UserStore},
    },
};
use rouille::{input::json::json_input, Request, Response};

pub fn create<S>(mut store: S, req: &Request) -> Result<Response, FailureResponse>
where
    S: UserStore + AccessTokenStore,
{
    // 認証情報のチェック
    let req_data = json_input::<ReqData>(&req).map_err(|e| {
        dbg!(e);
        FailureResponse::InvalidPayload
    })?;
    let user = match store.query_user_by_email_and_hashed_pass(
        req_data.email.as_str(),
        req_data.hashed_pass.as_str(),
    )? {
        Some(user) => user,
        None => return Err(FailureResponse::Unauthorized),
    };

    // 新規トークンの発行
    let access_token = AccessToken::new(user.id);
    store.save_access_token(&access_token)?;
    store.commit()?;

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
