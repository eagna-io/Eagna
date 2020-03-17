use crop_domain::{
    account::model::AccountName,
    contest::model::Contest,
    poll::model::{Comment, Poll},
};
use std::{ops::DerefMut, sync::Arc};
use tokio::sync::{
    broadcast::{channel, Receiver, Sender},
    Mutex,
};
use warp::filters::ws::Message;

use crate::routes::ws::msg::OutgoingMsg;

const MSG_CAPACITY: usize = 100;

/// 特定の1つのコンテストを管理する
#[derive(Clone)]
pub struct ContestManager {
    contest: Arc<Mutex<Contest>>,
    msg_sink: Sender<Message>,
}

impl ContestManager {
    pub fn new(contest: Contest) -> ContestManager {
        let (sender, _) = channel(MSG_CAPACITY);
        ContestManager {
            contest: Arc::new(Mutex::new(contest)),
            msg_sink: sender,
        }
    }

    pub async fn with_contest<F, T>(&self, f: F) -> T
    where
        F: for<'a> FnOnce(&'a mut Contest) -> T,
    {
        f(self.contest.lock().await.deref_mut())
    }

    pub fn subscribe(&self) -> Receiver<Message> {
        self.msg_sink.subscribe()
    }

    pub async fn add_poll_and_broadcast(&self, poll: Poll) {
        let mut lock = self.contest.lock().await;

        let poll = lock.add_poll(poll);
        let msg = OutgoingMsg::from(poll).into();

        let _ = self.msg_sink.send(msg);

        // 送信順序担保のため、sendが終わってからlockを解放する
        drop(lock)
    }

    pub async fn comment_and_broadcast(
        &self,
        account: AccountName,
        comment: String,
    ) -> Option<Comment> {
        let mut lock = self.contest.lock().await;
        if let Some(poll) = lock.current_poll_mut() {
            let comment = poll.add_comment(account, comment);
            let msg = OutgoingMsg::from(comment).into();

            // Commentをbroadcastする。
            // receiverがいなくてもエラーにしない。
            let _ = self.msg_sink.send(msg);

            let ret = comment.clone();

            // 送信順序担保のため、sendが終わってからlockを解放する
            drop(lock);

            Some(ret)
        } else {
            None
        }
    }
}
