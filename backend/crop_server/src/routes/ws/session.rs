use crate::context::contest::ContestManager;
use crate::routes::ws::msg::{IncomingMsg, OutgoingMsg};
use crop_domain::poll::model::Comment;
use futures::{
    future,
    sink::{Sink, SinkExt as _},
    stream::{Stream, StreamExt as _, TryStreamExt as _},
};
use std::convert::TryFrom;
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

        let (sink, stream) = ws.split();
        let fut1 = handle_outgoing(sink, subscriber);
        let fut2 = handle_incoming(stream, contest);
        future::join(fut1, fut2).await;
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

async fn handle_incoming(
    stream: impl Stream<Item = Result<Message, warp::Error>> + Unpin,
    contest: ContestManager,
) {
    if let Err(e) = stream
        .err_into::<anyhow::Error>()
        .and_then(|msg| future::ready(IncomingMsg::try_from(msg)))
        .try_for_each(move |msg| handle_each_incoming_msg(msg, contest.clone()))
        .await
    {
        log::debug!("{:?}", e);
    }
}

async fn handle_each_incoming_msg(msg: IncomingMsg, contest: ContestManager) -> anyhow::Result<()> {
    match msg {
        IncomingMsg::UpdateChoice(msg) => {
            contest
                .with_contest(|contest| {
                    if let Some(poll) = contest.current_poll_mut() {
                        Ok(poll.update_choice(msg.account, msg.choice))
                    } else {
                        Err(anyhow::anyhow!("No poll is available"))
                    }
                })
                .await
        }
        IncomingMsg::AddComment(msg) => contest
            .comment_and_broadcast(msg.account, msg.comment)
            .await
            .map(drop)
            .ok_or(anyhow::anyhow!("No poll is available")),
    }
}
