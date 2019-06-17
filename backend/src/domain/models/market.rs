mod order;
pub use order::{
    InitialSupplyOrder, MarketOrders, NormalOrder, Order, OrderId, OrderType, SettleOrder,
};

pub const MAX_SPLIT_RATE: f64 = 0.05; // 5 %;
pub const INITIAL_SUPPLY_COIN: AmountCoin = AmountCoin(10000);

use crate::domain::models::{
    lmsr,
    num::{AmountCoin, AmountToken},
    user::UserId,
};
use chrono::{DateTime, Utc};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub enum Market {
    Preparing(PreparingMarket),
    Open(OpenMarket),
    Closed(ClosedMarket),
    Settled(SettledMarket),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparingMarket {
    pub base: BaseInfos,
}

#[derive(Debug, Clone)]
pub struct OpenMarket {
    pub base: BaseInfos,
    pub orders: MarketOrders,
}

#[derive(Debug, Clone)]
pub struct ClosedMarket {
    pub base: BaseInfos,
    pub orders: MarketOrders,
}

#[derive(Debug, Clone)]
pub struct SettledMarket {
    pub base: BaseInfos,
    pub orders: MarketOrders,
    pub settle_token: Token,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// 基本的に変わることのない情報。
pub struct BaseInfos {
    pub id: MarketId,
    pub title: MarketTitle,
    pub organizer: MarketOrganizer,
    pub short_desc: MarketShortDesc,
    pub description: MarketDesc,
    pub lmsr_b: lmsr::B,
    pub open_time: DateTime<Utc>,
    pub close_time: DateTime<Utc>,
    pub tokens: MarketTokens,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketId(pub i32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketTitle(pub Arc<String>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketOrganizer(pub Arc<String>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketShortDesc(pub Arc<String>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketDesc(pub Arc<String>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketStatus {
    Preparing,
    Open,
    Closed,
    Settled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketTokens(pub Arc<Vec<Token>>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    pub id: TokenId,
    pub name: TokenName,
    pub description: TokenDesc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenId(pub i32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenName(pub Arc<String>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenDesc(pub Arc<String>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TryOrderError {
    InsufficientBalance,
    InvalidAmountOfToken,
    PriceOutOfRange,
}

impl std::ops::Deref for Market {
    type Target = BaseInfos;
    fn deref(&self) -> &BaseInfos {
        self.base_infos()
    }
}

impl Market {
    pub fn status(&self) -> MarketStatus {
        match self {
            Market::Preparing(_) => MarketStatus::Preparing,
            Market::Open(_) => MarketStatus::Open,
            Market::Closed(_) => MarketStatus::Closed,
            Market::Settled(_) => MarketStatus::Settled,
        }
    }

    pub fn base_infos(&self) -> &BaseInfos {
        match self {
            Market::Preparing(ref m) => &m.base,
            Market::Open(ref m) => &m.base,
            Market::Closed(ref m) => &m.base,
            Market::Settled(ref m) => &m.base,
        }
    }

    pub fn orders(&self) -> Option<&MarketOrders> {
        match self {
            Market::Preparing(_) => None,
            Market::Open(ref m) => Some(&m.orders),
            Market::Closed(ref m) => Some(&m.orders),
            Market::Settled(ref m) => Some(&m.orders),
        }
    }
}

use std::str::FromStr;
impl FromStr for MarketId {
    type Err = std::num::ParseIntError;
    fn from_str(src: &str) -> Result<MarketId, Self::Err> {
        i32::from_str(src).map(MarketId)
    }
}

impl PreparingMarket {
    /// open_time がすでに過ぎているかどうか
    pub fn can_open(&self) -> bool {
        self.base.open_time < Utc::now()
    }

    /// open処理をしてOpenMarketを返す.
    ///
    /// ## Panics
    /// まだopen_timeが来ていないとき
    pub fn open_uncheck(self) -> OpenMarket {
        let now = Utc::now();
        assert!(self.base.open_time < now);
        let orders = MarketOrders::new();
        OpenMarket {
            base: self.base,
            orders,
        }
    }
}

impl OpenMarket {
    /// close_time がすでに過ぎているかどうか
    pub fn can_close(&self) -> bool {
        self.base.close_time < Utc::now()
    }

    /// close処理をして、ClosedMarketを返す
    ///
    /// ## panics
    /// まだclose_timeがきていないとき
    pub fn close_uncheck(self) -> ClosedMarket {
        assert!(self.can_close());
        ClosedMarket {
            base: self.base,
            orders: self.orders,
        }
    }

    /// ユーザーにInitialSupplyを付与する
    /// - UserはまだInitialSupplyを受け取っていないか
    /// をチェックする
    pub fn try_supply_initial_coin(&mut self, user_id: &UserId) -> Result<(), ()> {
        log::debug!("Try supply initial coin to {:?}", user_id);

        if self.orders.is_already_supply_initial_coin_to(user_id) {
            return Err(());
        }

        let supply = InitialSupplyOrder {
            user_id: *user_id,
            amount_coin: INITIAL_SUPPLY_COIN,
            time: Utc::now(),
        };

        self.orders.push_valid_order(Order::InitialSupply(supply));

        Ok(())
    }

    /// 新しいNormalOrderを追加する。
    /// - Userの残高が十分にあるか
    /// - Priceは適切に設定されているか
    /// をチェックする.
    /// チェックが通った場合にのみ、NormalOrderを追加する
    pub fn try_order(&mut self, order: NormalOrder) -> Result<(), TryOrderError> {
        log::debug!("Try a new order : {:?}", order);

        // check balance
        if order.amount_token < AmountToken(0) {
            // user SELL the token. So check balance of token.
            let token_balance = self
                .orders
                .balance_of_user_token(order.user_id, order.token_id);
            if token_balance + order.amount_token < AmountToken(0) {
                return Err(TryOrderError::InsufficientBalance);
            }
        } else if order.amount_token > AmountToken(0) {
            // user BUY the token. So check balance of coin.
            let coin_balance = self.orders.balance_of_user_coin(order.user_id);
            if coin_balance + order.amount_coin < AmountCoin(0) {
                return Err(TryOrderError::InsufficientBalance);
            }
        } else {
            return Err(TryOrderError::InvalidAmountOfToken);
        }

        // check price
        let expect_amount_coin = -self.cost_of_token(&order.token_id, order.amount_token);
        if !order
            .amount_coin
            .is_around(&expect_amount_coin, MAX_SPLIT_RATE)
        {
            return Err(TryOrderError::PriceOutOfRange);
        }

        // update data
        let new_order = NormalOrder {
            amount_coin: expect_amount_coin,
            ..order
        };
        self.orders.push_valid_order(Order::Normal(new_order));
        Ok(())
    }

    fn token_distribution(&self) -> HashMap<TokenId, AmountToken> {
        let mut distribution = HashMap::new();

        // initialize
        for token in self.base.tokens.iter() {
            distribution.insert(token.id, AmountToken(0));
        }

        // update
        for (_id, order) in self.orders.iter() {
            match order {
                Order::Normal(n) => {
                    *distribution.get_mut(&n.token_id).unwrap() += n.amount_token;
                }
                _ => {}
            }
        }

        distribution
    }

    fn cost_of_token(&self, token_id: &TokenId, amount_token: AmountToken) -> AmountCoin {
        let lmsr_b = self.base.lmsr_b;
        let mut distribution = self.token_distribution();
        let cur_cost = lmsr::cost(lmsr_b, distribution.values());
        let cur_amount_token = *distribution
            .get(token_id)
            .expect("Token MUST belongs to the market");
        distribution.insert(*token_id, cur_amount_token + amount_token);
        let new_cost = lmsr::cost(lmsr_b, distribution.values());
        new_cost - cur_cost
    }

    pub fn last_normal_order(&self) -> Option<(OrderId, &NormalOrder)> {
        self.orders.last_normal_order()
    }
}

impl ClosedMarket {
    pub fn settle(mut self, settle_token_id: TokenId) -> SettledMarket {
        let settle_token = self
            .base
            .tokens
            .iter()
            .find(|t| t.id == settle_token_id)
            .expect("Logic error : call ClosedMarket::settle with non-market token_id")
            .clone();

        // Settle orderを発行
        let distribution = self.token_user_distribution();
        let now = Utc::now();
        for ((token_id, user_id), amount_token) in distribution.iter() {
            let amount_coin = match *token_id == settle_token_id {
                true => AmountCoin(amount_token.0 * 1000),
                false => AmountCoin(0),
            };
            let settle_order = SettleOrder {
                user_id: *user_id,
                token_id: *token_id,
                amount_token: -amount_token,
                amount_coin,
                time: now,
            };
            self.orders.push_valid_order(Order::Settle(settle_order));
        }

        SettledMarket {
            base: self.base,
            orders: self.orders,
            settle_token,
        }
    }

    fn token_user_distribution(&self) -> HashMap<(TokenId, UserId), AmountToken> {
        let mut distribution = HashMap::new();

        // update
        for (_id, order) in self.orders.iter() {
            match order {
                Order::Normal(n) => {
                    let key = (n.token_id, n.user_id);
                    let v = distribution.get(&key).cloned().unwrap_or(AmountToken(0));
                    distribution.insert(key, v + n.amount_token);
                }
                _ => {}
            }
        }

        distribution
    }
}

impl MarketTokens {
    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self.0.iter()
    }
}
