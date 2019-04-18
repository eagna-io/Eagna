#[macro_use]
pub mod failure_response;
pub mod access_token;
pub mod auth;

use self::access_token::create_access_token;
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

    pub fn run<A: std::net::ToSocketAddrs>(self, addr: A) {
        rouille::Server::new(addr, move |req| {
            rouille::log(req, ::std::io::stdout(), || self.routing(req))
        })
        .unwrap()
        .run();
    }

    fn routing(&self, req: &Request) -> Response {
        router!(req,
            (POST) (/access_token) => {
                create_access_token(&self, req)
            },
            _ => Response::empty_404()
        )
    }
}
