mod failure_response;
mod access_token;
mod auth;
mod me;
mod markets;
mod cronjob;

pub use self::auth::validate_bearer_header;
pub use self::failure_response::FailureResponse;

use diesel::pg::PgConnection as PgConn;
use redis::Connection as RedisConn;
use rouille::{router, Request, Response};

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

    pub fn get_new_redis_conn(&self) -> Result<RedisConn, FailureResponse> {
        self.redis
            .establish()
            .map_err(|_e| FailureResponse::ServerError)
    }

    pub fn get_new_pg_conn(&self) -> Result<PgConn, FailureResponse> {
        self.pg
            .establish()
            .map_err(|_e| FailureResponse::ServerError)
    }

    pub fn run<A: std::net::ToSocketAddrs>(self, addr: A) {
        rouille::Server::new(addr, move |req| {
            rouille::log(req, ::std::io::stdout(), || self.routing(req))
        })
        .unwrap()
        .run();
    }

    fn routing(&self, req: &Request) -> Response {
        let res = router!(req,
            (POST) (/access_token) => {
                access_token::create(&self, req)
            },
            (GET) (/me) => {
                me::get(&self, req)
            },
            (GET) (/me/markets) => {
                me::markets::get(&self, req)
            },
            (GET) (/markets/{id: i32}) => {
                markets::get(&self, req, id)
            },
            (GET) (/markets/{id: i32}/orders) => {
                markets::orders::get_all(self, req, id)
            },
            (POST) (/markets/{id: i32}/orders) => {
                markets::orders::post(self, req, id)
            },
            (GET) (/cronjob/open_market) => {
                cronjob::open_market::get(self, req)
            },
            _ => Err(FailureResponse::ResourceNotFound)
        );
        res.unwrap_or_else(<FailureResponse as Into<Response>>::into)
    }
}
