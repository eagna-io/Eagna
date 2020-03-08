use crate::rpc;
use crate::ws;
use futures::future;
use std::{convert::Infallible, net::SocketAddr};
use warp::Filter as _;

pub struct Server {}

impl Server {
    pub async fn bind(socket: impl Into<SocketAddr> + 'static) {
        let filter = rpc::filter().or(ws::filter());
        let svc = warp_json_rpc::service(filter);
        let make_svc = hyper::service::make_service_fn(move |_| future::ok::<_, Infallible>(svc));

        hyper::Server::bind(&socket.into())
            .serve(make_svc)
            .await
            .unwrap()
    }
}
