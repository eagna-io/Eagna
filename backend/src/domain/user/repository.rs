pub mod access_token;

use crate::domain::point::Point;
use crate::domain::user::*;
use crate::infra::postgres::{
    user::{NewPrizeTradeRecord, NewUser as NewUserInfra},
    PostgresInfra,
};
use failure::Fallible;
use getset::{CopyGetters, Getters};

#[derive(From)]
pub struct UserRepository<'a> {
    postgres: &'a dyn PostgresInfra,
}

impl<'a> UserRepository<'a> {
    pub fn save_user(&self, new_user: &NewUser) -> Fallible<()> {
        self.postgres.save_user(NewUserInfra {
            id: *new_user.id().as_uuid(),
            name: new_user.name().as_str(),
            email: new_user.email().as_str(),
            credential: &new_user.cred().cred()[..],
            salt: &new_user.cred().salt()[..],
        })
    }

    pub fn query_user(&self, user_id: &UserId) -> Result<Option<QueryUser<'a>>, failure::Error> {
        let user = match self.postgres.query_user(user_id.as_uuid())? {
            None => return Ok(None),
            Some(res) => res,
        };
        Ok(Some(QueryUser {
            id: *user_id,
            name: UserName::from_str(user.name)?,
            email: UserEmail::from_str(user.email)?,
            coin: user.coin as u32,
            is_admin: user.is_admin,
            pg: self.postgres,
        }))
    }

    pub fn save_user_prize_trade_request<U>(
        &self,
        user: &UserWithPrizeTradeRequest<U>,
    ) -> Fallible<()>
    where
        U: User,
    {
        let new_prize_trade_record = NewPrizeTradeRecord {
            id: user.requested_prize_trade_record().id,
            prize_id: *user.requested_prize_trade_record().prize_id.as_uuid(),
            point: user.requested_prize_trade_record().point.as_u32(),
            time: user.requested_prize_trade_record().time,
        };
        self.postgres
            .save_user_prize_trade_record(user.id().as_uuid(), new_prize_trade_record)
    }
}

#[derive(Getters, CopyGetters)]
pub struct QueryUser<'a> {
    #[get = "pub"]
    id: UserId,
    #[get = "pub"]
    name: UserName,
    #[get = "pub"]
    email: UserEmail,
    #[get_copy = "pub"]
    coin: u32,
    #[get_copy = "pub"]
    is_admin: bool,

    pg: &'a dyn PostgresInfra,
}

impl<'a> User for QueryUser<'a> {
    fn id(&self) -> &UserId {
        &self.id
    }
}

impl<'a> UserWithAttrs for QueryUser<'a> {
    fn name(&self) -> &UserName {
        &self.name
    }
    fn email(&self) -> &UserEmail {
        &self.email
    }
    fn coin(&self) -> u32 {
        self.coin
    }
    fn is_admin(&self) -> bool {
        self.is_admin
    }
}

pub trait UserWithPg: User + Sized {
    fn pg(&self) -> &dyn PostgresInfra;

    fn with_point(self) -> Result<WithPoint<Self>, failure::Error> {
        let point = self.pg().query_user_point(self.id().as_uuid())?;
        Ok(WithPoint {
            user: self,
            point: Point::from(point),
        })
    }
}

impl<'a> UserWithPg for QueryUser<'a> {
    fn pg(&self) -> &dyn PostgresInfra {
        self.pg
    }
}

macro_rules! impl_user_with_pg {
    ($ty: ident) => {
        impl<U: UserWithPg> UserWithPg for $ty<U> {
            fn pg(&self) -> &dyn PostgresInfra {
                self.user.pg()
            }
        }
    };
}

pub struct WithPoint<U> {
    user: U,
    point: Point,
}

impl<U: User> UserWithPoint for WithPoint<U> {
    fn point(&self) -> Point {
        self.point
    }
}

impl_user!(WithPoint);
impl_user_with_attrs!(WithPoint);
impl_user_with_pg!(WithPoint);
impl_user_with_prize_trade_history!(WithPoint);
impl_user_with_market_reward_history!(WithPoint);

pub struct WithPrizeTradeHistory<U> {
    user: U,
    prize_trade_history: Vec<PrizeTradeRecord>,
}

impl<U: User> UserWithPrizeTradeHistory for WithPrizeTradeHistory<U> {
    fn prize_trade_history(&self) -> &Vec<PrizeTradeRecord> {
        &self.prize_trade_history
    }
}

impl_user!(WithPrizeTradeHistory);
impl_user_with_attrs!(WithPrizeTradeHistory);
impl_user_with_pg!(WithPrizeTradeHistory);
impl_user_with_point!(WithPrizeTradeHistory);
impl_user_with_market_reward_history!(WithPrizeTradeHistory);

pub struct WithMarketRewardHistory<U> {
    user: U,
    market_reward_records: Vec<MarketRewardRecord>,
}

impl<U: User> UserWithMarketRewardHistory for WithMarketRewardHistory<U> {
    fn market_reward_history(&self) -> &Vec<MarketRewardRecord> {
        &self.market_reward_records
    }
}

impl<U: UserWithPrizeTradeHistory> WithMarketRewardHistory<U> {
    pub fn compute_point(self) -> WithPoint<Self> {
        let traded_point = self
            .user
            .prize_trade_history()
            .iter()
            .map(|r| r.point)
            .sum::<Point>();
        let reward_point = self
            .market_reward_history()
            .iter()
            .map(|r| r.point)
            .sum::<Point>();
        WithPoint {
            user: self,
            point: reward_point - traded_point,
        }
    }
}

impl_user!(WithMarketRewardHistory);
impl_user_with_attrs!(WithMarketRewardHistory);
impl_user_with_pg!(WithMarketRewardHistory);
impl_user_with_point!(WithMarketRewardHistory);
impl_user_with_prize_trade_history!(WithMarketRewardHistory);
