use futures::{future, stream::StreamExt as _};
use uuid::Uuid;
use warp::{filters, reject::Rejection, reply::Reply, Filter};

pub fn filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    filters::path::path("stream")
        .and(filters::path::param::<Uuid>())
        .and(filters::path::end())
        .and(filters::ws::ws())
        .map(upgrade_ws)
}

fn upgrade_ws(market_id: Uuid, ws: filters::ws::Ws) -> impl Reply {
    ws.on_upgrade(move |ws| handle_ws(market_id, ws))
}

async fn handle_ws(market_id: Uuid, ws: filters::ws::WebSocket) {
    ws.for_each(|msg| {
        dbg!(msg);
        future::ready(())
    })
    .await
}
