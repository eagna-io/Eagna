mod access_token;
mod auth;
mod cronjob;
mod failure_response;
mod markets;
mod me;

pub use self::auth::validate_bearer_header;
pub use self::failure_response::FailureResponse;
use crate::domain::{
    models::market::MarketId,
    services::{AccessTokenStore, MarketStore, StoreFactory, UserStore},
};

use rouille::{router, Request, Response};

#[derive(Debug, Clone)]
pub struct ApiServer<F> {
    store_factory: F,
}

impl<F> ApiServer<F> {
    pub fn new(store_factory: F) -> ApiServer<F> {
        ApiServer { store_factory }
    }

    pub fn run<A, S>(self, addr: A)
    where
        A: std::net::ToSocketAddrs,
        F: StoreFactory<S> + Send + Sync + 'static,
        S: AccessTokenStore + MarketStore + UserStore + Send + 'static,
    {
        rouille::Server::new(addr, move |req| {
            rouille::log(req, ::std::io::stdout(), || {
                routing(&self.store_factory, req)
            })
        })
        .unwrap()
        .run();
    }
}

pub fn routing<F, S>(store_factory: &F, req: &Request) -> Response
where
    F: StoreFactory<S> + Send + Sync + 'static,
    S: AccessTokenStore + MarketStore + UserStore + Send + 'static,
{
    let res = router!(req,
        (POST) (/access_token) => {
            access_token::create(store_factory.establish(), req)
        },
        (GET) (/me) => {
            me::get(store_factory.establish(), req)
        },
        (GET) (/me/markets) => {
            me::markets::get(store_factory.establish(), req)
        },
        (POST) (/markets) => {
            markets::post(store_factory.establish(), req)
        },
        (GET) (/markets/{id: MarketId}) => {
            markets::get(store_factory.establish(), req, id)
        },
        (PUT) (/markets/{id: MarketId}) => {
            markets::put(store_factory.establish(), req, id)
        },
        (GET) (/markets/{id: MarketId}/orders) => {
            markets::orders::get_all(store_factory.establish(), req, id)
        },
        (POST) (/markets/{id: MarketId}/orders) => {
            markets::orders::post(store_factory.establish(), req, id)
        },
        (GET) (/cronjob/open_market) => {
            cronjob::open_market::get(store_factory.establish(), req)
        },
        (GET) (/cronjob/close_market) => {
            cronjob::close_market::get(store_factory.establish(), req)
        },
        _ => Err(FailureResponse::ResourceNotFound)
    );
    res.unwrap_or_else(<FailureResponse as Into<Response>>::into)
}
