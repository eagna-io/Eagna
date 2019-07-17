mod get_list;
mod post;

pub use get_list::get_list;
pub use post::post;

use crate::domain::market::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiOrderModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    token_name: Option<TokenName>,
    amount_token: AmountToken,
    amount_coin: AmountCoin,
    time: DateTime<Utc>,
    #[serde(rename = "type")]
    type_: OrderType,
}

impl From<Order> for ApiOrderModel {
    fn from(order: Order) -> ApiOrderModel {
        let flat_order = order.flatten();
        ApiOrderModel {
            token_name: flat_order.token_name,
            amount_token: flat_order.amount_token,
            amount_coin: flat_order.amount_coin,
            time: flat_order.time,
            type_: flat_order.type_,
        }
    }
}
