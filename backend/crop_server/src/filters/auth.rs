use crate::error::Error;
use crop_domain::{
    account,
    admin::model::{AccessToken, AuthenticatedAdmin},
};
use http::StatusCode;
use std::str::FromStr;
use warp::{
    filters::{header::header, BoxedFilter},
    reject::Rejection,
    Filter,
};

pub fn admin() -> BoxedFilter<(AuthenticatedAdmin,)> {
    (header::<BearerToken<AccessToken>>("Authorization")
        .or(header::<BearerToken<AccessToken>>("authorization"))
        .unify())
    .map(|BearerToken(token)| AuthenticatedAdmin::from(token))
    .or_else(|r| {
        log::debug!("Admin authentication is rejected : {:?}", r);
        let err = Error::new(StatusCode::UNAUTHORIZED, "Unauthenticated");
        futures::future::err(Into::<Rejection>::into(err))
    })
    .boxed()
}

pub fn account() -> BoxedFilter<(account::Authenticated,)> {
    // TODO : case insensitive
    (header::<BearerToken<account::AccessToken>>("Authorization")
        .or(header::<BearerToken<account::AccessToken>>("authorization"))
        .unify())
    .map(|BearerToken(token)| account::Authenticated::from(token))
    .or_else(|r| {
        log::debug!("Account authentication is rejected : {:?}", r);
        let err = Error::new(StatusCode::UNAUTHORIZED, "Unauthenticated");
        futures::future::err(Into::<Rejection>::into(err))
    })
    .boxed()
}

#[derive(Clone, Copy, Debug)]
pub struct BearerToken<T>(T);

impl<T> FromStr for BearerToken<T>
where
    T: FromStr<Err = anyhow::Error>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (bearer, token) = s.split_at(7);
        if bearer != "Bearer " {
            return Err(anyhow::anyhow!("Not a Bearer token"));
        }
        Ok(BearerToken(T::from_str(token)?))
    }
}
