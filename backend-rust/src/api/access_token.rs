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
    #[derive(Debug, Deserialize)]
    struct Data {
        email: String,
        hashed_pass: String,
    }

    #[derive(Debug, Serialize)]
    struct ResData {
        access_token: String,
    }

    let req_data: Data = try_or_res!(json_input(&req), 400, 0, "Invalid payload");
    let pg_conn = try_or_res!(server.pg.establish_connection(), 500, 1, "Server error");
    let user_id = try_or_res!(
        authenticate_user(
            &pg_conn,
            req_data.email.as_str(),
            req_data.hashed_pass.as_str()
        ),
        401,
        2,
        "Credentials are invalid"
    );
    let redis_conn = try_or_res!(server.redis.establish_connection(), 500, 1, "Server error");
    let token = try_or_res!(create_token(&redis_conn, user_id), 500, 1, "Server error");

    let res_data = ResData {
        access_token: token,
    };
    Response::json(&res_data)
}

fn authenticate_user(conn: &PgConnection, email: &str, hashed_pass: &str) -> Result<i32, Error> {
    use crate::postgres::schema::users::{self, columns};
    use diesel::prelude::*;

    Ok(users::table
        .filter(columns::email.eq(email))
        .filter(columns::hashed_pass.eq(hashed_pass))
        .select(columns::id)
        .first::<i32>(conn)?)
}

fn create_token(conn: &RedisConn, user_id: i32) -> Result<String, Error> {
    let mut buf = [0; TOKEN_SIZE];
    rand::thread_rng().fill_bytes(&mut buf[..]);
    let token = base64::encode(&buf[..]);
    conn.set_ex(&token, user_id, TOKEN_EXPIRE_SEC)?;
    Ok(token)
}
