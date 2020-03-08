use crate::routes::ws::msg::{FeedMsg, IncomingMsg};
use crate::state::market::MarketManager;
use crop_domain::{account::model::AccountId, market::model::OutcomeId};
use futures::{
    future,
    sink::{Sink, SinkExt as _},
    stream::{Stream, StreamExt as _, TryStreamExt as _},
};
use std::convert::TryFrom as _;
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

        let (sink, stream) = ws.split();
        let fut1 = handle_incoming(stream, market);
        let fut2 = handle_outgoing(sink, subscriber);
        future::join(fut1, fut2).await;
    }
}

async fn handle_incoming(
    stream: impl Stream<Item = Result<Message, warp::Error>>,
    market: MarketManager,
) {
    stream
        .err_into::<anyhow::Error>()
        .and_then(|msg| future::ready(IncomingMsg::try_from(&msg).map_err(anyhow::Error::from)))
        .try_for_each(move |msg| {
            let cloned_market = market.clone();
            async move {
                match msg {
                    IncomingMsg::Vote(vote) => {
                        let account_id = AccountId(vote.account_id);
                        let outcome_id = OutcomeId(vote.outcome_id);
                        cloned_market.vote(account_id, outcome_id).await;
                        Ok(())
                    }
                }
            }
        })
        .await
        .unwrap_or_else(|e| log::debug!("{:?}", e))
}

async fn handle_outgoing(
    sink: impl Sink<Message, Error = warp::Error> + Unpin,
    subscriber: Receiver<FeedMsg>,
) {
    let mut msg_stream = subscriber
        .err_into::<anyhow::Error>()
        .map_ok(|msg| msg.into());
    sink.sink_err_into::<anyhow::Error>()
        .send_all(&mut msg_stream)
        .await
        .unwrap_or_else(|e| log::debug!("{:?}", e))
}
