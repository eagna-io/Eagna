use crop_domain::market::order::model::Order;
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
    #[serde(rename = "order")]
    Order(Order),
}

impl Into<Message> for OutgoingMsg {
    fn into(self) -> Message {
        Message::text(serde_json::to_string(&self).unwrap())
    }
}
