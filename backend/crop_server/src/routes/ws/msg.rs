use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;
use warp::filters::ws::Message;

/*
 * ============
 * IncomingMsg
 * ============
 */
#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum IncomingMsg {
    Vote(VoteMsg),
}

impl<'a> TryFrom<&'a Message> for IncomingMsg {
    type Error = serde_json::error::Error;

    fn try_from(msg: &'a Message) -> Result<Self, Self::Error> {
        serde_json::from_slice(msg.as_bytes())
    }
}

/// ## Message format
///
/// ```json
/// {
///     "type": "vote",
///     "outcomeId": "4ef1a321-61bd-4c56-84c1-ddb327d38b91",
///     "accountName": "Atsuking"
/// }
/// ```
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VoteMsg {
    pub outcome_id: Uuid,
    pub account_id: Uuid,
}

/*
 * ============
 * OutgoingMsg
 * ============
 */
#[derive(Serialize, Clone)]
#[serde(tag = "type")]
pub enum OutgoingMsg {
    Feed(FeedMsg),
}

impl Into<Message> for OutgoingMsg {
    fn into(self) -> Message {
        Message::text(serde_json::to_string(&self).unwrap())
    }
}

/// ## Message format
///
/// ```json
/// {
///     "type": "feed",
///     "outcomeId": "4ef1a321-61bd-4c56-84c1-ddb327d38b91",
///     "accountName": "Atsuking",
///     "timestamp": 1583316553000
/// }
/// ```
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeedMsg {
    pub outcome_id: Uuid,
    pub account_id: Uuid,
    /// Unixタイムスタンプのms表現
    /// https://docs.rs/chrono/0.4.10/chrono/struct.DateTime.html#method.timestamp_millis
    pub timestamp: i64,
}

impl Into<Message> for FeedMsg {
    fn into(self) -> Message {
        OutgoingMsg::Feed(self).into()
    }
}
