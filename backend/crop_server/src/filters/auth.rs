use crate::error::Error;
use crop_domain::admin::model::{AccessToken, AuthenticatedAdmin};
use http::StatusCode;
use warp::{filters, reject::Rejection, Filter};

pub fn admin() -> impl Filter<Extract = (AuthenticatedAdmin,), Error = Rejection> + Clone {
    filters::header::header::<AccessToken>("Authorization")
        .map(|token| AuthenticatedAdmin::from(token))
        .or_else(|r| {
            log::debug!("Admin authentication is rejected : {:?}", r);
            let err = Error::new(StatusCode::UNAUTHORIZED, "Unauthenticated");
            futures::future::err(err.into())
        })
}
