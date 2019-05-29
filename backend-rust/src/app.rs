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
            rouille::log_custom(req, log_ok, log_err, || self.routing(req))
        })
        .unwrap()
        .run();
    }

    pub fn routing<S>(&self, req: &Request) -> Response
    where
        F: StoreFactory<S> + Send + Sync + 'static,
        S: AccessTokenStore + MarketStore + UserStore + Send + 'static,
    {
        let res = router!(req,
            (GET) (/me) => {
                me::get(self.store_factory.establish(), req)
            },
            (GET) (/me/markets) => {
                me::markets::get(self.store_factory.establish(), req)
            },
            (POST) (/users) => {
                users::post(self.store_factory.establish(), req)
            },
            (POST) (/markets) => {
                markets::post(self.store_factory.establish(), req)
            },
            (GET) (/markets/{id: MarketId}) => {
                markets::get(self.store_factory.establish(), req, id)
            },
            (PUT) (/markets/{id: MarketId}) => {
                markets::put(self.store_factory.establish(), req, id)
            },
            (GET) (/markets/{id: MarketId}/orders) => {
                markets::orders::get_all(self.store_factory.establish(), req, id)
            },
            (POST) (/markets/{id: MarketId}/orders) => {
                markets::orders::post(self.store_factory.establish(), req, id)
            },
            (GET) (/cronjob/open_market) => {
                cronjob::open_market::get(self.store_factory.establish(), req)
            },
            (GET) (/cronjob/close_market) => {
                cronjob::close_market::get(self.store_factory.establish(), req)
            },
            (OPTION) (/{_any_path: String}) => {
                Ok(self.cors_response())
            },
            _ => Err(FailureResponse::ResourceNotFound)
        );
        res.unwrap_or_else(<FailureResponse as Into<Response>>::into)
    }

    fn cors_response(&self) -> Response {
        Response::text("").with_additional_header(
            "Access-Control-Allow-Origin",
            self.access_allow_hosts.clone(),
        )
    }
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
