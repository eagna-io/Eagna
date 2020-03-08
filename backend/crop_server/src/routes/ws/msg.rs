use crop_domain::market::order::model::Order;
use serde::Serialize;
use uuid::Uuid;
use warp::filters::ws::Message;

// Currently, there are no `IncomingMsg`.

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

impl From<Order> for FeedMsg {
    fn from(order: Order) -> FeedMsg {
        FeedMsg {
            outcome_id: order.outcome_id.0,
            account_id: order.account_id.0,
            timestamp: order.time.timestamp_millis(),
        }
    }
}
