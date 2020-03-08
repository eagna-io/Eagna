use serde::Serialize;
use uuid::Uuid;

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
