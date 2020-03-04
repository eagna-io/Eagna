use crate::ws::msg::FeedMsg;
use crop_domain::{
    account::model::AccountId,
    market::model::{Market, OutcomeId},
};
use std::sync::Arc;
use tokio::sync::{
    broadcast::{channel, Receiver, Sender},
    Mutex,
};

const MSG_CAPACITY: usize = 100;

#[derive(Clone)]
pub struct State {
    market: Arc<Mutex<Market>>,
    feed_sink: Sender<FeedMsg>,
}

impl State {
    pub fn new(market: Market) -> State {
        let (sender, _) = channel(MSG_CAPACITY);
        State {
            market: Arc::new(Mutex::new(market)),
            feed_sink: sender,
        }
    }

    pub fn subscribe(&self) -> Receiver<FeedMsg> {
        self.feed_sink.subscribe()
    }

    pub async fn vote(&self, account_id: AccountId, outcome_id: OutcomeId) {
        let mut lock = self.market.lock().await;
        let order = lock.vote(account_id, outcome_id);

        let msg = FeedMsg {
            outcome_id: outcome_id.0,
            account_id: account_id.0,
            timestamp: order.time.timestamp_millis(),
        };

        // FeedMsgをbroadcastする。
        // receiverがいなくてもエラーにしない。
        let _ = self.feed_sink.send(msg);

        // channelに送信する順序を担保するため、
        // 送信が終わってからlockを解放する
        drop(lock);
    }
}
