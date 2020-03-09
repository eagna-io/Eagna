use crate::{context::Context, routes};
use futures::future;
use std::{convert::Infallible, net::SocketAddr};

pub struct Server {}

impl Server {
    pub async fn bind(socket: impl Into<SocketAddr> + 'static, ctx: Context) {
        let svc = warp_json_rpc::service(routes::filter(ctx));
        let make_svc =
            hyper::service::make_service_fn(move |_| future::ok::<_, Infallible>(svc.clone()));

        hyper::Server::bind(&socket.into())
            .serve(make_svc)
            .await
            .unwrap()
    }
}
