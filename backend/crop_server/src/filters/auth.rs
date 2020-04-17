use crate::error::Error;
use crop_domain::{
    account,
    admin::model::{AccessToken, AuthenticatedAdmin},
};
use http::StatusCode;
use warp::{filters, reject::Rejection, Filter};

pub fn admin() -> impl Filter<Extract = (AuthenticatedAdmin,), Error = Rejection> + Clone {
    filters::header::header::<AccessToken>("Authorization")
        .map(AuthenticatedAdmin::from)
        .or_else(|r| {
            log::debug!("Admin authentication is rejected : {:?}", r);
            let err = Error::new(StatusCode::UNAUTHORIZED, "Unauthenticated");
            futures::future::err(err.into())
        })
}

pub fn account() -> impl Filter<Extract = (account::Authenticated,), Error = Rejection> + Clone {
    filters::header::header::<account::AccessToken>("Authorization")
        .map(account::Authenticated::from)
        .or_else(|r| {
            log::debug!("Account authentication is rejected : {:?}", r);
            let err = Error::new(StatusCode::UNAUTHORIZED, "Unauthenticated");
            futures::future::err(err.into())
        })
}
