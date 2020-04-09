use crop_domain::{
    account::model::AccountName,
    contest::poll::model::{ChoiceName, Comment, Poll},
};
use crop_primitive::string::String;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, convert::TryFrom};
use warp::filters::ws::Message;

// Currently, there are no `IncomingMsg`.

/*
 * ============
 * OutgoingMsg
 * ============
 */
#[derive(Serialize, Clone, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum OutgoingMsg<'a> {
    Comment(Cow<'a, Comment>),
    Poll(Cow<'a, Poll>),
}

impl<'a> Into<Message> for OutgoingMsg<'a> {
    fn into(self) -> Message {
        Message::text(serde_json::to_string(&self).unwrap())
    }
}

impl<'a> From<&'a Comment> for OutgoingMsg<'a> {
    fn from(comment: &'a Comment) -> Self {
        OutgoingMsg::Comment(Cow::Borrowed(comment))
    }
}

impl<'a> From<&'a Poll> for OutgoingMsg<'a> {
    fn from(poll: &'a Poll) -> Self {
        OutgoingMsg::Poll(Cow::Borrowed(poll))
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
