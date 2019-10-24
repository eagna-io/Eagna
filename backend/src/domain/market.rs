pub mod num;
pub mod order;
pub mod repository;
pub mod services;
pub use num::*;
pub use order::*;
pub use repository::*;

pub const MAX_SLIP_RATE: f64 = 0.05; // 5 %;
pub const INITIAL_SUPPLY_COIN: AmountCoin = AmountCoin(10000);
pub const REWARD_COIN_PER_TOKEN: AmountCoin = AmountCoin(1000);

use crate::domain::{
    lmsr,
    organizer::{Organizer, OrganizerId},
    point::Point,
    user::UserId,
};
use crate::primitive::{NonEmptyString, NonEmptyVec};
use chrono::{DateTime, Utc};
use failure::Fallible;
use getset::Getters;
use num_rational::Ratio;
use rand::{Rng, RngCore};
use std::{collections::HashMap, str::FromStr};
use uuid::Uuid;

#[derive(Debug, From)]
/// アプリケーション層がこのモデルを取得するためには
/// リポジトリから取得するか、新規作成するしかない。
pub enum Market {
    Upcoming(UpcomingMarket),
    Open(OpenMarket),
    Closed(ClosedMarket),
    Resolved(ResolvedMarket),
}

impl Market {
    /// 新規にマーケットエンティティを作成する。
    /// 「新規作成」と「再構築」は違うことに注意。
    /// このメソッドはOrganizerを要求するが、それはオーガナイザーが
    /// 実際に存在することを強制するため。
    /// Organizer は OrganizerRepository から取得するしかないので
    /// Organizer 構造体が存在するなら対応するオーガナイザーが存在する
    pub fn new(
        title: NonEmptyString,
        organizer: &Organizer,
        desc: String,
        lmsr_b: lmsr::B,
        total_reward_point: Point,
        open: DateTime<Utc>,
        close: DateTime<Utc>,
        tokens: NonEmptyVec<MarketToken>,
        prizes: NonEmptyVec<MarketPrize>,
    ) -> Market {
        let id = MarketId::new();
        let attrs = MarketAttrs {
            title,
            organizer_id: organizer.id().clone(),
            description: desc,
            lmsr_b,
            total_reward_point,
            open,
            close,
            tokens,
            prizes,
        };
        let orders = MarketOrders::new();
        Market::Upcoming(UpcomingMarket {
            id,
            token_distribution: TokenDistribution::from(&attrs.tokens, &orders),
            attrs,
            orders,
        })
    }
}

impl AbstractMarket for Market {
    fn id(&self) -> &MarketId {
        match self {
            Market::Upcoming(ref inner) => inner.id(),
            Market::Open(ref inner) => inner.id(),
            Market::Closed(ref inner) => inner.id(),
            Market::Resolved(ref inner) => inner.id(),
        }
    }

    fn attrs(&self) -> &MarketAttrs {
        match self {
            Market::Upcoming(ref inner) => inner.attrs(),
            Market::Open(ref inner) => inner.attrs(),
            Market::Closed(ref inner) => inner.attrs(),
            Market::Resolved(ref inner) => inner.attrs(),
        }
    }

    fn orders(&self) -> &MarketOrders {
        match self {
            Market::Upcoming(ref inner) => inner.orders(),
            Market::Open(ref inner) => inner.orders(),
            Market::Closed(ref inner) => inner.orders(),
            Market::Resolved(ref inner) => inner.orders(),
        }
    }

    fn status(&self) -> MarketStatus {
        match self {
            Market::Upcoming(_) => MarketStatus::Upcoming,
            Market::Open(_) => MarketStatus::Open,
            Market::Closed(_) => MarketStatus::Closed,
            Market::Resolved(_) => MarketStatus::Upcoming,
        }
    }

    fn token_distribution(&self) -> &TokenDistribution {
        match self {
            Market::Upcoming(ref inner) => inner.token_distribution(),
            Market::Open(ref inner) => inner.token_distribution(),
            Market::Closed(ref inner) => inner.token_distribution(),
            Market::Resolved(ref inner) => inner.token_distribution(),
        }
    }
}

pub trait AbstractMarket {
    fn id(&self) -> &MarketId;
    fn attrs(&self) -> &MarketAttrs;
    fn orders(&self) -> &MarketOrders;
    fn token_distribution(&self) -> &TokenDistribution;
    fn status(&self) -> MarketStatus;

    /// 全Orderを走査するのでOrder数に比例してコストが高くなる
    fn num_users(&self) -> usize {
        self.orders()
            .iter()
            .filter(|o| o.type_() == OrderType::CoinSupply)
            .count()
    }

