use crate::{api::FailureResponse, Server};
use diesel::{pg::PgConnection, result::Error as PgError};
use redis::{Commands, Connection as RedisConn};
use rouille::{input::json::json_input, Request, Response};

// Tokenの長さは64文字。base64の規格により、それは48byteになる
const TOKEN_SIZE: usize = 64 / 4 * 3;
const TOKEN_EXPIRE_SEC: usize = 60 * 60 * 24;

pub fn create(server: &Server, req: &Request) -> Result<Response, FailureResponse> {
    #[derive(Debug, Deserialize)]
    struct ReqData {
        email: String,
        hashed_pass: String,
    }

    #[derive(Debug, Serialize)]
    struct ResData {
        access_token: String,
    }

    // 認証情報のチェック
    let req_data = json_input::<ReqData>(&req).map_err(|_e| FailureResponse::InvalidPayload)?;
    let pg_conn = server.get_new_pg_conn()?;
    let user_id = authenticate_user(
        &pg_conn,
        req_data.email.as_str(),
        req_data.hashed_pass.as_str(),
    )?;

    // 新規トークンの発行
    let redis_conn = server.get_new_redis_conn()?;
    let token = create_token(&redis_conn, user_id).map_err(|_e| FailureResponse::ServerError)?;

    let res_data = ResData {
        access_token: token,
    };
    Ok(Response::json(&res_data))
}

fn authenticate_user(
    conn: &PgConnection,
    email: &str,
    hashed_pass: &str,
) -> Result<i32, FailureResponse> {
    use crate::postgres::schema::users::{self, columns};
    use diesel::prelude::*;

    users::table
        .filter(columns::email.eq(email))
        .filter(columns::hashed_pass.eq(hashed_pass))
        .select(columns::id)
        .first(conn)
        .map_err(|e| match e {
            PgError::NotFound => FailureResponse::Unauthorized,
            _ => FailureResponse::ServerError,
        })
}

fn create_token(conn: &RedisConn, user_id: i32) -> Result<String, FailureResponse> {
    use rand::RngCore;

    let mut buf = [0; TOKEN_SIZE];
    rand::thread_rng().fill_bytes(&mut buf[..]);
    let token = base64::encode(&buf[..]);
    conn.set_ex(&token, user_id, TOKEN_EXPIRE_SEC)
        .map_err(|_| FailureResponse::ServerError)?;
    Ok(token)
}
