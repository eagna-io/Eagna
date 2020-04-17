use super::{Comment, CommentId};
use crate::account::AccountId;
use crate::contest::poll::ChoiceName;
use chrono::{DateTime, Utc};
use crop_infra::pg::comment::QueriedComment;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct BriefComment {
    pub(in crate::contest) id: CommentId,
    pub(in crate::contest) account_id: AccountId,
    pub(in crate::contest) choice_name: Option<ChoiceName>,
    pub(in crate::contest) created_at: DateTime<Utc>,
    pub(in crate::contest) comment: String,
}

impl Comment for BriefComment {
    fn id(&self) -> &CommentId {
        &self.id
    }

    fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    fn choice_name(&self) -> Option<&ChoiceName> {
        self.choice_name.as_ref()
    }

    fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    fn comment(&self) -> &str {
        self.comment.as_str()
    }
}

impl From<QueriedComment> for BriefComment {
    fn from(comment: QueriedComment) -> BriefComment {
        BriefComment {
            id: CommentId(comment.id),
            account_id: AccountId(comment.account_id),
            choice_name: comment.choice_name.map(ChoiceName),
            created_at: comment.created_at,
            comment: comment.content,
        }
    }
}
