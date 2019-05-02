pub mod markets;

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

    unimplemented!();
}