    fn point_coin_rate(&self) -> Fallible<PointCoinRate> {
        let num_users = self.num_users();
        if num_users == 0 {
            return Err(failure::err_msg(
                "Since there is no user, it is infinite rate",
            ));
        }
        let total_issued_coin = INITIAL_SUPPLY_COIN * num_users as i32;
        Ok(PointCoinRate(Ratio::new(
            self.attrs().total_reward_point().as_u32(),
            total_issued_coin.as_i32() as u32,
        )))
    }
}

/// ポイントとコインの交換比率を表現する構造体
pub struct PointCoinRate(Ratio<u32>);

impl PointCoinRate {
    pub fn as_f64(&self) -> f64 {
        (*self.0.numer() as f64) / (*self.0.denom() as f64)
    }
}

impl std::ops::Mul<AmountCoin> for PointCoinRate {
    type Output = (Point, FractPoint);

    fn mul(self, rhs: AmountCoin) -> (Point, FractPoint) {
        assert!(rhs >= AmountCoin::zero());
        let point = self.0 * rhs.as_i32() as u32;
        (Point::from(point.to_integer()), FractPoint(point.fract()))
    }
}

pub struct FractPoint(Ratio<u32>);

impl FractPoint {
    /// Returns 1 with a probability of this value.
    pub fn to_integer_with_probability<Rng>(&self, rng: &mut Rng) -> Point
    where
        Rng: RngCore,
    {
        Point::from(rng.gen_ratio(*self.0.numer(), *self.0.denom()) as u32)
    }
}

