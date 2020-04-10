use crate::response::{self, Response};
use futures::future;
use http::StatusCode;
use warp::reject::{Reject, Rejection};

#[derive(Debug, Clone)]
pub struct Error {
    pub status: StatusCode,
    pub msg: &'static str,
}

impl Error {
    pub fn new(status: StatusCode, msg: &'static str) -> Self {
        Error { status, msg }
    }

    pub async fn recover(reject: Rejection) -> Result<Response, Rejection> {
        match reject.find::<Error>() {
            Some(e) => future::ok(response::new(e.status, &e.msg)),
            None => future::err(reject),
        }
        .await
    }
}

impl Reject for Error {}

impl Into<Rejection> for Error {
    fn into(self) -> Rejection {
        warp::reject::custom(self)
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Error {
        log::error!("Internal Server Error : {:?}", e);
        Error::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    }
}
