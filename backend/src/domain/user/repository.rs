use super::{User, UserEmail, UserId, UserName};
use crate::infra::postgres::{user::NewUser, PostgresInfra};

#[derive(From)]
pub struct UserRepository<'a> {
    postgres: &'a dyn PostgresInfra,
}

impl<'a> UserRepository<'a> {
    pub fn save_user(&self, user: &User) -> Result<(), failure::Error> {
        let new_user = NewUser {
            id: user.id.as_str(),
            name: user.name.as_str(),
            email: user.email.as_str(),
        };
        self.postgres.save_user(new_user)
    }

    pub fn query_user(&self, user_id: &UserId) -> Result<Option<User>, failure::Error> {
        match self.postgres.query_user(user_id.as_str())? {
            None => Ok(None),
            Some(res) => Ok(Some(User {
                id: *user_id,
                name: UserName::from(res.name),
                email: UserEmail::from(res.email),
                is_admin: res.is_admin,
            })),
        }
    }
}
