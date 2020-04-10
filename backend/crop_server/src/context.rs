use crate::{error::Error, response::Response};
use crop_infra::pg::Pool;
use futures::future::{TryFuture, TryFutureExt};
use warp::reject::Rejection;

#[derive(Clone)]
pub struct Context {
    pub pg: Pool,
}

impl Context {
    pub fn new(pg: Pool) -> Context {
        Context { pg }
    }

    pub async fn handle_request<F, Fut>(self, func: F) -> Result<Response, Rejection>
    where
        F: FnOnce(Context) -> Fut,
        Fut: TryFuture<Ok = Response, Error = Error>,
    {
        func(self).err_into().await
    }
}
