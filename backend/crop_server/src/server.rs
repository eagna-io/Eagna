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

        let fut1 = hyper::Server::bind(&socket.into()).serve(make_svc).boxed();
        let fut2 = Server::start_cron(ctx).boxed();

        if let future::Either::Left((Err(e), _)) = future::select(fut1, fut2).await {
            log::error!("{:?}", e);
        }
    }

    async fn start_cron(ctx: Context) {
        let contest = ctx.contest_manager();
        interval(Duration::from_secs(5))
            .for_each(|_| contest.close_and_broadcast_or_ignore())
            .await;
    }
}
