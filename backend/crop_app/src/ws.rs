use futures::{future, stream::StreamExt as _};
use warp::{filters, reject::Rejection, reply::Reply, Filter};

pub fn filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    filters::path::path("stream")
        .and(filters::path::end())
        .and(filters::ws::ws())
        .map(|ws: filters::ws::Ws| ws.on_upgrade(handle_ws))
}

async fn handle_ws(ws: filters::ws::WebSocket) {
    ws.for_each(|msg| {
        dbg!(msg);
        future::ready(())
    })
    .await
}
