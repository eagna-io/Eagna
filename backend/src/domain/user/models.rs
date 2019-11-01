pub mod access_token;
pub use access_token::*;

use crate::domain::{
    market::MarketId,
    point::Point,
    prize::{Prize, PrizeId},
};
use crate::primitive::{EmptyStringError, NonEmptyString};
use arrayvec::ArrayString;
use chrono::{DateTime, Utc};
use failure::Fallible;
use getset::Getters;
use uuid::Uuid;

pub trait User: Sized {
    fn id(&self) -> &UserId;
    fn name(&self) -> &UserName;
    fn email(&self) -> &UserEmail;
    fn is_admin(&self) -> bool;

    fn new_access_token(&self) -> AccessToken {
        AccessToken::new(self.id())
    }
}

macro_rules! impl_user {
    ($ty: ident) => {
        impl<U: User> User for $ty<U> {
            fn id(&self) -> &UserId {
                self.user.id()
            }
            fn name(&self) -> &UserName {
                self.user.name()
            }
            fn email(&self) -> &UserEmail {
                self.user.email()
            }
            fn is_admin(&self) -> bool {
                self.user.is_admin()
            }
        }
    };
}

pub trait UserWithPoint: User {
    fn point(&self) -> Point;

    fn request_prize_trade(self, prize: &Prize) -> Fallible<UserWithPrizeTradeRequest<Self>> {
        if self.point() - *prize.point() < Point::zero() {
            return Err(failure::err_msg(
                "user does not have enough point to request prize trade",
            ));
        }
        let record = PrizeTradeRecord::new(prize);
        Ok(UserWithPrizeTradeRequest {
            user: self,
            requested_prize_trade_record: record,
        })
    }
}

macro_rules! impl_user_with_point {
    ($ty: ident) => {
        impl<U: UserWithPoint> UserWithPoint for $ty<U> {
            fn point(&self) -> Point {
                self.user.point()
            }
        }
    };
}

pub trait UserWithPrizeTradeHistory: User {
    fn prize_trade_history(&self) -> &Vec<PrizeTradeRecord>;
}

macro_rules! impl_user_with_prize_trade_history {
    ($ty: ident) => {
        impl<U: UserWithPrizeTradeHistory> UserWithPrizeTradeHistory for $ty<U> {
            fn prize_trade_history(&self) -> &Vec<PrizeTradeRecord> {
                self.user.prize_trade_history()
            }
        }
    };
}

pub trait UserWithMarketRewardHistory: User {
    fn market_reward_history(&self) -> &Vec<MarketRewardRecord>;
}

macro_rules! impl_user_with_market_reward_history {
    ($ty: ident) => {
        impl<U: UserWithMarketRewardHistory> UserWithMarketRewardHistory for $ty<U> {
            fn market_reward_history(&self) -> &Vec<MarketRewardRecord> {
                self.user.market_reward_history()
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct UserWithPrizeTradeRequest<U> {
    user: U,
    requested_prize_trade_record: PrizeTradeRecord,
}

impl<U: UserWithPoint> UserWithPoint for UserWithPrizeTradeRequest<U> {
    fn point(&self) -> Point {
        self.user.point() - self.requested_prize_trade_record.point
    }
}

impl_user!(UserWithPrizeTradeRequest);
impl_user_with_market_reward_history!(UserWithPrizeTradeRequest);

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct NewUser {
    pub(super) id: UserId,
    pub(super) name: UserName,
    pub(super) email: UserEmail,
}

impl NewUser {
    /// 新たにエンティティが作られる時の関数
    pub fn new(id: UserId, name: UserName, email: UserEmail) -> NewUser {
        NewUser { id, name, email }
    }
}

impl User for NewUser {
    fn id(&self) -> &UserId {
        &self.id
    }
    fn name(&self) -> &UserName {
        &self.name
    }
    fn email(&self) -> &UserEmail {
        &self.email
    }
    fn is_admin(&self) -> bool {
        false
    }
}

impl UserWithPoint for NewUser {
    fn point(&self) -> Point {
        Point::zero()
    }
}

impl UserWithPrizeTradeHistory for NewUser {
    fn prize_trade_history(&self) -> &Vec<PrizeTradeRecord> {
        lazy_static::lazy_static! {
            static ref EMPTY_VEC: Vec<PrizeTradeRecord> = Vec::new();
        }
        &EMPTY_VEC
    }
}

impl UserWithMarketRewardHistory for NewUser {
    fn market_reward_history(&self) -> &Vec<MarketRewardRecord> {
        lazy_static::lazy_static! {
            static ref EMPTY_VEC: Vec<MarketRewardRecord> = Vec::new();
        }
        &EMPTY_VEC
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Firebase が発行するuidは現在28文字。
/// しかし将来的に増える可能性がある（Firebaseはuidの長さについて言及していない）ので、
/// 48文字まで対応できるようにしている。
/// もしFirebaseのuidが36文字以上になってきたら、48以上にすることを検討すべき
pub struct UserId(ArrayString<[u8; 48]>);

impl UserId {
    pub fn from_str(s: &str) -> UserId {
        UserId(ArrayString::from(s).unwrap())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Deref, From)]
pub struct UserName(NonEmptyString);

impl UserName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn from_str(s: String) -> Result<Self, EmptyStringError> {
        Ok(UserName(NonEmptyString::from_str(s)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Deref, From)]
pub struct UserEmail(NonEmptyString);

impl UserEmail {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn from_str(s: String) -> Result<Self, EmptyStringError> {
        Ok(UserEmail(NonEmptyString::from_str(s)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct PrizeTradeRecord {
    pub(super) id: Uuid,
    pub(super) prize_id: PrizeId,
    pub(super) point: Point,
    pub(super) time: DateTime<Utc>,
    pub(super) status: PrizeTradeStatus,
}

impl PrizeTradeRecord {
    /// PrizeTradeRecord モデルを新規作成する。
    /// `Prize` を要求することで、その `Prize` が
    /// 実際に存在していることを証明する。
    /// `UserWithPoint::request_prize_trade` メソッドから呼び出される。
    fn new(prize: &Prize) -> PrizeTradeRecord {
        PrizeTradeRecord {
            id: Uuid::new_v4(),
            prize_id: *prize.id(),
            point: *prize.point(),
            time: Utc::now(),
            status: PrizeTradeStatus::Requested,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrizeTradeStatus {
    Requested,
    Processed,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct MarketRewardRecord {
    pub(super) market_id: MarketId,
    pub(super) point: Point,
}
