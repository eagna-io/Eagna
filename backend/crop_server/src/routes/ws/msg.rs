use crop_domain::{
    account::model::AccountName,
    market::{model::Outcome, order::model::Order},
};
use schemars::JsonSchema;
use serde::Serialize;
use warp::filters::ws::Message;

// Currently, there are no `IncomingMsg`.

/*
 * ============
 * OutgoingMsg
 * ============
 */
#[derive(Serialize, Clone, JsonSchema)]
#[serde(tag = "type")]
pub enum OutgoingMsg {
    Feed(FeedMsg),
}

impl Into<Message> for OutgoingMsg {
    fn into(self) -> Message {
        Message::text(serde_json::to_string(&self).unwrap())
    }
}

/// ```json
/// {
///     "type": "feed",
///     "outcomeId": "4ef1a321-61bd-4c56-84c1-ddb327d38b91",
///     "accountName": "Atsuking",
///     "timestamp": 1583316553000
/// }
/// ```
#[derive(Serialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedMsg {
    pub outcome: Outcome,
    pub account_name: AccountName,
    /// Unixタイムスタンプのms表現
    /// https://docs.rs/chrono/0.4.10/chrono/struct.DateTime.html#method.timestamp_millis
    pub timestamp: i64,
}

impl Into<Message> for FeedMsg {
    fn into(self) -> Message {
        OutgoingMsg::Feed(self).into()
    }
}

impl From<Order> for FeedMsg {
    fn from(order: Order) -> FeedMsg {
        FeedMsg {
            outcome: order.outcome,
            account_name: order.account_name,
            timestamp: order.time.timestamp_millis(),
        }
    }
}
