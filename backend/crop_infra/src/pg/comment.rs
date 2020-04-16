use super::{schema::comments, Connection};
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
    ///
    /// TODO
    /// test
    fn query_recent_by_poll_id(&self, poll_id: &Uuid) -> anyhow::Result<Vec<QueriedComment>> {
        Ok(comments::table
            .filter(comments::poll_id.is_not_distinct_from(poll_id))
            .select((
                comments::id,
                comments::contest_id,
                comments::poll_id,
                comments::account_id,
                comments::choice_name,
                comments::created_at,
                comments::content,
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
    pub id: &'a Uuid,
    pub contest_id: Option<&'a Uuid>,
    pub poll_id: Option<&'a Uuid>,
    pub account_id: &'a Uuid,
    pub choice_name: Option<&'a str>,
    pub created_at: &'a DateTime<Utc>,
    pub content: &'a str,
}

#[derive(Queryable)]
pub struct QueriedComment {
    pub id: Uuid,
    pub contest_id: Option<Uuid>,
    pub poll_id: Option<Uuid>,
    pub account_id: Uuid,
    pub choice_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub content: String,
}
