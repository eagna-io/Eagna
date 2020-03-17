use crop_domain::{
    account::model::AccountName,
    poll::model::{ChoiceName, Comment},
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
}

impl Into<Message> for OutgoingMsg {
    fn into(self) -> Message {
        Message::text(serde_json::to_string(&self).unwrap())
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
