mod auth;
mod cronjob;
mod failure_response;
mod infra_manager;
mod markets;
mod req;
mod users;

pub use self::auth::validate_bearer_header;
pub use self::failure_response::FailureResponse;
pub use self::infra_manager::{InfraManager, InfraManagerFactory};
pub use self::req::get_params;
use crate::domain::market::MarketId;

use rouille::{router, Request, Response};
use std::time::Duration;

#[derive(Debug, Clone, Constructor)]
pub struct ApiServer {
    infra_factory: InfraManagerFactory,
    access_allow_hosts: String, // comma separated host names
}

impl ApiServer {
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
        routing(infra, req).unwrap_or_else(<FailureResponse as Into<Response>>::into)
    }

    pub fn append_cors_header(&self, resp: Response) -> Response {
        resp.with_additional_header(
            "Access-Control-Allow-Origin",
            self.access_allow_hosts.clone(),
        )
    }
}

pub fn routing(infra: InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    router!(req,
        (POST) (/users/) => {
            users::post(infra, req)
        },
        (GET) (/markets/) => {
            markets::get_list(infra, req)
        },
        (POST) (/markets/) => {
            markets::post(infra, req)
        },
        (GET) (/markets/{id: MarketId}/) => {
            markets::get(infra, req, id)
        },
        (PUT) (/markets/{id: MarketId}/) => {
            markets::put(infra, req, id)
        },
        (GET) (/markets/{id: MarketId}/orders/) => {
            markets::orders::get_all(infra, req, id)
        },
        (POST) (/markets/{id: MarketId}/orders/) => {
            markets::orders::post(infra, req, id)
        },
        (GET) (/cronjob/check_markets/) => {
            cronjob::check_markets::get(infra, req)
        },
        _ => Err(FailureResponse::ResourceNotFound)
    )
}

fn log_ok(req: &Request, resp: &Response, elap: Duration) {
    log::info!(
        "{} {} {} {} ms",
        resp.status_code,
        req.method(),
        req.raw_url(),
        elap.as_millis()
    );
    log::trace!("Request : {:?}", req);
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
