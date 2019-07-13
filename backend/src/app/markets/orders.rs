mod get_all;
mod post;

pub use get_all::get_all;
pub use post::post;

use crate::domain::market::*;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiOrderModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    token_name: Option<TokenName>,
    amount_token: AmountToken,
    amount_coin: AmountCoin,
    time: DateTime<Utc>,
    #[serde(reaname = "type")]
    type_: OrderType,
}

impl From<Order> for ApiOrderModel {
    fn from(order: Order) -> ApiOrderModel {
        ApiOrderModel {
            token_name: order.token_name(),
            amount_token: order.amount_token(),
            amount_coin: order.amount_coin(),
            time: order.time(),
            type_: order.order_type(),
        }
    }
}
