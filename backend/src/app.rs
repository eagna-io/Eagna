mod auth;
mod cronjob;
mod failure_response;
mod markets;
mod me;
mod users;

pub use self::auth::validate_bearer_header;
pub use self::failure_response::FailureResponse;
use crate::domain::{
    models::market::MarketId,
    services::{AccessTokenStore, MarketStore, StoreFactory, UserStore},
};

use log::{error, info};
use rouille::{router, Request, Response};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ApiServer<F> {
    store_factory: F,
    access_allow_hosts: String, // comma separated host names
}

impl<F> ApiServer<F> {
    pub fn new<S>(store_factory: F, access_allow_hosts: S) -> ApiServer<F>
    where
        S: Into<String>,
    {
        ApiServer {
            store_factory,
            access_allow_hosts: access_allow_hosts.into(),
        }
    }

    pub fn run<A, S>(self, addr: A)
    where
        A: std::net::ToSocketAddrs,
        F: StoreFactory<S> + Send + Sync + 'static,
        S: AccessTokenStore + MarketStore + UserStore + Send + 'static,
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
            "OPTIONS" => Some(Response::text("").with_additional_header(
                "Access-Control-Allow-Headers",
                "Authorization, Content-Type",
            )),
            _ => None,
        }
    }

    pub fn process_request<S>(&self, req: &Request) -> Response
    where
        F: StoreFactory<S> + Send + Sync + 'static,
        S: AccessTokenStore + MarketStore + UserStore + Send + 'static,
    {
        self.store_factory
            .transaction(|store| routing(store, req))
            .unwrap_or_else(<FailureResponse as Into<Response>>::into)
    }

    pub fn append_cors_header(&self, resp: Response) -> Response {
        resp.with_additional_header(
            "Access-Control-Allow-Origin",
            self.access_allow_hosts.clone(),
        )
    }
}

pub fn routing<S>(store: &mut S, req: &Request) -> Result<Response, FailureResponse>
where
    S: AccessTokenStore + MarketStore + UserStore + Send + 'static,
{
    router!(req,
        (GET) (/me/) => {
            me::get(store, req)
        },
        (GET) (/me/markets/) => {
            me::markets::get(store, req)
        },
        (POST) (/users/) => {
            users::post(store, req)
        },
        (POST) (/markets/) => {
            markets::post(store, req)
        },
        (GET) (/markets/{id: MarketId}/) => {
            markets::get(store, req, id)
        },
        (PUT) (/markets/{id: MarketId}/) => {
            markets::put(store, req, id)
        },
        (GET) (/markets/{id: MarketId}/orders/) => {
            markets::orders::get_all(store, req, id)
        },
        (POST) (/markets/{id: MarketId}/orders/) => {
            markets::orders::post(store, req, id)
        },
        (GET) (/cronjob/check_markets/) => {
            cronjob::check_markets::get(store, req)
        },
        _ => Err(FailureResponse::ResourceNotFound)
    )
}

fn log_ok(req: &Request, resp: &Response, elap: Duration) {
    info!(
        "{} {} {} {} ms",
        resp.status_code,
        req.method(),
        req.raw_url(),
        elap.as_millis()
    );
}

fn log_err(req: &Request, elap: Duration) {
    error!(
        "Handler panicked: {} {} {} ms",
        req.method(),
        req.raw_url(),
        elap.as_millis()
    );
}
