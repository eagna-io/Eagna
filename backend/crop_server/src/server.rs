use crate::routes;
use futures::future;
use std::{convert::Infallible, net::SocketAddr};

pub struct Server {}

impl Server {
    pub async fn bind(socket: impl Into<SocketAddr> + 'static) {
        let svc = warp_json_rpc::service(routes::filter());
        let make_svc = hyper::service::make_service_fn(move |_| future::ok::<_, Infallible>(svc));

        hyper::Server::bind(&socket.into())
            .serve(make_svc)
            .await
            .unwrap()
    }
}
