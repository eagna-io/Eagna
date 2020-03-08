use crop_domain::{
    account::model::AccountId,
    market::model::{Market, OutcomeId},
    market::order::model::Order,
};
use std::sync::Arc;
use tokio::sync::{
    broadcast::{channel, Receiver, Sender},
    Mutex,
};

const MSG_CAPACITY: usize = 100;

/// 特定の1つのマーケットを管理する
#[derive(Clone)]
pub struct MarketManager {
    market: Arc<Mutex<Market>>,
    feed_sink: Sender<Order>,
}

impl MarketManager {
    pub fn new(market: Market) -> MarketManager {
        let (sender, _) = channel(MSG_CAPACITY);
        MarketManager {
            market: Arc::new(Mutex::new(market)),
            feed_sink: sender,
        }
    }

    pub fn subscribe(&self) -> Receiver<Order> {
        self.feed_sink.subscribe()
    }

    pub async fn vote_and_broadcast(&self, account_id: AccountId, outcome_id: OutcomeId) -> Order {
        let mut lock = self.market.lock().await;
        let order = lock.vote(account_id, outcome_id);

        // FeedMsgをbroadcastする。
        // receiverがいなくてもエラーにしない。
        let _ = self.feed_sink.send(order);

        // channelに送信する順序を担保するため、
        // 送信が終わってからlockを解放する
        drop(lock);

        order
    }
}
