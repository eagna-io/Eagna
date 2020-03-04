use crate::ws;
use std::net::SocketAddr;

pub struct Server {}

impl Server {
    pub async fn bind(socket: impl Into<SocketAddr> + 'static) {
        warp::serve(ws::filter()).bind(socket).await
    }
}
