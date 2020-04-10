use crop_domain::admin::model::{AccessToken, AuthenticatedAdmin};
use std::convert::Infallible;
use warp::{filters, reply, Filter};

pub fn admin(
) -> impl Filter<Extract = (Result<AuthenticatedAdmin, impl reply::Reply>,), Error = Infallible> {
    filters::header::header::<AccessToken>("Authorization")
        .map(|token| Ok(AuthenticatedAdmin::from(token)))
        .recover(|e| {
            log::debug!("{:?}", e);
            futures::future::ok(Err(reply::json(&"Unauthenticated")))
        })
        .unify()
}
