use super::{AmountCoin, AmountToken, TokenName};
use crate::domain::user::UserId;
use crate::infra::postgres::types::OrderType as InfraOrderType;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketOrders {
    orders: Vec<Order>,
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
        let order = Order::new_coin_supply(self.next_order_id(), user_id, amount_coin);
        self.orders.push(order);
        self.orders.last().unwrap()
    }

    /// NormalOrder を履歴に追加する。
    /// それが適切なものかどうかはチェックしない
    pub(super) fn add_normal_order(
        &mut self,
        user_id: UserId,
        token_name: TokenName,
        amount_token: AmountToken,
        amount_coin: AmountCoin,
    ) -> &Order {
        let order = Order::new_normal(
            self.next_order_id(),
            user_id,
            token_name,
            amount_token,
            amount_coin,
        );
        self.orders.push(order);
        self.orders.last().unwrap()
    }

    /// RewardOrder を履歴に追加する。
    /// それが適切なものかどうかはチェックしない
    pub(super) fn add_reward_order(
        &mut self,
        user_id: UserId,
        token_name: TokenName,
        amount_coin: AmountCoin,
    ) -> &Order {
        let order = Order::new_reward(self.next_order_id(), user_id, token_name, amount_coin);
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

    pub fn iter_related_to_user<'a>(&'a self, user_id: &'a UserId) -> impl Iterator<Item = &'a Order> {
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
        token_name: &TokenName,
    ) -> AmountToken {
        self.iter_related_to_user(user_id)
            .map(|order| order.amount_token())
            .sum()
    }

    /// 対象のUserがオーダーを出したことがあるかどうか
    pub fn is_already_supply_initial_coin_to(&self, user_id: &UserId) -> bool {
        self.iter_related_to_user(user_id).next().is_some()
    }

    pub fn compute_amount_token_of_each_user(
        &self,
        token_name: &TokenName,
    ) -> HashMap<UserId, AmountToken> {
        let mut user_token_map = HashMap::new();

        let iter = self
            .iter()
            .filter(|o| o.token_name().filter(|tname| *tname == token_name).is_some());
        for order in iter {
            *user_token_map
                .entry(*order.user_id())
                .or_insert(AmountToken(0)) += order.amount_token();
        }

        return user_token_map;
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
            Order::CoinSupply(ref o) => &o.id,
            Order::Normal(ref o) => &o.id,
            Order::Reward(ref o) => &o.id,
        }
    }

    pub fn user_id(&self) -> &UserId {
        match self {
            Order::CoinSupply(ref o) => &o.user_id,
            Order::Normal(ref o) => &o.user_id,
            Order::Reward(ref o) => &o.user_id,
        }
    }

    pub fn token_name(&self) -> Option<&TokenName> {
        match self {
            Order::CoinSupply(ref o) => None,
            Order::Normal(ref o) => Some(&o.token_name),
            Order::Reward(ref o) => Some(&o.token_name),
        }
    }

    pub fn amount_token(&self) -> AmountToken {
        match self {
            Order::CoinSupply(ref o) => AmountToken(0),
            Order::Normal(ref o) => o.amount_token,
            Order::Reward(ref o) => AmountToken(0),
        }
    }

    pub fn amount_coin(&self) -> AmountCoin {
        match self {
            Order::CoinSupply(ref o) => o.amount_coin,
            Order::Normal(ref o) => o.amount_coin,
            Order::Reward(ref o) => o.amount_coin,
        }
    }

    pub fn order_type(&self) -> OrderType {
        match self {
            Order::CoinSupply(_) => OrderType::CoinSupply,
            Order::Normal(_) => OrderType::Normal,
            Order::Reward(_) => OrderType::Reward,
        }
    }

    pub fn time(&self) -> DateTime<Utc> {
        match self {
            Order::CoinSupply(ref o) => o.time,
            Order::Normal(ref o) => o.time,
            Order::Reward(ref o) => o.time,
        }
    }

    fn new_coin_supply(id: OrderId, user_id: UserId, amount_coin: AmountCoin) -> Order {
        Order::CoinSupply(CoinSupplyOrder::new(id, user_id, amount_coin))
    }

    fn new_normal(
        id: OrderId,
        user_id: UserId,
        token_name: TokenName,
        amount_token: AmountToken,
        amount_coin: AmountCoin,
    ) -> Order {
        Order::Normal(NormalOrder::new(
            id,
            user_id,
            token_name,
            amount_token,
            amount_coin,
        ))
    }

    fn new_reward(
        id: OrderId,
        user_id: UserId,
        token_name: TokenName,
        amount_coin: AmountCoin,
    ) -> Order {
        Order::Reward(RewardOrder::new(id, user_id, token_name, amount_coin))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
struct CoinSupplyOrder {
    pub id: OrderId,
    pub user_id: UserId,
    pub amount_coin: AmountCoin,
    pub time: DateTime<Utc>,
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

#[derive(Debug, Clone, PartialEq, Eq, From)]
struct NormalOrder {
    pub id: OrderId,
    pub user_id: UserId,
    pub token_name: TokenName,
    pub amount_token: AmountToken,
    pub amount_coin: AmountCoin,
    pub time: DateTime<Utc>,
}

impl NormalOrder {
    fn new(
        id: OrderId,
        user_id: UserId,
        token_name: TokenName,
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

#[derive(Debug, Clone, PartialEq, Eq, From)]
struct RewardOrder {
    pub id: OrderId,
    pub user_id: UserId,
    pub token_name: TokenName,
    pub amount_coin: AmountCoin,
    pub time: DateTime<Utc>,
}

impl RewardOrder {
    fn new(
        id: OrderId,
        user_id: UserId,
        token_name: TokenName,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderType {
    CoinSupply,
    Normal,
    Reward,
}

impl OrderType {
    pub fn as_infra(&self) -> InfraOrderType {
        match self {
            OrderType::CoinSupply => InfraOrderType::CoinSupply,
            OrderType::Normal => InfraOrderType::Normal,
            OrderType::Reward => InfraOrderType::Reward,
        }
    }
}