macro_rules! impl_abstract_market {
    ($ty: ident, $status: expr) => {
        impl AbstractMarket for $ty {
            fn id(&self) -> &MarketId {
                &self.id
            }
            fn attrs(&self) -> &MarketAttrs {
                &self.attrs
            }
            fn orders(&self) -> &MarketOrders {
                &self.orders
            }
            fn token_distribution(&self) -> &TokenDistribution {
                &self.token_distribution
            }
            fn status(&self) -> MarketStatus {
                $status
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct UpcomingMarket {
    id: MarketId,
    attrs: MarketAttrs,
    orders: MarketOrders,
    token_distribution: TokenDistribution,
}

impl_abstract_market!(UpcomingMarket, MarketStatus::Upcoming);

impl UpcomingMarket {
    pub fn try_open(self) -> Result<OpenMarket, UpcomingMarket> {
        if self.is_opened() {
            Ok(OpenMarket {
                id: self.id,
                attrs: self.attrs,
                orders: self.orders,
                token_distribution: self.token_distribution,
            })
        } else {
            Err(self)
        }
    }

    fn is_opened(&self) -> bool {
        self.attrs.open < Utc::now()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct OpenMarket {
    id: MarketId,
    attrs: MarketAttrs,
    orders: MarketOrders,
    token_distribution: TokenDistribution,
}

impl_abstract_market!(OpenMarket, MarketStatus::Open);

#[derive(Debug, Clone, PartialEq, Eq, Fail)]
pub enum TryOrderError {
    #[fail(display = "User does not have sufficient balance of coin")]
    InsufficientBalance,
    #[fail(display = "User may try to buy or sell 0 token")]
    InvalidAmountOfToken,
    #[fail(display = "Specified token does not exists on specified market")]
    InvalidToken,
}

#[derive(Debug, Clone, PartialEq, Eq, Fail)]
pub enum SupplyInitialCoinError {
    #[fail(display = "User is already received initial coin")]
    AlreadyReceived,
}

impl OpenMarket {
    pub fn try_close(self) -> Result<ClosedMarket, OpenMarket> {
        if self.is_closed() {
            Ok(ClosedMarket {
                id: self.id,
                attrs: self.attrs,
                orders: self.orders,
                token_distribution: self.token_distribution,
            })
        } else {
            Err(self)
        }
    }

    fn is_closed(&self) -> bool {
        self.attrs.close < Utc::now()
    }

    /// ユーザーがまだInitialSupplyを受け取っていない場合、
    /// InitialSupplyを付与する
    pub fn try_supply_initial_coin(
        &mut self,
        user_id: &UserId,
    ) -> Result<&Order, SupplyInitialCoinError> {
        log::debug!("Try supply initial coin to {:?}", user_id);

        if self.orders.is_already_supply_initial_coin_to(user_id) {
            return Err(SupplyInitialCoinError::AlreadyReceived);
        }

        Ok(self
            .orders
            .add_coin_supply_order(*user_id, INITIAL_SUPPLY_COIN))
    }

    /// 新しいNormalOrderを追加する。
    /// - Userの残高が十分にあるか
    /// をチェックする.
    /// チェックが通った場合にのみ、NormalOrderを追加する
    /// amount_token が0より大きければBUY、0より小さければSELLのオーダー
    pub fn try_add_normal_order(
        &mut self,
        user_id: &UserId,
        token_name: &NonEmptyString,
        amount_token: &AmountToken,
    ) -> Result<&Order, TryOrderError> {
        log::debug!(
            "Try add a new order [ user : {:?}, token : {:?}, amount: {:?}",
            user_id,
            token_name,
            amount_token
        );

        if !self.attrs.is_valid_token(token_name) {
            return Err(TryOrderError::InvalidToken);
        }

        if *amount_token == AmountToken::zero() {
            return Err(TryOrderError::InvalidAmountOfToken);
        }

        let amount_coin = self.compute_amount_coin_of_order(token_name, *amount_token);

        if *amount_token < AmountToken::zero() {
            // 売り注文なので、トークンの残高をチェックする
            let token_balance = self
                .orders
                .compute_balance_of_user_token(user_id, token_name);
            if token_balance + *amount_token < AmountToken::zero() {
                return Err(TryOrderError::InsufficientBalance);
            }
        } else {
            // 買い注文なので、コインの残高をチェックする
            let coin_balance = self.orders.compute_balance_of_user_coin(user_id);
            if coin_balance + amount_coin < AmountCoin::zero() {
                return Err(TryOrderError::InsufficientBalance);
            }
        }

        // token_distribution をアップデート
        self.token_distribution
            .update_add(token_name, *amount_token);

        Ok(self
            .orders
            .add_normal_order(*user_id, token_name.clone(), *amount_token, amount_coin))
    }

    /// 指定のTokenを、指定の数量売る/買うとき、増える/減るCoinの量
    fn compute_amount_coin_of_order(
        &self,
        token_name: &NonEmptyString,
        amount_token: AmountToken,
    ) -> AmountCoin {
        let lmsr_b = self.attrs.lmsr_b;

        let current_cost = lmsr::cost(lmsr_b, self.token_distribution.values().copied());

        let new_distribution_values = self.token_distribution.iter().map(|(tname, amt)| {
            if tname == token_name {
                *amt + amount_token
            } else {
                *amt
            }
        });
        let new_cost = lmsr::cost(lmsr_b, new_distribution_values);

        // costが増えたとき、coinは減る（買い注文）and vice versa.
        -(new_cost - current_cost)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct ClosedMarket {
    id: MarketId,
    attrs: MarketAttrs,
    orders: MarketOrders,
    token_distribution: TokenDistribution,
}

impl_abstract_market!(ClosedMarket, MarketStatus::Closed);

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct ResolvedMarket {
    id: MarketId,
    attrs: MarketAttrs,
    orders: MarketOrders,
    token_distribution: TokenDistribution,
    resolved_token_name: NonEmptyString,
    reward_records: RewardRecords,
}

impl_abstract_market!(ResolvedMarket, MarketStatus::Resolved);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardRecords(HashMap<UserId, Point>);

impl std::ops::Deref for RewardRecords {
    type Target = HashMap<UserId, Point>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenDistribution(HashMap<NonEmptyString, AmountToken>);

impl TokenDistribution {
    fn from(tokens: &NonEmptyVec<MarketToken>, orders: &MarketOrders) -> TokenDistribution {
        let mut map = HashMap::new();
        for token in tokens.iter() {
            map.insert(token.name.clone(), AmountToken::zero());
        }

        let mut token_distribution = TokenDistribution(map);

        for normal_order in orders.filter_normal_orders() {
            token_distribution.update_add(&normal_order.token_name(), *normal_order.amount_token())
        }

        token_distribution
    }

    pub fn get(&self, token_name: &NonEmptyString) -> Option<AmountToken> {
        self.0.get(token_name).copied()
    }

    pub fn update_add(&mut self, token_name: &NonEmptyString, amount_token: AmountToken) {
        match self.0.get_mut(token_name) {
            Some(current_v) => *current_v += amount_token,
            None => {}
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &AmountToken> {
        self.0.values()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&NonEmptyString, &AmountToken)> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct MarketAttrs {
    title: NonEmptyString,
    organizer_id: OrganizerId,
    description: String,
    lmsr_b: lmsr::B,
    total_reward_point: Point,
    open: DateTime<Utc>,
    close: DateTime<Utc>,
    // tokens は、DB の market_tokens テーブルに保存されている
    // idx カラムの値でソートされている。
    tokens: NonEmptyVec<MarketToken>,
    prizes: NonEmptyVec<MarketPrize>,
}

impl MarketAttrs {
    pub fn is_valid_token(&self, token_name: &NonEmptyString) -> bool {
        self.tokens.iter().find(|t| &t.name == token_name).is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, From)]
pub struct MarketId(Uuid);

impl MarketId {
    fn new() -> MarketId {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketStatus {
    Upcoming,
    Open,
    Closed,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
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

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct MarketPrize {
    id: i32,
    name: NonEmptyString,
    thumbnail_url: String,
    target: String,
}

impl MarketPrize {
    pub fn new(
        id: i32,
        name: NonEmptyString,
        thumbnail_url: String,
        target: String,
    ) -> MarketPrize {
        MarketPrize {
            id,
            name,
            thumbnail_url,
            target,
        }
    }
}
