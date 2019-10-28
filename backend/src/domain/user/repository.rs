use super::*;
use crate::infra::postgres::{
    types::PrizeTradeStatus as InfraPrizeTradeStatus, user::NewUser as InfraNewUser, PostgresInfra,
};

#[derive(From)]
pub struct UserRepository<'a> {
    postgres: &'a dyn PostgresInfra,
}

impl<'a> UserRepository<'a> {
    /// ユーザーを新規作成したとき、それを infra に記録する。
    /// PointHistory は記録しない。
    pub fn save_new_user(&self, user: &NewUser) -> Result<(), failure::Error> {
        let new_user = InfraNewUser {
            id: user.id.as_str(),
            name: user.name.as_str(),
            email: user.email.as_str(),
        };
        self.postgres.save_user(new_user)
    }

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
                prize_id: record.prize_id,
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
}

impl<'a> UserWithPg for QueryUser<'a> {
    fn pg(&self) -> &dyn PostgresInfra {
        self.pg
    }
}

pub struct WithPoint<U> {
    user: U,
    point: Point,
}

impl<U: User> User for WithPoint<U> {
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

impl<U: User> UserWithPoint for WithPoint<U> {
    fn point(&self) -> Point {
        self.point
    }
}

impl<U: UserWithPg> UserWithPg for WithPoint<U> {
    fn pg(&self) -> &dyn PostgresInfra {
        self.user.pg()
    }

    fn with_point(self) -> Result<WithPoint<Self>, failure::Error> {
        Ok(WithPoint {
            point: self.point,
            user: self,
        })
    }
}

pub struct WithPrizeTradeHistory<U> {
    user: U,
    prize_trade_history: Vec<PrizeTradeRecord>,
}

impl<U: User> User for WithPrizeTradeHistory<U> {
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

impl<U: User> UserWithPrizeTradeHistory for WithPrizeTradeHistory<U> {
    fn prize_trade_history(&self) -> &Vec<PrizeTradeRecord> {
        &self.prize_trade_history
    }
}

impl<U: UserWithPoint> UserWithPoint for WithPrizeTradeHistory<U> {
    fn point(&self) -> Point {
        self.user.point()
    }
}

impl<U: UserWithPg> UserWithPg for WithPrizeTradeHistory<U> {
    fn pg(&self) -> &dyn PostgresInfra {
        self.user.pg()
    }

    fn with_prize_trade_history(self) -> Result<WithPrizeTradeHistory<Self>, failure::Error> {
        Ok(WithPrizeTradeHistory {
            prize_trade_history: self.prize_trade_history.clone(),
            user: self,
        })
    }
}
