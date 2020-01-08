pub mod access_token;

use self::access_token::AccessToken;
use crate::domain::{market::num::AmountCoin, point::Point, user::services::auth::Credentials};
use crate::primitive::{EmptyStringError, NonEmptyString};
use failure::Fallible;
use getset::Getters;
use uuid::Uuid;

/// Userモデルを表現するインターフェイス
/// このトレイトは最低限の要求しかしない。
/// より詳細な情報が必要であれば `UserWithAttrs` や `UserWithPoint`
/// などを要求する。
pub trait User: Sized {
    fn id(&self) -> &UserId;

    fn new_access_token(&self) -> AccessToken {
        AccessToken::new(self.id())
    }
}

/// 基本的な属性を保持するUserモデルを表現するインターフェイス
pub trait UserWithAttrs: User {
    fn name(&self) -> &UserName;
    fn email(&self) -> &UserEmail;
    fn coin(&self) -> AmountCoin;
    fn point(&self) -> Point;
    fn is_admin(&self) -> bool;

    fn into_admin(self) -> Fallible<Admin<Self>> {
        if self.is_admin() {
            Ok(Admin { user: self })
        } else {
            Err(failure::err_msg(format!(
                "{:?} is not an Admin user",
                self.id()
            )))
        }
    }
}

/// このprivateなメソッドを定義するために利用
trait UserWithAttrsExt: UserWithAttrs {
    fn provide_coin(self, provided: AmountCoin) -> UserProvidedCoin<Self> {
        UserProvidedCoin {
            user: self,
            provided,
        }
    }
}

impl<U> UserWithAttrsExt for U where U: UserWithAttrs {}

/*
 * ==================
 *  NewUser model
 * ==================
 */
#[derive(Clone, Getters, Into)]
#[get = "pub"]
pub struct NewUser {
    id: UserId,
    name: UserName,
    email: UserEmail,
    cred: Credentials,
}

impl NewUser {
    /// 新たにエンティティが作られる時の関数
    pub fn new(name: UserName, email: UserEmail, cred: Credentials) -> NewUser {
        NewUser {
            id: UserId::new(),
            name,
            email,
            cred,
        }
    }
}

impl User for NewUser {
    fn id(&self) -> &UserId {
        &self.id
    }
}

impl UserWithAttrs for NewUser {
    fn name(&self) -> &UserName {
        &self.name
    }
    fn email(&self) -> &UserEmail {
        &self.email
    }
    fn coin(&self) -> AmountCoin {
        AmountCoin::zero()
    }
    fn point(&self) -> Point {
        Point::zero()
    }
    fn is_admin(&self) -> bool {
        false
    }
}

/*
 * =================
 * UserProvidedCoin model
 * =================
 *
 * - コインを付与されたユーザーを表すモデル
 * - Repositoryに保存できる
 * - Admin::provide_coin_to_user メソッドを通じて生成する
 */
pub struct UserProvidedCoin<U> {
    user: U,
    provided: AmountCoin,
}

impl<U: User> User for UserProvidedCoin<U> {
    fn id(&self) -> &UserId {
        self.user.id()
    }
}

impl<U: UserWithAttrs> UserWithAttrs for UserProvidedCoin<U> {
    fn name(&self) -> &UserName {
        &self.user.name()
    }
    fn email(&self) -> &UserEmail {
        &self.user.email()
    }
    fn coin(&self) -> AmountCoin {
        self.user.coin() + self.provided
    }
    fn point(&self) -> Point {
        self.user.point()
    }
    fn is_admin(&self) -> bool {
        self.user.is_admin()
    }
}

/*
 * ==============
 * Admin model
 * ==============
 */
pub struct Admin<U> {
    user: U,
}

impl<U> Admin<U> {
    pub fn provide_coin_to_user<UU>(&self, user: UU, coin: AmountCoin) -> UserProvidedCoin<UU>
    where
        UU: UserWithAttrs,
    {
        user.provide_coin(coin)
    }
}

impl<U: User> User for Admin<U> {
    fn id(&self) -> &UserId {
        self.user.id()
    }
}

/*
 * ===================
 * Attribute models
 * ===================
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, From)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> UserId {
        UserId(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
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

    pub fn into_string(self) -> String {
        self.0.into_string()
    }

    pub fn from_str(s: String) -> Result<Self, EmptyStringError> {
        Ok(UserEmail(NonEmptyString::from_str(s)?))
    }
}
