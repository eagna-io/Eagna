use super::{auth::validate_bearer_header, failure_response::FailureResponse};
use crate::Server;
use failure::Error;
use rouille::{Request, Response};

pub fn get_me(server: &Server, req: &Request) -> Response {
    inner(server, req).unwrap_or_else(<FailureResponse as Into<Response>>::into)
}

fn inner(server: &Server, req: &Request) -> Result<Response, FailureResponse> {
    let redis_conn = server
        .redis
        .establish()
        .map_err(|_e| FailureResponse::ServerError)?;
    let user_id =
        validate_bearer_header(&redis_conn, req).map_err(|_e| FailureResponse::Unauthorized)?;
    let pg_conn = server
        .pg
        .establish()
        .map_err(|_e| FailureResponse::ServerError)?;
    let (name, email) = query_user(&pg_conn, user_id).map_err(|_e| FailureResponse::ServerError)?;

    #[derive(Debug, Serialize)]
    struct ResData {
        user_id: i32,
        name: String,
        email: String,
    }

    let res_data = ResData {
        user_id,
        name,
        email,
    };
    Ok(Response::json(&res_data))
}

fn query_user(conn: &PgConn, user_id: i32) -> Result<(String, String), Error> {
    use crate::postgres::schema::users::{self, columns};
    use diesel::prelude::*;

    Ok(users::table
        .filter(columns::id.eq(user_id))
        .select((columns::name, columns::email))
        .first::<(String, String)>(conn)?)
}
