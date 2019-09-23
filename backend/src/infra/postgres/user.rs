use super::{
    schema::{user_prize_trade_history, user_reward_point_history, users},
    Postgres,
};
use chrono::{DateTime, Utc};
use diesel::{dsl::sum, prelude::*, result::Error as PgError};

pub trait PostgresUserInfra {
    fn save_user<'a>(&self, new_user: NewUser<'a>) -> Result<(), failure::Error>;

    fn query_user(&self, user_id: &str) -> Result<Option<QueryUser>, failure::Error>;

    fn query_user_point(&self, user_id: &str) -> Result<u32, failure::Error>;
}

pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
}

pub struct QueryUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

impl PostgresUserInfra for Postgres {
    fn save_user<'a>(&self, new_user: NewUser<'a>) -> Result<(), failure::Error> {
        diesel::insert_into(users::table)
            .values(InsertableUser {
                fb_uid: new_user.id,
                name: new_user.name,
                email: new_user.email,
            })
            .execute(&self.conn)?;
        Ok(())
    }

    fn query_user(&self, user_id: &str) -> Result<Option<QueryUser>, failure::Error> {
        match users::table
            .filter(users::fb_uid.eq(user_id))
            .first::<QueryableUser>(&self.conn)
        {
            Ok(query_res) => Ok(Some(QueryUser {
                id: query_res.fb_uid,
                name: query_res.name,
                email: query_res.email,
                is_admin: query_res.is_admin,
            })),
            Err(PgError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Userの現在の保有コインを取得する。
    /// Userが存在しない場合は0を返す。
    fn query_user_point(&self, user_id: &str) -> Result<u32, failure::Error> {
        let earned = user_reward_point_history::table
            .filter(user_reward_point_history::columns::user_id.eq(user_id))
            .select(sum(user_reward_point_history::columns::point))
            .first::<Option<i64>>(&self.conn)?
            .unwrap_or(0);
        let consumed = user_prize_trade_history::table
            .filter(user_prize_trade_history::columns::user_id.eq(user_id))
            .select(sum(user_prize_trade_history::columns::price))
            .first::<Option<i64>>(&self.conn)?
            .unwrap_or(0);
        assert!(earned > consumed);
        return Ok((earned - consumed) as u32);
    }
}

#[derive(Insertable)]
#[table_name = "users"]
struct InsertableUser<'a> {
    fb_uid: &'a str,
    name: &'a str,
    email: &'a str,
}

#[derive(Queryable)]
struct QueryableUser {
    fb_uid: String,
    name: String,
    email: String,
    is_admin: bool,
    _created: DateTime<Utc>,
}
