use super::{AmountCoin, AmountToken};
use crate::domain::user::UserId;
use crate::primitive::NonEmptyString;
use chrono::{DateTime, Utc};
use getset::Getters;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketOrders {
    /// 時系列順にソートされている必要がある
    pub(super) orders: Vec<Order>,
}

impl MarketOrders {
    pub(super) fn new() -> MarketOrders {
        MarketOrders { orders: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    pub fn last_order(&self) -> Option<&Order> {
        self.iter().next_back()
    }

    /// CoinSupplyOrder を履歴に追加する。
    /// それが適切なものかどうかはチェックしない
    pub(super) fn add_coin_supply_order(
        &mut self,
        user_id: UserId,
        amount_coin: AmountCoin,
    ) -> &Order {
        let order = Order::from(CoinSupplyOrder::new(
            self.next_order_id(),
            user_id,
            amount_coin,
        ));
        self.orders.push(order);
        self.orders.last().unwrap()
    }

    /// NormalOrder を履歴に追加する。
    /// それが適切なものかどうかはチェックしない
    pub(super) fn add_normal_order(
        &mut self,
        user_id: UserId,
        token_name: NonEmptyString,
        amount_token: AmountToken,
        amount_coin: AmountCoin,
    ) -> &Order {
        let order = Order::from(NormalOrder::new(
            self.next_order_id(),
            user_id,
            token_name,
            amount_token,
            amount_coin,
        ));
        self.orders.push(order);
        self.orders.last().unwrap()
    }

    /// RewardOrder を履歴に追加する。
    /// それが適切なものかどうかはチェックしない
    pub(super) fn add_reward_order(
        &mut self,
        user_id: UserId,
        token_name: NonEmptyString,
        amount_coin: AmountCoin,
    ) -> &Order {
        let order = Order::from(RewardOrder::new(
            self.next_order_id(),
            user_id,
            token_name,
            amount_coin,
        ));
        self.orders.push(order);
        self.orders.last().unwrap()
    }

    fn next_order_id(&self) -> OrderId {
        self.iter()
            .next_back()
            .map(|o| o.id().next_id())
            .unwrap_or(OrderId::first())
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

    pub fn compute_balance_of_user_coin(&self, user_id: &UserId) -> AmountCoin {
        self.iter_related_to_user(user_id)
            .map(|order| order.amount_coin())
            .sum()
    }

    pub fn compute_balance_of_user_token(
        &self,
        user_id: &UserId,
        token_name: &NonEmptyString,
    ) -> AmountToken {
        self.iter_related_to_user(user_id)
            .filter(|o| o.token_name() == Some(token_name))
            .map(|order| order.amount_token())
            .sum()
    }

    /// 対象のUserがオーダーを出したことがあるかどうか
    pub fn is_already_supply_initial_coin_to(&self, user_id: &UserId) -> bool {
        self.iter_related_to_user(user_id).next().is_some()
    }

    pub fn filter_normal_orders(&self) -> impl Iterator<Item = &NormalOrder> {
        self.iter().filter_map(|o| match o {
            Order::CoinSupply(_) => None,
            Order::Normal(ref n) => Some(n),
            Order::Reward(_) => None,
        })
    }

    pub fn filter_reward_orders(&self) -> impl Iterator<Item = &RewardOrder> {
        self.iter().filter_map(|o| match o {
            Order::CoinSupply(_) => None,
            Order::Normal(_) => None,
            Order::Reward(ref r) => Some(r),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum Order {
    CoinSupply(CoinSupplyOrder),
    Normal(NormalOrder),
    Reward(RewardOrder),
}

impl Order {
    pub fn id(&self) -> &OrderId {
        match self {
            Order::CoinSupply(ref inner) => inner.id(),
            Order::Normal(ref inner) => inner.id(),
            Order::Reward(ref inner) => inner.id(),
        }
    }

    pub fn user_id(&self) -> &UserId {
        match self {
            Order::CoinSupply(ref inner) => inner.user_id(),
            Order::Normal(ref inner) => inner.user_id(),
            Order::Reward(ref inner) => inner.user_id(),
        }
    }

    pub fn token_name(&self) -> Option<&NonEmptyString> {
        match self {
            Order::CoinSupply(_) => None,
            Order::Normal(ref inner) => Some(inner.token_name()),
            Order::Reward(_) => None,
        }
    }

    pub fn amount_token(&self) -> AmountToken {
        match self {
            Order::CoinSupply(_) => AmountToken::zero(),
            Order::Normal(ref inner) => *inner.amount_token(),
            Order::Reward(_) => AmountToken::zero(),
        }
    }

    pub fn amount_coin(&self) -> &AmountCoin {
        match self {
            Order::CoinSupply(ref inner) => inner.amount_coin(),
            Order::Normal(ref inner) => inner.amount_coin(),
            Order::Reward(ref inner) => inner.amount_coin(),
        }
    }

    pub fn type_(&self) -> OrderType {
        match self {
            Order::CoinSupply(_) => OrderType::CoinSupply,
            Order::Normal(_) => OrderType::Normal,
            Order::Reward(_) => OrderType::Reward,
        }
    }

    pub fn time(&self) -> &DateTime<Utc> {
        match self {
            Order::CoinSupply(ref inner) => inner.time(),
            Order::Normal(ref inner) => inner.time(),
            Order::Reward(ref inner) => inner.time(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, From, Getters)]
#[get = "pub"]
pub struct CoinSupplyOrder {
    id: OrderId,
    user_id: UserId,
    amount_coin: AmountCoin,
    time: DateTime<Utc>,
}

impl CoinSupplyOrder {
    fn new(id: OrderId, user_id: UserId, amount_coin: AmountCoin) -> CoinSupplyOrder {
        CoinSupplyOrder {
            id,
            user_id,
            amount_coin,
            time: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, From, Getters)]
#[get = "pub"]
pub struct NormalOrder {
    id: OrderId,
    user_id: UserId,
    token_name: NonEmptyString,
    amount_token: AmountToken,
    amount_coin: AmountCoin,
    time: DateTime<Utc>,
}

impl NormalOrder {
    fn new(
        id: OrderId,
        user_id: UserId,
        token_name: NonEmptyString,
        amount_token: AmountToken,
        amount_coin: AmountCoin,
    ) -> NormalOrder {
        NormalOrder {
            id,
            user_id,
            token_name,
            amount_token,
            amount_coin,
            time: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, From, Getters)]
#[get = "pub"]
pub struct RewardOrder {
    id: OrderId,
    user_id: UserId,
    token_name: NonEmptyString,
    amount_coin: AmountCoin,
    time: DateTime<Utc>,
}

impl RewardOrder {
    fn new(
        id: OrderId,
        user_id: UserId,
        token_name: NonEmptyString,
        amount_coin: AmountCoin,
    ) -> RewardOrder {
        RewardOrder {
            id,
            user_id,
            token_name,
            amount_coin,
            time: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub struct OrderId(i32);

impl OrderId {
    fn first() -> OrderId {
        OrderId(0)
    }

    fn next_id(&self) -> OrderId {
        OrderId(self.0 + 1)
    }

    pub fn as_i32(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderType {
    CoinSupply,
    Normal,
    Reward,
}
