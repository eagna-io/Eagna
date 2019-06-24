use crate::{
    app::{validate_bearer_header, FailureResponse},
    domain::{
        models::user::UserId,
        services::{user_store::NewUser, AccessTokenStore, UserStore},
    },
};
use rouille::{input::json::json_input, Request, Response};

pub fn post<S>(store: &mut S, req: &Request) -> Result<Response, FailureResponse>
where
    S: AccessTokenStore + UserStore,
{
    #[derive(Deserialize)]
    struct ReqData {
        name: String,
        email: String,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct ResData<'a> {
        id: &'a UserId,
        name: &'a str,
        email: &'a str,
        is_admin: bool,
    }

    let req_data = json_input::<ReqData>(req).map_err(|_| FailureResponse::InvalidPayload)?;

    let access_token = validate_bearer_header(store, req)?;

    let new_user = NewUser {
        id: &access_token.user_id,
        name: req_data.name.as_str(),
        email: req_data.email.as_str(),
    };

    store.save_user(new_user)?;

    let res_data = ResData {
        id: &access_token.user_id,
        name: req_data.name.as_str(),
        email: req_data.email.as_str(),
        is_admin: false,
    };

    Ok(Response::json(&res_data).with_status_code(201))
}