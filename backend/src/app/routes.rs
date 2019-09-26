mod cronjob;
mod markets;
mod prizes;
mod users;

use crate::domain::market::MarketId;

use super::{FailureResponse, InfraManager};
use rouille::{router, Request, Response};

pub fn routing(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    router!(req,
        (POST) (/users/) => {
            users::post(infra, req)
        },
        (GET) (/users/me/) => {
            users::me(infra, req)
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
            markets::orders::get_list(infra, req, id)
        },
        (POST) (/markets/{id: MarketId}/orders/) => {
            markets::orders::post(infra, req, id)
        },
        (GET) (/prizes/) => {
            prizes::get_list(infra, req)
        },
        (POST) (/prizes/) => {
            prizes::post(infra, req)
        },
        (GET) (/cronjob/check_markets/) => {
            cronjob::check_markets::get(infra, req)
        },
        _ => Err(FailureResponse::ResourceNotFound)
    )
}
