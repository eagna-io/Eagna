use super::{
    schema::{accounts, comments},
    Connection,
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

pub trait CommentTable {
    fn conn(&self) -> &Connection;

    fn save<'a>(&self, comment: NewComment<'a>) -> anyhow::Result<()> {
        diesel::insert_into(comments::table)
            .values(comment)
            .execute(self.conn())?;
        Ok(())
    }

    /// 直近100件のコメントを取得する
    fn query_recent_by_poll_id(&self, poll_id: &Uuid) -> anyhow::Result<Vec<QueriedComment>> {
        Ok(comments::table
            .inner_join(accounts::table)
            .filter(comments::poll_id.eq(poll_id))
            .select((
                comments::poll_id,
                comments::account_id,
                accounts::name,
                comments::content,
                comments::created_at,
            ))
            .order(comments::created_at.desc())
            .limit(100)
            .load::<QueriedComment>(self.conn())?)
    }
}

impl CommentTable for Connection {
    fn conn(&self) -> &Connection {
        self
    }
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub poll_id: &'a Uuid,
    pub account_id: &'a Uuid,
    pub content: &'a str,
}

#[derive(Queryable)]
pub struct QueriedComment {
    pub poll_id: Uuid,
    pub account_id: Uuid,
    pub account_name: String,
    pub contest: String,
    pub created_at: DateTime<Utc>,
}
