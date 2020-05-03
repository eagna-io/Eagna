use crate::res::Error;
use crop_domain::{
    account::AccessToken as AccountAccessToken, admin::AccessToken as AdminAccessToken,
};
use http::StatusCode;
use std::str::FromStr;
use warp::{
    filters::{header::header, BoxedFilter},
    reject::Rejection,
    Filter,
};

pub fn admin() -> BoxedFilter<(AdminAccessToken,)> {
    (header::<BearerToken<AdminAccessToken>>("Authorization")
        .or(header::<BearerToken<AdminAccessToken>>("authorization"))
        .unify())
    .map(|BearerToken(token)| token)
    .or_else(|r| {
        log::debug!("Admin authentication is rejected : {:?}", r);
        let err = Error::new(StatusCode::UNAUTHORIZED, "Unauthenticated");
        futures::future::err(Into::<Rejection>::into(err))
    })
    .boxed()
}

pub fn account() -> BoxedFilter<(AccountAccessToken,)> {
    // TODO : case insensitive
    (header::<BearerToken<AccountAccessToken>>("Authorization")
        .or(header::<BearerToken<AccountAccessToken>>("authorization"))
        .unify())
    .map(|BearerToken(token)| token)
    .or_else(|r| {
        log::debug!("Account authentication is rejected : {:?}", r);
        let err = Error::new(StatusCode::UNAUTHORIZED, "Unauthenticated");
        futures::future::err(Into::<Rejection>::into(err))
    })
    .boxed()
}

#[derive(Clone, Copy, Debug)]
struct BearerToken<T>(T);

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
