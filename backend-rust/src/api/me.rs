pub mod orders;

use crate::{
    api::{validate_bearer_header, FailureResponse},
    Server,
};
use diesel::pg::PgConnection as PgConn;
use rouille::{Request, Response};

pub fn get(server: &Server, req: &Request) -> Result<Response, FailureResponse> {
    let redis_conn = server.get_new_redis_conn()?;
    let user_id = validate_bearer_header(&redis_conn, req)?;
    let pg_conn = server.get_new_pg_conn()?;
    let user = query_user(&pg_conn, user_id)?;

    #[derive(Debug, Serialize)]
    struct ResData {
        id: i32,
        name: String,
        email: String,
    }

    let res_data = ResData {
        id: user_id,
        name: user.name,
        email: user.email,
    };
    Ok(Response::json(&res_data))
}

#[derive(Debug, Queryable)]
struct User {
    name: String,
    email: String,
}

fn query_user(conn: &PgConn, user_id: i32) -> Result<User, FailureResponse> {
    use crate::postgres::schema::users::{self, columns};
    use diesel::prelude::*;

    users::table
        .filter(columns::id.eq(user_id))
        .select((columns::name, columns::email))
        .first::<User>(conn)
        // このケースでは、user_id によるクエリが成功しないのはサーバーのエラー
        .map_err(|_e| FailureResponse::ServerError)
}
