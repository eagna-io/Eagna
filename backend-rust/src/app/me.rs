pub mod markets;

use crate::{
    app::{validate_bearer_header, FailureResponse},
    domain::models::user::UserId,
    domain::services::{AccessTokenStore, UserStore},
};
use rouille::{Request, Response};

pub fn get<S>(store: &S, req: &Request) -> Result<Response, FailureResponse>
where
    S: AccessTokenStore + UserStore,
{
    let access_token = validate_bearer_header(store, req)?;
    let user = match store.query_user(&access_token.user_id) {
        Ok(Some(user)) => user,
        Ok(None) => {
            println!("User does not exists, but AccessToken exists");
            return Err(FailureResponse::ServerError);
        }
        Err(e) => {
            dbg!(e);
            return Err(FailureResponse::ServerError);
        }
    };

    let res_data = ResData {
        id: user.id,
        name: user.name,
        email: user.email,
    };
    Ok(Response::json(&res_data))
}

#[derive(Debug, Serialize)]
struct ResData {
    id: UserId,
    name: String,
    email: String,
}
