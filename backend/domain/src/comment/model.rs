use crate::account::AccountId;
use crate::contest::{ChoiceName, Contest, ContestId, ContestStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    contest_id: ContestId,
    account_id: AccountId,
    comment: String,
    choice_name: Option<ChoiceName>,
}

impl Comment {
    pub fn new(
        contest: &Contest,
        account_id: AccountId,
        comment: String,
    ) -> anyhow::Result<Comment> {
        if contest.status == ContestStatus::Archived {
            return Err(anyhow::anyhow!("Contest status is archived"));
        }

        let choice_name = contest
            .current_poll
            .as_ref()
            .and_then(|poll| poll.account_choices.get(&account_id))
            .cloned();

        Ok(Comment {
            contest_id: contest.id,
            account_id,
            comment,
            choice_name,
        })
    }
}
