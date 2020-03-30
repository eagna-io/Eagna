use crop_domain::{
    account::model::AccountName,
    contest::model::Contest,
    poll::model::{ChoiceName, Comment},
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
        F: for<'a> FnOnce(&'a mut Contest, &'a Sender<Message>) -> T,
    {
        f(self.contest.lock().await.deref_mut(), &self.msg_sink)
    }

    pub fn subscribe(&self) -> Receiver<Message> {
        self.msg_sink.subscribe()
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

    pub async fn close_and_broadcast_or_ignore(&self) {
        let mut lock = self.contest.lock().await;
        if let Some(poll) = lock.current_poll_mut() {
            if poll.close_or_ignore() {
                let msg = OutgoingMsg::from(&*poll).into();
                let _ = self.msg_sink.send(msg);
            }
        }
        drop(lock);
    }

    pub async fn resolve_and_broadcast(&self, choice: ChoiceName) -> anyhow::Result<()> {
        let mut lock = self.contest.lock().await;
        if let Some(poll) = lock.current_poll_mut() {
            poll.resolve(choice)?;
            let msg = OutgoingMsg::from(&*poll).into();
            let _ = self.msg_sink.send(msg);
        }
        drop(lock);
        Ok(())
    }
}
