use crate::account::AccountId;
use crate::contest::{AnswerId, Contest, ContestId, ContestStatus};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    pub id: CommentId,
    pub contest_id: ContestId,
    pub account_id: AccountId,
    pub comment: String,
    /// Comment作成時のAccountの回答
    pub answer_id: Option<AnswerId>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommentId(pub Uuid);

impl CommentId {
    fn new() -> Self {
        CommentId(Uuid::new_v4())
    }
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
            id: CommentId::new(),
            contest_id: contest.id,
            account_id,
            comment,
            answer_id,
            created_at: Utc::now(),
        })
    }
}
