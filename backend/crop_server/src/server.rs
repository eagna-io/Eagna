use crate::{context::Context, routes};
use futures::{
    future::{self, FutureExt as _},
    stream::StreamExt as _,
};
use std::{convert::Infallible, net::SocketAddr};
use tokio::time::{interval, Duration};
use warp::{filters::log::log, Filter as _};

pub struct Server {}

impl Server {
    pub async fn bind(socket: impl Into<SocketAddr> + 'static, ctx: Context) {
        let filter = routes::filter(ctx.clone()).with(log("crop_server"));
        let svc = warp_json_rpc::service(filter);
        let make_svc =
            hyper::service::make_service_fn(move |_| future::ok::<_, Infallible>(svc.clone()));

        hyper::Server::bind(&socket.into()).serve(make_svc).await;
    }
}
