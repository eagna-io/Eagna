#[macro_use]
pub mod failure_response;
pub mod auth;

use crate::auth::{authenticate_user, create_token};
use rouille::{input::json::json_input, router, Request, Response};

#[derive(Debug, Clone)]
pub struct Server {
    pg: crate::PgConnectionFactory,
    redis: crate::RedisConnectionFactory,
}

impl Server {
    pub fn new_with_env() -> Server {
        Server {
            pg: crate::PgConnectionFactory::new_with_env(),
            redis: crate::RedisConnectionFactory::new_with_env(),
        }
    }

    pub fn run<A: std::net::ToSocketAddrs>(self, addr: A) {
        rouille::Server::new(addr, move |req| {
            rouille::log(req, ::std::io::stdout(), || {
                router!(req,
                    (POST) (/access_token) => {
                        self.create_access_token(req)
                    },
                    _ => Response::empty_404()
                )
            })
        })
        .unwrap()
        .run();
    }

    fn create_access_token(&self, req: &Request) -> Response {
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
        let pg_conn = try_or_res!(self.pg.establish_connection(), 500, 1, "Server error");
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
        let redis_conn = try_or_res!(self.redis.establish_connection(), 500, 1, "Server error");
        let token = try_or_res!(create_token(&redis_conn, user_id), 500, 1, "Server error");

        let res_data = ResData {
            access_token: token,
        };
        Response::json(&res_data)
    }
}
