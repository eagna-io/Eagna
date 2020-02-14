use crate::market::{num::AmountCoin, services::manager::UserPointIncreased};
use crate::point::Point;
use crate::user::*;
use crop_infra::postgres::{user::NewUser as NewUserInfra, PostgresInfra};
use getset::{CopyGetters, Getters};

#[derive(From)]
pub struct UserRepository<'a> {
    postgres: &'a dyn PostgresInfra,
}

impl<'a> UserRepository<'a> {
    pub fn save_user(&self, new_user: &NewUser) -> anyhow::Result<()> {
        self.postgres.save_user(NewUserInfra {
            id: *new_user.id().as_uuid(),
            name: new_user.name().as_str(),
            email: new_user.email().as_str(),
            credential: &new_user.cred().cred()[..],
            salt: &new_user.cred().salt()[..],
        })
    }

    pub fn query_user(&self, user_id: &UserId) -> anyhow::Result<Option<QueryUser>> {
        let user = match self.postgres.query_user(user_id.as_uuid())? {
            None => return Ok(None),
            Some(res) => res,
        };
        Ok(Some(QueryUser {
            id: *user_id,
            name: UserName::from_string(user.name)?,
            email: UserEmail::from_string(user.email)?,
            coin: AmountCoin::from(user.coin),
            point: Point::from(user.point as u32),
            is_admin: user.is_admin,
        }))
    }

    pub fn update_user<U>(&self, user: &U) -> anyhow::Result<()>
    where
        U: UpdatableUser,
    {
        user.update_user(self.postgres)
    }
}

/*
 * ===================
 * QueryUser
 * ===================
 */
#[derive(Getters, CopyGetters)]
pub struct QueryUser {
    #[get = "pub"]
    id: UserId,
    #[get = "pub"]
    name: UserName,
    #[get = "pub"]
    email: UserEmail,
    #[get_copy = "pub"]
    coin: AmountCoin,
    #[get_copy = "pub"]
    point: Point,
    #[get_copy = "pub"]
    is_admin: bool,
}

impl User for QueryUser {
    fn id(&self) -> &UserId {
        &self.id
    }
}

impl UserWithAttrs for QueryUser {
    fn name(&self) -> &UserName {
        &self.name
    }
    fn email(&self) -> &UserEmail {
        &self.email
    }
    fn coin(&self) -> AmountCoin {
        self.coin
    }
    fn point(&self) -> Point {
        self.point
    }
    fn is_admin(&self) -> bool {
        self.is_admin
    }
}

/*
 * =====================
 * UpdatableUser
 * =====================
 */
pub trait UpdatableUser {
    fn update_user(&self, pg: &dyn PostgresInfra) -> anyhow::Result<()>;
}

impl<U> UpdatableUser for UserCoinUpdated<U>
where
    U: UserWithAttrs,
{
    fn update_user(&self, pg: &dyn PostgresInfra) -> anyhow::Result<()> {
        pg.update_user_coin(self.id().as_uuid(), self.coin().as_i32() as u32)
    }
}

impl UpdatableUser for UserPointIncreased {
    fn update_user(&self, pg: &dyn PostgresInfra) -> anyhow::Result<()> {
        pg.add_assign_user_point(self.id().as_uuid(), self.point_increased().as_i32())
    }
}
