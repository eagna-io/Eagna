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
    services::{AccessTokenStore, MarketStore, TransactionalStore, UserStore},
};

use rouille::{router, Request, Response};

#[derive(Debug, Clone)]
pub struct ApiServer<S> {
    store: S,
}

impl<S> ApiServer<S> {
    pub fn new(store: S) -> ApiServer<S> {
        ApiServer { store }
    }

    pub fn run<A>(self, addr: A)
    where
        A: std::net::ToSocketAddrs,
        S: TransactionalStore
            + AccessTokenStore
            + MarketStore
            + UserStore
            + Send
            + Sync
            + 'static,
    {
        rouille::Server::new(addr, move |req| {
            rouille::log(req, ::std::io::stdout(), || self.routing(req))
        })
        .unwrap()
        .run();
    }

    fn routing(&self, req: &Request) -> Response
    where
        S: TransactionalStore
            + AccessTokenStore
            + MarketStore
            + UserStore
            + Send
            + Sync
            + 'static,
    {
        let res = self.store.transaction(|| {
            router!(req,
                (POST) (/access_token) => {
                    access_token::create(&self.store, req)
                },
                (GET) (/me) => {
                    me::get(&self.store, req)
                },
                (GET) (/me/markets) => {
                    me::markets::get(&self.store, req)
                },
                (GET) (/markets/{id: MarketId}) => {
                    markets::get(&self.store, req, id)
                },
                (GET) (/markets/{id: MarketId}/orders) => {
                    markets::orders::get_all(&self.store, req, id)
                },
                (POST) (/markets/{id: MarketId}/orders) => {
                    markets::orders::post(&self.store, req, id)
                },
                (GET) (/cronjob/open_market) => {
                    cronjob::open_market::get(&self.store, req)
                },
                _ => Err(FailureResponse::ResourceNotFound)
            )
        });
        res.unwrap_or_else(<FailureResponse as Into<Response>>::into)
    }
}
