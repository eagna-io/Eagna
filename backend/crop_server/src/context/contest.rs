use crate::routes::ws::contests::_id::OutgoingMsg;
use crop_domain::contest::ContestId;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};
use warp::filters::ws::Message;

#[derive(Clone)]
pub struct ContestManager {
    senders: Arc<RwLock<HashMap<ContestId, broadcast::Sender<Message>>>>,
}

impl ContestManager {
    pub fn new() -> ContestManager {
        ContestManager {
            senders: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn enable_subscribe(&self, contest_id: ContestId) {
        let (sender, _) = broadcast::channel(1);
        self.senders.write().await.insert(contest_id, sender);
    }

    pub async fn disable_subscribe(&self, contest_id: ContestId) {
        self.senders.write().await.remove(&contest_id);
    }

    pub async fn notify_update<'a, U: 'a>(&self, contest_id: ContestId, update: U)
    where
        OutgoingMsg<'a>: From<U>,
    {
        let msg = OutgoingMsg::from(update).into();
        self.broadcast_msg(contest_id, msg).await
    }

    pub async fn broadcast_msg(&self, contest_id: ContestId, msg: Message) {
        if let Some(sender) = self.senders.read().await.get(&contest_id) {
            // receiver がいないことによるエラーは無視
            let _ = sender.send(msg);
        }
    }

    pub async fn subscribe(&self, contest_id: &ContestId) -> Option<broadcast::Receiver<Message>> {
        self.senders
            .read()
            .await
            .get(contest_id)
            .map(|tx| tx.subscribe())
    }
}
