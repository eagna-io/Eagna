use crate::context::contest::ContestManager;
use crate::routes::ws::msg::OutgoingMsg;
use crop_domain::poll::model::Comment;
use futures::{
    sink::{Sink, SinkExt as _},
    stream::TryStreamExt as _,
};
use tokio::sync::broadcast::Receiver;
use warp::filters::ws::{Message, WebSocket};

pub struct Session {
    contest: ContestManager,
    ws: WebSocket,
}

impl Session {
    pub fn new(contest: ContestManager, ws: WebSocket) -> Session {
        Session { contest, ws }
    }

    pub async fn handle(self) {
        let Session { contest, ws } = self;
        let subscriber = contest.subscribe();

        handle_outgoing(ws, subscriber).await;
    }
}

async fn handle_outgoing(
    sink: impl Sink<Message, Error = warp::Error> + Unpin,
    subscriber: Receiver<Comment>,
) {
    let mut msg_stream = subscriber
        .err_into::<anyhow::Error>()
        .map_ok(|comment| OutgoingMsg::Comment(comment).into());
    sink.sink_err_into::<anyhow::Error>()
        .send_all(&mut msg_stream)
        .await
        .unwrap_or_else(|e| log::debug!("{:?}", e))
}
