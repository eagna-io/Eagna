pub mod point_history;
pub mod repository;
pub use repository::*;

use crate::domain::point::Point;
use crate::primitive::{EmptyStringError, NonEmptyString};
use arrayvec::ArrayString;
use chrono::{DateTime, Utc};
use getset::Getters;
use uuid::Uuid;

pub trait User: Sized {
    fn id(&self) -> &UserId;
    fn name(&self) -> &UserName;
    fn email(&self) -> &UserEmail;
    fn is_admin(&self) -> bool;
}

pub trait UserWithPoint: User {
    fn point(&self) -> Point;
}

pub trait UserWithPrizeTradeHistory: User {
    fn prize_trade_history(&self) -> &Vec<PrizeTradeRecord>;
}

pub struct NewUser {
    id: UserId,
    name: UserName,
    email: UserEmail,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct PrizeTradeRecord {
    id: Uuid,
    prize_id: Uuid,
    point: Point,
    time: DateTime<Utc>,
    status: PrizeTradeStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrizeTradeStatus {
    Requested,
    Processed,
}
