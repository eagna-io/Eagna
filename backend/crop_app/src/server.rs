use crate::ws;

pub struct Server {}

impl Server {
    #[tokio::main]
    pub async fn serve() {
        warp::serve(ws::filter()).bind(([127, 0, 0, 1], 3030)).await
    }
}
