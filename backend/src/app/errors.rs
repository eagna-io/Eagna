use http::status::StatusCode;
use std::borrow::Cow;
use thiserror::Error as StdError;
use warp::{reject::Rejection, reply::Reply};

#[derive(Debug, From, StdError)]
#[error("Error response {status} : {msg}")]
pub struct Error {
    status: StatusCode,
    msg: Cow<'static, str>,
}

impl Error {
    pub fn internal_error<E>(e: E) -> Error
    where
        E: std::fmt::Display,
    {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            msg: e.to_string().into(),
        }
    }
}

pub fn recover(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(err) = err.find_cause::<Error>() {
        Ok(warp::reply::with_status(
            err.msg.clone().into_owned(),
            err.status,
        ))
    } else {
        Err(err)
    }
}
