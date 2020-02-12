#[macro_use]
extern crate serde;

mod auth;
mod failure_response;
mod infra_manager;
mod req;
mod routes;

pub use self::auth::validate_bearer_header;
pub use self::failure_response::FailureResponse;
pub use self::infra_manager::{InfraManager, InfraManagerFactory};
pub use self::req::{get_param, get_params};

use rouille::{Request, Response};
use std::time::Duration;

pub struct ApiServer {
    pub infra_factory: InfraManagerFactory,
    pub access_allow_hosts: String, // comma separated host names
}

impl ApiServer {
    pub fn new(infra_factory: InfraManagerFactory, access_allow_hosts: String) -> ApiServer {
        ApiServer {
            infra_factory,
            access_allow_hosts,
        }
    }

    pub fn run<A>(self, addr: A)
    where
        A: std::net::ToSocketAddrs,
    {
        rouille::Server::new(addr, move |req| {
            rouille::log_custom(req, log_ok, log_err, || {
                let res = match self.filter_cors_preflight(req) {
                    Some(res) => res,
                    None => self.process_request(req),
                };
                self.append_cors_header(res)
            })
        })
        .unwrap()
        .run();
    }

    pub fn filter_cors_preflight(&self, req: &Request) -> Option<Response> {
        match req.method() {
            "OPTIONS" => Some(
                Response::text("")
                    .with_additional_header(
                        "Access-Control-Allow-Headers",
                        "Authorization, Content-Type",
                    )
                    .with_additional_header(
                        "Access-Control-Allow-Methods",
                        "OPTION, GET, POST, PUT",
                    ),
            ),
            _ => None,
        }
    }

    pub fn process_request(&self, req: &Request) -> Response {
        let infra = self.infra_factory.create();
        routes::routing(&infra, req).unwrap_or_else(<FailureResponse as Into<Response>>::into)
    }

    pub fn append_cors_header(&self, resp: Response) -> Response {
        resp.with_additional_header(
            "Access-Control-Allow-Origin",
            self.access_allow_hosts.clone(),
        )
    }
}

fn log_ok(req: &Request, resp: &Response, elap: Duration) {
    log::info!(
        "{} {} {} {} ms",
        resp.status_code,
        req.method(),
        req.raw_url(),
        elap.as_millis()
    );
    log::debug!("Request : {:?}", req);
    log::debug!("Response : {:?}", resp);
}

fn log_err(req: &Request, elap: Duration) {
    log::error!(
        "Handler panicked: {} {} {} ms",
        req.method(),
        req.raw_url(),
        elap.as_millis()
    );
    log::debug!("Request : {:?}", req);
}
