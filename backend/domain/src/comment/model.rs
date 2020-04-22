use crate::account::AccountId;
use crate::contest::{AnswerId, Contest, ContestId, ContestStatus};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    contest_id: ContestId,
    account_id: AccountId,
    comment: String,
    /// Comment作成時のAccountの回答
    answer_id: Option<AnswerId>,
    created_at: DateTime<Utc>,
}

impl Comment {
    /// ## Developper note
    /// トランザクションが違うので、可能性としては
    /// ContestがArchiveされた後にCommentが作成される、
    /// という状態になることもありえるが、
    /// その状態はすぐに収束する、かつ高い厳密性が求められている
    /// わけではないため、結果整合性で対応する。
    pub fn new(
        contest: &Contest,
        account_id: AccountId,
        comment: String,
    ) -> anyhow::Result<Comment> {
        if contest.status == ContestStatus::Archived {
            return Err(anyhow::anyhow!("Contest status is archived"));
        }

        let answer_id = contest
            .current_poll
            .as_ref()
            .and_then(|poll| poll.final_answers.get(&account_id))
            .map(|answer| answer.id);

        Ok(Comment {
            contest_id: contest.id,
            account_id,
            comment,
            answer_id,
            created_at: Utc::now(),
        })
    }
}
