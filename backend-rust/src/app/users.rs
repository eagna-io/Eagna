use crate::{
    app::{validate_bearer_header, FailureResponse},
    domain::{
        models::user::UserId,
        services::{user_store::NewUser, AccessTokenStore, UserStore},
    },
};
use rouille::{input::json::json_input, Request, Response};

pub fn post<S>(mut store: S, req: &Request) -> Result<Response, FailureResponse>
where
    S: AccessTokenStore + UserStore,
{
    #[derive(Deserialize)]
    struct ReqData {
        name: String,
        email: String,
    }

    #[derive(Serialize)]
    struct ResData<'a> {
        id: &'a UserId,
        name: &'a str,
        email: &'a str,
    }

    let req_data = json_input::<ReqData>(req).map_err(|_| FailureResponse::InvalidPayload)?;

    let access_token = validate_bearer_header(&mut store, req)?;

    let new_user = NewUser {
        id: &access_token.user_id,
        name: req_data.name.as_str(),
        email: req_data.email.as_str(),
    };

    store.save_user(new_user)?;
    store.commit()?;

    let res_data = ResData {
        id: &access_token.user_id,
        name: req_data.name.as_str(),
        email: req_data.email.as_str(),
    };

    Ok(Response::json(&res_data).with_status_code(201))
}
