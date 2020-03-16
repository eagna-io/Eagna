use crop_domain::{account::model::AccountName, contest::model::Contest, poll::model::Comment};
use std::{ops::DerefMut, sync::Arc};
use tokio::sync::{
    broadcast::{channel, Receiver, Sender},
    Mutex,
};

const MSG_CAPACITY: usize = 100;

/// 特定の1つのコンテストを管理する
#[derive(Clone)]
pub struct ContestManager {
    contest: Arc<Mutex<Contest>>,
    comment_sink: Sender<Comment>,
}

impl ContestManager {
    pub fn new(contest: Contest) -> ContestManager {
        let (sender, _) = channel(MSG_CAPACITY);
        ContestManager {
            contest: Arc::new(Mutex::new(contest)),
            comment_sink: sender,
        }
    }

    pub async fn with_contest<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut Contest) -> T,
    {
        f(self.contest.lock().await.deref_mut())
    }

    pub fn subscribe(&self) -> Receiver<Comment> {
        self.comment_sink.subscribe()
    }

    pub async fn comment_and_broadcast(
        &self,
        account: AccountName,
        comment: String,
    ) -> Option<Comment> {
        let mut lock = self.contest.lock().await;
        if let Some(poll) = lock.current_poll_mut() {
            let comment = poll.add_comment(account, comment);

            // Commentをbroadcastする。
            // receiverがいなくてもエラーにしない。
            let _ = self.comment_sink.send(comment.clone());

            let ret = comment.clone();

            // 送信順序担保のため、sendが終わってからlockを解放する
            drop(lock);

            Some(ret)
        } else {
            None
        }
    }
}
