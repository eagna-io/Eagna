use crate::account::AccountId;
use crate::contest::poll::ChoiceName;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod brief;

pub use brief::BriefComment;

pub trait Comment {
    fn id(&self) -> &CommentId;

    fn account_id(&self) -> &AccountId;

    fn choice_name(&self) -> Option<&ChoiceName>;

    fn created_at(&self) -> &DateTime<Utc>;

    fn comment(&self) -> &str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct CommentId(pub Uuid);

impl CommentId {
    pub fn new() -> CommentId {
        CommentId(Uuid::new_v4())
    }
}
