use super::failure_response::FailureResponse;
use crate::Server;
use diesel::pg::PgConnection;
use failure::Error;
use rand::RngCore;
use redis::{Commands, Connection as RedisConn};
use rouille::{input::json::json_input, Request, Response};

// Tokenの長さは64文字。base64の規格により、それは48byteになる
const TOKEN_SIZE: usize = 64 / 4 * 3;
const TOKEN_EXPIRE_SEC: usize = 60 * 60 * 24;

pub fn create_access_token(server: &Server, req: &Request) -> Response {
    inner(server, req).unwrap_or_else(<FailureResponse as Into<Response>>::into)
}

fn inner(server: &Server, req: &Request) -> Result<Response, FailureResponse> {
    #[derive(Debug, Deserialize)]
    struct ReqData {
        email: String,
        hashed_pass: String,
    }

    #[derive(Debug, Serialize)]
    struct ResData {
        access_token: String,
    }

    let req_data = json_input::<ReqData>(&req).map_err(|_e| FailureResponse::InvalidPayload)?;
    let pg_conn = server
        .pg
        .establish()
        .map_err(|_e| FailureResponse::ServerError)?;
    let user_id = authenticate_user(
        &pg_conn,
        req_data.email.as_str(),
        req_data.hashed_pass.as_str(),
    )
    .map_err(|_e| FailureResponse::Unauthorized)?;
    let redis_conn = server
        .redis
        .establish()
        .map_err(|_e| FailureResponse::ServerError)?;
    let token = create_token(&redis_conn, user_id).map_err(|_e| FailureResponse::ServerError)?;

    let res_data = ResData {
        access_token: token,
    };
    Ok(Response::json(&res_data))
}

fn authenticate_user(conn: &PgConnection, email: &str, hashed_pass: &str) -> Result<i32, Error> {
    use crate::postgres::schema::users::{self, columns};
    use diesel::prelude::*;

    Ok(users::table
        .filter(columns::email.eq(email))
        .filter(columns::hashed_pass.eq(hashed_pass))
        .select(columns::id)
        .first(conn)?)
}

fn create_token(conn: &RedisConn, user_id: i32) -> Result<String, Error> {
    let mut buf = [0; TOKEN_SIZE];
    rand::thread_rng().fill_bytes(&mut buf[..]);
    let token = base64::encode(&buf[..]);
    conn.set_ex(&token, user_id, TOKEN_EXPIRE_SEC)?;
    Ok(token)
}
