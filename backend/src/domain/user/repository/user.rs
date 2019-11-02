use crate::domain::user::*;
use crate::domain::{market::MarketId, point::Point, prize::PrizeId};
use crate::infra::postgres::{
    types::PrizeTradeStatus as InfraPrizeTradeStatus, user::NewPrizeTradeRecord, PostgresInfra,
};
use failure::Fallible;

#[derive(From)]
pub struct UserRepository<'a> {
    postgres: &'a dyn PostgresInfra,
}

impl<'a> UserRepository<'a> {
    pub fn query_user(&self, user_id: &UserId) -> Result<Option<QueryUser<'a>>, failure::Error> {
        let user = match self.postgres.query_user(user_id.as_str())? {
            None => return Ok(None),
            Some(res) => res,
        };
        Ok(Some(QueryUser {
            id: *user_id,
            name: UserName::from_str(user.name)?,
            email: UserEmail::from_str(user.email)?,
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
            .save_user_prize_trade_record(user.id().as_str(), new_prize_trade_record)
    }
}

pub struct QueryUser<'a> {
    id: UserId,
    name: UserName,
    email: UserEmail,
    is_admin: bool,

    pg: &'a dyn PostgresInfra,
}

impl<'a> User for QueryUser<'a> {
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
        self.is_admin
    }
}

pub trait UserWithPg: User + Sized {
    fn pg(&self) -> &dyn PostgresInfra;

    fn with_point(self) -> Result<WithPoint<Self>, failure::Error> {
        let point = self.pg().query_user_point(self.id().as_str())?;
        Ok(WithPoint {
            user: self,
            point: Point::from(point),
        })
    }

    fn with_prize_trade_history(self) -> Result<WithPrizeTradeHistory<Self>, failure::Error> {
        let history = self
            .pg()
            .query_user_prize_trade_records(self.id().as_str())?
            .into_iter()
            .map(|record| PrizeTradeRecord {
                id: record.id,
                prize_id: PrizeId::from(record.prize_id),
                point: Point::from(record.point),
                time: record.time,
                status: match record.status {
                    InfraPrizeTradeStatus::Requested => PrizeTradeStatus::Requested,
                    InfraPrizeTradeStatus::Processed => PrizeTradeStatus::Processed,
                },
            })
            .collect();
        Ok(WithPrizeTradeHistory {
            user: self,
            prize_trade_history: history,
        })
    }

    fn with_market_reward_history(self) -> Result<WithMarketRewardHistory<Self>, failure::Error> {
        let history = self
            .pg()
            .query_user_market_reward_records(self.id().as_str())?
            .into_iter()
            .map(|record| MarketRewardRecord {
                market_id: MarketId::from(record.market_id),
                point: Point::from(record.point),
            })
            .collect();
        Ok(WithMarketRewardHistory {
            user: self,
            market_reward_records: history,
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
impl_user_with_pg!(WithMarketRewardHistory);
impl_user_with_point!(WithMarketRewardHistory);
impl_user_with_prize_trade_history!(WithMarketRewardHistory);
