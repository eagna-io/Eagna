mod num;
mod order;
mod repository;
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
use crate::infra::postgres::types::MarketStatus as InfraMarketStatus;
use chrono::{DateTime, Utc};
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
        title: MarketTitle,
        organizer: &Organizer,
        desc: MarketDesc,
        lmsr_b: lmsr::B,
        total_reward_point: Point,
        open: MarketOpenTime,
        close: MarketCloseTime,
        tokens: MarketTokens,
        prizes: MarketPrizes,
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

pub trait AbstractMarket {
    fn id(&self) -> &MarketId;

    fn attrs(&self) -> &MarketAttrs;

    fn status(&self) -> MarketStatus;

    fn token_distribution(&self) -> &TokenDistribution;

    fn orders(&self) -> &MarketOrders;

    fn resolved_token_name(&self) -> Option<&TokenName>;

    fn flatten(self) -> FlattenMarket;
}

macro_rules! market_inner_fn_ref {
    ($fn: ident, $ret: ty) => {
        fn $fn(&self) -> $ret {
            match self {
                Market::Upcoming(ref m) => m.$fn(),
                Market::Open(ref m) => m.$fn(),
                Market::Closed(ref m) => m.$fn(),
                Market::Resolved(ref m) => m.$fn(),
            }
        }
    }
}

impl AbstractMarket for Market {
    market_inner_fn_ref!(id, &MarketId);
    market_inner_fn_ref!(attrs, &MarketAttrs);
    market_inner_fn_ref!(status, MarketStatus);
    market_inner_fn_ref!(token_distribution, &TokenDistribution);
    market_inner_fn_ref!(orders, &MarketOrders);
    market_inner_fn_ref!(resolved_token_name, Option<&TokenName>);

    fn flatten(self) -> FlattenMarket {
        match self {
            Market::Upcoming(m) => m.flatten(),
            Market::Open(m) => m.flatten(),
            Market::Closed(m) => m.flatten(),
            Market::Resolved(m) => m.flatten(),
        }
    }
}

