use crate::routes::ws::msg::FeedMsg;
use crate::state::market::MarketManager;
use crop_domain::market::order::model::Order;
use futures::{
    sink::{Sink, SinkExt as _},
    stream::TryStreamExt as _,
};
use tokio::sync::broadcast::Receiver;
use warp::filters::ws::{Message, WebSocket};

pub struct Session {
    market: MarketManager,
    ws: WebSocket,
}

impl Session {
    pub fn new(market: MarketManager, ws: WebSocket) -> Session {
        Session { market, ws }
    }

    pub async fn handle(self) {
        let Session { market, ws } = self;
        let subscriber = market.subscribe();

        handle_outgoing(ws, subscriber).await;
    }
}

async fn handle_outgoing(
    sink: impl Sink<Message, Error = warp::Error> + Unpin,
    subscriber: Receiver<Order>,
) {
    let mut msg_stream = subscriber
        .err_into::<anyhow::Error>()
        .map_ok(|order| FeedMsg::from(order).into());
    sink.sink_err_into::<anyhow::Error>()
        .send_all(&mut msg_stream)
        .await
        .unwrap_or_else(|e| log::debug!("{:?}", e))
}
