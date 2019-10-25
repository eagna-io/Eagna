use super::*;
use crate::infra::postgres::{user::NewUser, PostgresInfra};

#[derive(From)]
pub struct UserRepository<'a> {
    postgres: &'a dyn PostgresInfra,
}

impl<'a> UserRepository<'a> {
    /// ユーザーを新規作成したとき、それを infra に記録する。
    /// PointHistory は記録しない。
    pub fn save_new_user(&self, user: &User) -> Result<(), failure::Error> {
        let new_user = NewUser {
            id: user.id.as_str(),
            name: user.name.as_str(),
            email: user.email.as_str(),
        };
        self.postgres.save_user(new_user)
    }

    pub fn query_user(&self, user_id: &UserId) -> Result<Option<User>, failure::Error> {
        let user = match self.postgres.query_user(user_id.as_str())? {
            None => return Ok(None),
            Some(res) => res,
        };
        let user_point = self.postgres.query_user_point(user_id.as_str())?;
        Ok(Some(User {
            id: *user_id,
            name: UserName::from_str(user.name)?,
            email: UserEmail::from_str(user.email)?,
            is_admin: user.is_admin,
            point: Point::from(user_point),
        }))
    }
}
