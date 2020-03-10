use crop_domain::{
    account::model::AccountName,
    market::model::{Market, OutcomeId},
    market::order::model::Order,
};
use std::{ops::DerefMut, sync::Arc};
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

    pub async fn with_market<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut Market) -> T,
    {
        f(self.market.lock().await.deref_mut())
    }

    pub fn subscribe(&self) -> Receiver<Order> {
        self.feed_sink.subscribe()
    }

    pub async fn vote_and_broadcast(
        &self,
        account_name: AccountName,
        outcome_id: OutcomeId,
    ) -> Option<Order> {
        let mut lock = self.market.lock().await;
        if let Some(order) = lock.vote(account_name, outcome_id) {
            // FeedMsgをbroadcastする。
            // receiverがいなくてもエラーにしない。
            let _ = self.feed_sink.send(order.clone());

            Some(order.clone())
        } else {
            None
        }
    }
}
