pub mod point_history;
pub mod repository;
pub use point_history::*;
pub use repository::*;

use crate::domain::point::Point;
use crate::primitive::{EmptyStringError, NonEmptyString};
use arrayvec::ArrayString;
use getset::Getters;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct User {
    id: UserId,
    name: UserName,
    email: UserEmail,
    is_admin: bool,
    point: Point,
}

impl User {
    /// 新たにエンティティが作られる時の関数
    pub fn new(id: UserId, name: UserName, email: UserEmail) -> User {
        User {
            id,
            name,
            email,
            is_admin: false,
            point: Point::zero(),
        }
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
