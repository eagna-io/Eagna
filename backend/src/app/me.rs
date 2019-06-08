pub mod markets;

use crate::{
    app::{validate_bearer_header, FailureResponse},
    domain::models::user::UserId,
    domain::services::{AccessTokenStore, UserStore},
};
use rouille::{Request, Response};

pub fn get<S>(mut store: S, req: &Request) -> Result<Response, FailureResponse>
where
    S: AccessTokenStore + UserStore,
{
    let access_token = validate_bearer_header(&mut store, req)?;
    let user = match store.query_user(&access_token.user_id)? {
        Some(user) => user,
        None => {
            // Firebase上には存在するが、DBに登録されていない
            log::debug!("User need to sign-up first");
            return Err(FailureResponse::Unauthorized);
        }
    };

    let res_data = ResData {
        id: user.id,
        name: user.name,
        email: user.email,
        is_admin: user.is_admin,
    };
    Ok(Response::json(&res_data))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResData {
    id: UserId,
    name: String,
    email: String,
    is_admin: bool,
}
