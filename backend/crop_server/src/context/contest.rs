use crate::routes::ws::contests::_id::OutgoingMsgSource;
use crop_domain::contest::ContestId;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};

type MsgSource = Arc<dyn OutgoingMsgSource>;

#[derive(Clone)]
pub struct ContestManager {
    senders: Arc<RwLock<HashMap<ContestId, broadcast::Sender<MsgSource>>>>,
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

    pub async fn broadcast_msg<S>(&self, contest_id: ContestId, msg: S)
    where
        S: OutgoingMsgSource + 'static,
    {
        if let Some(sender) = self.senders.read().await.get(&contest_id) {
            // receiver がいないことによるエラーは無視
            let _ = sender.send(Arc::new(msg));
        }
    }

    pub async fn subscribe(
        &self,
        contest_id: &ContestId,
    ) -> Option<broadcast::Receiver<MsgSource>> {
        if let Some(sender) = self.senders.read().await.get(contest_id) {
            return Some(sender.subscribe());
        }
        let (sender, receiver) = broadcast::channel(1);
        self.senders.write().await.insert(*contest_id, sender);
        Some(receiver)
    }
}
