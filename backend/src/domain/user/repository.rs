pub mod access_token;

use crate::domain::point::Point;
use crate::domain::user::*;
use crate::infra::postgres::{user::NewUser as NewUserInfra, PostgresInfra};
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

    pub fn query_user(&self, user_id: &UserId) -> Result<Option<QueryUser>, failure::Error> {
        let user = match self.postgres.query_user(user_id.as_uuid())? {
            None => return Ok(None),
            Some(res) => res,
        };
        Ok(Some(QueryUser {
            id: *user_id,
            name: UserName::from_str(user.name)?,
            email: UserEmail::from_str(user.email)?,
            coin: user.coin as u32,
            point: Point::from(user.point as u32),
            is_admin: user.is_admin,
        }))
    }

    pub fn update_user<U>(&self, user: &U) -> Fallible<()>
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
    coin: u32,
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
    fn coin(&self) -> u32 {
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
    fn update_user(&self, pg: &dyn PostgresInfra) -> Fallible<()>;
}

impl<U> UpdatableUser for UserProvidedCoin<U>
where
    U: UserWithAttrs,
{
    fn update_user(&self, pg: &dyn PostgresInfra) -> Fallible<()> {
        todo!()
    }
}