pub struct FlattenMarket {
    pub id: MarketId,
    pub attrs: MarketAttrs,
    pub status: MarketStatus,
    pub token_distribution: TokenDistribution,
    pub orders: MarketOrders,
    pub resolved_token_name: Option<TokenName>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpcomingMarket {
    id: MarketId,
    attrs: MarketAttrs,
    orders: MarketOrders,
    token_distribution: TokenDistribution,
}

impl UpcomingMarket {
    pub fn try_open(self) -> Result<OpenMarket, UpcomingMarket> {
        if self.attrs.open.is_opened() {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenMarket {
    id: MarketId,
    attrs: MarketAttrs,
    orders: MarketOrders,
    token_distribution: TokenDistribution,
}

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
        if self.attrs.close.is_closed() {
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
        token_name: &TokenName,
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
        token_name: &TokenName,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosedMarket {
    id: MarketId,
    attrs: MarketAttrs,
    orders: MarketOrders,
    token_distribution: TokenDistribution,
}

#[derive(Debug, Clone, PartialEq, Eq, Fail)]
pub enum ResolveMarketError {
    #[fail(display = "Specified token does not exists on specified market")]
    InvalidTokenId,
}

impl ClosedMarket {
    pub fn resolve(
        mut self,
        resolved_token_name: TokenName,
    ) -> Result<ResolvedMarket, ResolveMarketError> {
        if !self.attrs.is_valid_token(&resolved_token_name) {
            return Err(ResolveMarketError::InvalidTokenId);
        }

        // Reward orderを発行
        for (user_id, amount_token) in self
            .orders
            .compute_amount_token_of_each_user(&resolved_token_name)
            .into_iter()
        {
            self.orders.add_reward_order(
                user_id,
                resolved_token_name.clone(),
                REWARD_COIN_PER_TOKEN * amount_token.as_i32(),
            );
        }

        Ok(ResolvedMarket {
            id: self.id,
            attrs: self.attrs,
            orders: self.orders,
            token_distribution: self.token_distribution,
            resolved_token_name,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedMarket {
    id: MarketId,
    attrs: MarketAttrs,
    orders: MarketOrders,
    token_distribution: TokenDistribution,
    resolved_token_name: TokenName,
}

macro_rules! impl_abstract_market_except_for_resolved {
    ($ty: ty, $status: expr) => {
        impl AbstractMarket for $ty {
            fn id(&self) -> &MarketId {
                &self.id
            }

            fn attrs(&self) -> &MarketAttrs {
                &self.attrs
            }

            fn status(&self) -> MarketStatus {
                $status
            }

            fn token_distribution(&self) -> &TokenDistribution {
                &self.token_distribution
            }

            fn orders(&self) -> &MarketOrders {
                &self.orders
            }

            fn resolved_token_name(&self) -> Option<&TokenName> {
                None
            }

            fn flatten(self) -> FlattenMarket {
                FlattenMarket {
                    id: self.id,
                    attrs: self.attrs,
                    status: $status,
                    token_distribution: self.token_distribution,
                    orders: self.orders,
                    resolved_token_name: None,
                }
            }
        }
    };
}

impl_abstract_market_except_for_resolved!(UpcomingMarket, MarketStatus::Upcoming);
impl_abstract_market_except_for_resolved!(OpenMarket, MarketStatus::Open);
impl_abstract_market_except_for_resolved!(ClosedMarket, MarketStatus::Closed);

impl AbstractMarket for ResolvedMarket {
    fn id(&self) -> &MarketId {
        &self.id
    }

    fn attrs(&self) -> &MarketAttrs {
        &self.attrs
    }

    fn status(&self) -> MarketStatus {
        MarketStatus::Resolved
    }

    fn token_distribution(&self) -> &TokenDistribution {
        &self.token_distribution
    }

    fn orders(&self) -> &MarketOrders {
        &self.orders
    }

    fn resolved_token_name(&self) -> Option<&TokenName> {
        Some(&self.resolved_token_name)
    }

    fn flatten(self) -> FlattenMarket {
        FlattenMarket {
            id: self.id,
            attrs: self.attrs,
            status: MarketStatus::Resolved,
            token_distribution: self.token_distribution,
            orders: self.orders,
            resolved_token_name: Some(self.resolved_token_name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TokenDistribution(HashMap<TokenName, AmountToken>);

impl TokenDistribution {
    fn from(tokens: &MarketTokens, orders: &MarketOrders) -> TokenDistribution {
        let mut map = HashMap::new();
        for token in tokens.iter() {
            map.insert(token.name.clone(), AmountToken::zero());
        }

        let mut token_distribution = TokenDistribution(map);

        for normal_order in orders.filter_normal_orders() {
            token_distribution.update_add(&normal_order.token_name, normal_order.amount_token)
        }

        token_distribution
    }

    pub fn get(&self, token_name: &TokenName) -> Option<AmountToken> {
        self.0.get(token_name).copied()
    }

    pub fn update_add(&mut self, token_name: &TokenName, amount_token: AmountToken) {
        match self.0.get_mut(token_name) {
            Some(current_v) => *current_v += amount_token,
            None => {}
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &AmountToken> {
        self.0.values()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TokenName, &AmountToken)> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketAttrs {
    pub title: MarketTitle,
    pub organizer_id: OrganizerId,
    pub description: MarketDesc,
    pub lmsr_b: lmsr::B,
    pub total_reward_point: Point,
    pub open: MarketOpenTime,
    pub close: MarketCloseTime,
    pub tokens: MarketTokens,
    pub prizes: MarketPrizes,
}

impl MarketAttrs {
    fn is_valid_token(&self, token_name: &TokenName) -> bool {
        self.tokens.iter().find(|t| &t.name == token_name).is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, From)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub struct MarketTitle(String);

impl MarketTitle {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub struct MarketDesc(String);

impl MarketDesc {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub struct MarketOpenTime(DateTime<Utc>);

impl MarketOpenTime {
    pub fn is_opened(&self) -> bool {
        self.0 < Utc::now()
    }

    pub fn as_date_time(&self) -> &DateTime<Utc> {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub struct MarketCloseTime(DateTime<Utc>);

impl MarketCloseTime {
    pub fn is_closed(&self) -> bool {
        self.0 < Utc::now()
    }

    pub fn as_date_time(&self) -> &DateTime<Utc> {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketStatus {
    Upcoming,
    Open,
    Closed,
    Resolved,
}

impl Into<InfraMarketStatus> for MarketStatus {
    fn into(self) -> InfraMarketStatus {
        match self {
            MarketStatus::Upcoming => InfraMarketStatus::Upcoming,
            MarketStatus::Open => InfraMarketStatus::Open,
            MarketStatus::Closed => InfraMarketStatus::Closed,
            MarketStatus::Resolved => InfraMarketStatus::Resolved,
        }
    }
}

/// MarketTokens は、DB の market_tokens テーブルに保存されている
/// idx カラムの値でソートされている。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub struct MarketTokens(Vec<Token>);

impl MarketTokens {
    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub name: TokenName,
    pub description: TokenDesc,
    pub thumbnail_url: TokenThumbnailUrl,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, From)]
pub struct TokenName(String);

impl TokenName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub struct TokenDesc(String);

impl TokenDesc {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub struct TokenThumbnailUrl(String);

impl TokenThumbnailUrl {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub struct MarketPrizes(Vec<Prize>);

impl MarketPrizes {
    pub fn iter(&self) -> impl Iterator<Item = &Prize> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prize {
    pub id: PrizeId,
    pub name: PrizeName,
    pub thumbnail_url: PrizeThumbnailUrl,
    pub target: PrizeTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub struct PrizeId(i32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub struct PrizeName(String);

impl PrizeName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub struct PrizeThumbnailUrl(String);

impl PrizeThumbnailUrl {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub struct PrizeTarget(String);

impl PrizeTarget {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
