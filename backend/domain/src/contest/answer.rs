use crate::{
    account::AccountId,
    contest::{ChoiceName, PollId},
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// アカウントのPollに対する一回の回答アクションを表現するモデル
/// 同一アカウントが同一Pollに対して複数作成することもある。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    pub id: AnswerId,
    pub account_id: AccountId,
    pub poll_id: PollId,
    pub choice_name: ChoiceName,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnswerId(pub Uuid);

impl AnswerId {
    fn new() -> Self {
        AnswerId(Uuid::new_v4())
    }
}

impl Answer {
    pub(crate) fn new(account_id: &AccountId, poll_id: &PollId, choice_name: ChoiceName) -> Answer {
        Answer {
            id: AnswerId::new(),
            account_id: *account_id,
            poll_id: *poll_id,
            choice_name,
            created_at: Utc::now(),
        }
    }
}
