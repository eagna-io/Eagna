use crate::domain::models::{
    market::TokenId,
    num::{AmountCoin, AmountToken},
    user::UserId,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct MarketOrders {
    pub orders: Vec<Order>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Normal(NormalOrder),
    InitialSupply(InitialSupplyOrder),
    Settle(SettleOrder),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NormalOrder {
    pub user_id: UserId,
    pub token_id: TokenId,
    pub amount_token: AmountToken,
    pub amount_coin: AmountCoin,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitialSupplyOrder {
    pub user_id: UserId,
    pub amount_coin: AmountCoin,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettleOrder {
    pub user_id: UserId,
    pub token_id: TokenId,
    pub amount_token: AmountToken,
    pub amount_coin: AmountCoin,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct OrderId(pub i32);

impl MarketOrders {
    /// InitialSupplyOrder で初期化。
    /// MarketOrdersは、必ず最初にInitialSupplyで初期化される。
    pub fn new_with_initial_supply_orders<I>(init_orders: I) -> MarketOrders
    where
        I: Iterator<Item = InitialSupplyOrder> + ExactSizeIterator,
    {
        let mut orders = Vec::with_capacity(init_orders.len());
        for init_order in init_orders {
            orders.push(Order::InitialSupply(init_order));
        }
        MarketOrders { orders }
    }

    pub fn iter(&self) -> impl Iterator<Item = (OrderId, &Order)> + DoubleEndedIterator {
        self.orders
            .iter()
            .enumerate()
            .map(|(idx, order)| (OrderId(idx as i32), order))
    }

    pub fn related_to_user(&self, user_id: UserId) -> impl Iterator<Item = (OrderId, &Order)> {
        self.iter().filter(move |(_i, o)| *o.user_id() == user_id)
    }

    pub fn balance_of_user_coin(&self, user_id: UserId) -> AmountCoin {
        self.related_to_user(user_id)
            .map(|(_id, order)| order.amount_coin())
            .sum()
    }

    pub fn balance_of_user_token(&self, user_id: UserId, token_id: TokenId) -> AmountToken {
        self.related_to_user(user_id)
            .filter_map(|(_id, order)| match order {
                Order::Normal(n) if n.token_id == token_id => Some(n.amount_token),
                Order::Settle(n) if n.token_id == token_id => Some(n.amount_token),
                _ => None,
            })
            .sum()
    }

    /// 末尾に新しいOrderを追加する.
    /// 呼び出し元は、追加するOrderが適切であることを保証しなければならない。
    /// 現在はMarket構造体からのみ呼び出しされる想定
    pub(super) fn push_valid_order(&mut self, order: Order) {
        self.orders.push(order);
    }

    pub fn last_normal_order(&self) -> Option<(OrderId, &NormalOrder)> {
        self.iter()
            .filter_map(|(id, order)| match order {
                Order::Normal(n) => Some((id, n)),
                _ => None,
            })
            .next_back()
    }
}

impl Order {
    pub fn user_id(&self) -> &UserId {
        match self {
            Order::Normal(o) => &o.user_id,
            Order::InitialSupply(o) => &o.user_id,
            Order::Settle(o) => &o.user_id,
        }
    }

    pub fn amount_token(&self) -> AmountToken {
        match self {
            Order::Normal(o) => o.amount_token,
            Order::InitialSupply(_) => AmountToken(0),
            Order::Settle(o) => o.amount_token,
        }
    }

    pub fn amount_coin(&self) -> AmountCoin {
        match self {
            Order::Normal(o) => o.amount_coin,
            Order::InitialSupply(o) => o.amount_coin,
            Order::Settle(o) => o.amount_coin,
        }
    }

    pub fn time(&self) -> &DateTime<Utc> {
        match self {
            Order::Normal(o) => &o.time,
            Order::InitialSupply(o) => &o.time,
            Order::Settle(o) => &o.time,
        }
    }
}

impl NormalOrder {
    pub fn new(
        user_id: UserId,
        token_id: TokenId,
        amount_token: AmountToken,
        amount_coin: AmountCoin,
    ) -> NormalOrder {
        NormalOrder {
            user_id,
            token_id,
            amount_token,
            amount_coin,
            time: Utc::now(),
        }
    }
}
