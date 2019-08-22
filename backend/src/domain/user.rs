pub mod repository;
pub use repository::*;

use arrayvec::ArrayString;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    name: UserName,
    email: UserEmail,
    is_admin: bool,
}

impl User {
    /// 新たにエンティティが作られる時の関数
    pub fn new(id: UserId, name: UserName, email: UserEmail) -> User {
        User {
            id,
            name,
            email,
            is_admin: false,
        }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn email(&self) -> &UserEmail {
        &self.email
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From, Deref)]
pub struct UserName(String);

impl UserName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From, Deref)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}