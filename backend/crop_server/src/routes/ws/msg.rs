use chrono::{DateTime, Utc};
use crop_domain::{
    account::model::AccountName,
    poll::model::{ChoiceColor, ChoiceName, Comment, Poll},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use warp::filters::ws::Message;

// Currently, there are no `IncomingMsg`.

/*
 * ============
 * OutgoingMsg
 * ============
 */
#[derive(Serialize, Clone, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum OutgoingMsg {
    Comment(Comment),
    Poll(PollMsg),
}

impl Into<Message> for OutgoingMsg {
    fn into(self) -> Message {
        Message::text(serde_json::to_string(&self).unwrap())
    }
}

#[derive(Serialize, Clone, JsonSchema)]
pub struct PollMsg {
    pub end_at: DateTime<Utc>,
    pub choices: Vec<(ChoiceName, ChoiceColor)>,
    pub resolved: Option<ChoiceName>,
}

impl<'a> From<&'a Poll> for PollMsg {
    fn from(poll: &Poll) -> PollMsg {
        PollMsg {
            end_at: poll.end_at,
            choices: poll
                .choices
                .iter()
                .map(|(n, c)| (n.clone(), c.clone()))
                .collect(),
            resolved: poll.resolved.clone(),
        }
    }
}

/*
 * ===============
 * IncomingMsg
 * ===============
 */
#[derive(Deserialize, Clone, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum IncomingMsg {
    UpdateChoice(UpdateChoiceMsg),
    AddComment(AddCommentMsg),
}

impl TryFrom<Message> for IncomingMsg {
    type Error = anyhow::Error;

    fn try_from(msg: Message) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice::<IncomingMsg>(msg.as_bytes())?)
    }
}

#[derive(Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChoiceMsg {
    pub account: AccountName,
    pub choice: ChoiceName,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Clone, JsonSchema)]
pub struct AddCommentMsg {
    pub account: AccountName,
    pub comment: String,
}
