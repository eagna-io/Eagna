use crate::lmsr;
use crate::market::num::{AmountCoin, AmountToken};
use crate::market::order::MarketOrders;
use crate::user::models::UserId;
use chrono::{DateTime, Utc};
use crop_primitive::{NonEmptyString, NonEmptyVec};
use getset::Getters;
use std::{collections::HashMap, str::FromStr};
use uuid::Uuid;

/*
 * ==================
 * Market trait
 * ==================
 */
pub trait Market {
    fn id(&self) -> MarketId;
    fn attrs(&self) -> &MarketAttrs;
    fn status(&self) -> MarketStatus;
    fn orders(&self) -> &MarketOrders;

    fn is_valid_token(&self, token_name: &NonEmptyString) -> bool {
        self.attrs().tokens().iter().any(|t| &t.name == token_name)
    }

    fn compute_token_distribution(&self) -> TokenDistribution {
        TokenDistribution::from(&self.attrs().tokens, self.orders())
    }

    fn compute_balance_of_user_token(
        &self,
        user_id: &UserId,
        token: &NonEmptyString,
    ) -> AmountToken {
        self.orders().compute_balance_of_user_token(user_id, token)
    }
}

/*
 * ========================
 * Upcoming Market
 * ========================
 */
pub trait UpcomingMarket: Market {
    fn is_openable(&self) -> bool {
        self.attrs().open < Utc::now()
    }
}

/*
 * ======================
 * Open Market struct
 * ======================
 */
pub trait OpenMarket: Market {
    fn is_closable(&self) -> bool {
        self.attrs().close < Utc::now()
    }

    /// 指定のTokenを増やす/減らすオーダーで、減る/増えるCoinの量
    #[doc(hidden)]
    fn compute_cost_of_order(
        &self,
        token_name: &NonEmptyString,
        amount_token: AmountToken,
    ) -> AmountCoin {
        let lmsr_b = self.attrs().lmsr_b;
        let token_distribution = self.compute_token_distribution();

        let cur_cost = lmsr::cost(lmsr_b, token_distribution.values().copied());

        let new_distribution_values = token_distribution.iter().map(|(token, amt)| {
            if token == token_name {
                *amt + amount_token
            } else {
                *amt
            }
        });
        let new_cost = lmsr::cost(lmsr_b, new_distribution_values);

        // costが増えたとき、coinは減る
        new_cost - cur_cost
    }

    /// トークンの購入に必要なコイン量を計算
    fn compute_price_of_buy(
        &self,
        token_name: &NonEmptyString,
        amount_token: AmountToken,
    ) -> AmountCoin {
        self.compute_cost_of_order(token_name, amount_token)
    }

    /// トークンの売却で手に入るコイン量を計算
    fn compute_gain_of_sell(
        &self,
        token_name: &NonEmptyString,
        amount_token: AmountToken, // > 0
    ) -> AmountCoin {
        -self.compute_cost_of_order(token_name, -amount_token)
    }
}

/*
 * ======================
 * Closed Market struct
 * ======================
 */
pub trait ClosedMarket: Market {}

/*
 * ======================
 * Resolved Market struct
 * ======================
 */
pub trait ResolvedMarket: Market {
    fn resolved_token_name(&self) -> &NonEmptyString;
}

/*
 * ==================
 * Another models
 * ==================
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, From)]
pub struct MarketId(Uuid);

impl MarketId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> MarketId {
        MarketId(Uuid::new_v4())
    }
    /// ex : "e643a0dadc5c4c2d9585c2c6da0cf77d"
    pub fn to_str(&self) -> impl AsRef<str> {
        let mut str_buf = Uuid::encode_buffer();
        let tmp_str = self.0.to_simple_ref().encode_lower(&mut str_buf);
        arrayvec::ArrayString::<[u8; uuid::adapter::Simple::LENGTH]>::from(tmp_str).unwrap()
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl FromStr for MarketId {
    type Err = uuid::parser::ParseError;
    fn from_str(src: &str) -> Result<MarketId, Self::Err> {
        Ok(MarketId(Uuid::parse_str(src)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters, From)]
#[get = "pub"]
pub struct MarketAttrs {
    title: NonEmptyString,
    description: String,
    lmsr_b: lmsr::B,
    open: DateTime<Utc>,
    close: DateTime<Utc>,
    // tokens は、DB の market_tokens テーブルに保存されている
    // idx カラムの値でソートされている。
    tokens: NonEmptyVec<MarketToken>,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters, From)]
#[get = "pub"]
pub struct MarketToken {
    name: NonEmptyString,
    description: String,
    thumbnail_url: String,
}

impl MarketToken {
    pub fn new(name: NonEmptyString, description: String, thumbnail_url: String) -> MarketToken {
        MarketToken {
            name,
            description,
            thumbnail_url,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketStatus {
    Upcoming,
    Open,
    Closed,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenDistribution<'a>(HashMap<&'a NonEmptyString, AmountToken>);

impl<'a> TokenDistribution<'a> {
    fn from(
        tokens: &'a NonEmptyVec<MarketToken>,
        orders: &'a MarketOrders,
    ) -> TokenDistribution<'a> {
        let mut map = HashMap::new();
        for token in tokens.iter() {
            map.insert(&token.name, AmountToken::zero());
        }

        let mut token_distribution = TokenDistribution(map);

        for order in orders.iter() {
            token_distribution.update_add(&order.token_name(), *order.amount_token())
        }

        token_distribution
    }

    pub fn get(&self, token_name: &NonEmptyString) -> Option<AmountToken> {
        self.0.get(token_name).copied()
    }

    pub fn update_add(&mut self, token_name: &NonEmptyString, amount_token: AmountToken) {
        if let Some(current_v) = self.0.get_mut(token_name) {
            *current_v += amount_token;
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &AmountToken> {
        self.0.values()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'a NonEmptyString, &AmountToken)> {
        self.0.iter().map(|(s, a)| (*s, a))
    }
}
