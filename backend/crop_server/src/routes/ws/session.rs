use crate::context::contest::ContestManager;
use crate::routes::ws::msg::{IncomingMsg, OutgoingMsg};
use futures::{
    future,
    sink::{Sink, SinkExt as _},
    stream::{self, Stream, StreamExt as _, TryStreamExt as _},
};
use std::convert::TryFrom;
use tokio::sync::broadcast::Receiver;
use warp::filters::ws::{Message, WebSocket};

pub struct Session<ST, SI> {
    contest: ContestManager,
    ws_stream: ST,
    ws_sink: SI,
}

pub fn new(
    contest: ContestManager,
    ws: WebSocket,
) -> Session<
    impl Stream<Item = anyhow::Result<IncomingMsg>> + Send + Unpin + 'static,
    impl Sink<Message, Error = anyhow::Error> + Send + Unpin + 'static,
> {
    let (sink, stream) = ws.split();

    let ws_stream = stream
        .err_into::<anyhow::Error>()
        .and_then(|msg| future::ready(IncomingMsg::try_from(msg)));

    let ws_sink = sink.sink_err_into::<anyhow::Error>();

    Session {
        contest,
        ws_stream,
        ws_sink,
    }
}

impl<ST, SI> Session<ST, SI>
where
    ST: Stream<Item = anyhow::Result<IncomingMsg>> + Send + Unpin + 'static,
    SI: Sink<Message, Error = anyhow::Error> + Send + Unpin + 'static,
{
    pub async fn handle(mut self) {
        if let Err(e) = self.initialize().await {
            log::info!("{:?}", e);
            return;
        }

        let Session {
            contest,
            ws_stream,
            ws_sink,
        } = self;
        let subscriber = contest.subscribe();

        let fut1 = handle_outgoing(ws_sink, subscriber);
        let fut2 = handle_incoming(ws_stream, contest);
        future::join(fut1, fut2).await;
    }

    async fn initialize(&mut self) -> anyhow::Result<()> {
        if let Some(msg) = self
            .contest
            .with_contest(|contest, _| {
                contest
                    .current_poll()
                    .map(|poll| OutgoingMsg::from(poll).into())
            })
            .await
        {
            self.ws_sink
                .send_all(&mut stream::once(future::ok(msg)))
                .await?;
        }
        Ok(())
    }
}

async fn handle_outgoing(
    mut sink: impl Sink<Message, Error = anyhow::Error> + Unpin,
    subscriber: Receiver<Message>,
) {
    let mut msg_stream = subscriber.err_into::<anyhow::Error>();
    sink.send_all(&mut msg_stream)
        .await
        .unwrap_or_else(|e| log::debug!("{:?}", e))
}

async fn handle_incoming(
    stream: impl Stream<Item = anyhow::Result<IncomingMsg>> + Unpin,
    contest: ContestManager,
) {
    if let Err(e) = stream
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
                .with_contest(|contest, _| {
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
