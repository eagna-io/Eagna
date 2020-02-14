use crate::market::num::{AmountCoin, AmountToken};
use crate::user::UserId;
use chrono::{DateTime, Utc};
use crop_primitive::NonEmptyString;
use getset::Getters;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub struct MarketOrders {
    /// 時系列順にソートされている必要がある
    pub(super) orders: Vec<Order>,
}

impl MarketOrders {
    pub const fn new() -> MarketOrders {
        MarketOrders { orders: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    pub fn last_order(&self) -> Option<&Order> {
        self.iter().next_back()
    }

    /// Order を履歴に追加する。
    /// それが適切なものかどうかはチェックしない
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, order: Order) -> MarketOrders {
        self.orders.push(order);
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = &Order> + DoubleEndedIterator {
        self.orders.iter()
    }

    pub fn iter_related_to_user<'a>(
        &'a self,
        user_id: &'a UserId,
    ) -> impl Iterator<Item = &'a Order> {
        self.iter().filter(move |o| o.user_id() == user_id)
    }

    pub fn compute_balance_of_user_token(
        &self,
        user_id: &UserId,
        token_name: &NonEmptyString,
    ) -> AmountToken {
        self.iter_related_to_user(user_id)
            .filter(|o| o.token_name() == token_name)
            .map(|order| order.amount_token())
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, From, Getters)]
#[get = "pub"]
pub struct Order {
    id: OrderId,
    user_id: UserId,
    token_name: NonEmptyString,
    amount_token: AmountToken,
    amount_coin: AmountCoin,
    time: DateTime<Utc>,
}

impl Order {
    pub fn new(
        user_id: UserId,
        token_name: NonEmptyString,
        amount_token: AmountToken,
        amount_coin: AmountCoin,
    ) -> Order {
        Order {
            id: OrderId::new(),
            user_id,
            token_name,
            amount_token,
            amount_coin,
            time: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub struct OrderId(Uuid);

impl OrderId {
    fn new() -> OrderId {
        OrderId(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}
