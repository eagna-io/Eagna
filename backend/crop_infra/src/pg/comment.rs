use super::{
    schema::{accounts, choices, comments},
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
    ///
    /// TODO
    /// test
    fn query_recent_by_poll_id(&self, poll_id: &Uuid) -> anyhow::Result<Vec<QueriedComment>> {
        Ok(comments::table
            .inner_join(accounts::table)
            .left_join(
                choices::table.on(comments::choice_name
                    .is_not_distinct_from(choices::name.nullable())
                    .and(comments::poll_id.eq(choices::poll_id))),
            )
            .filter(comments::poll_id.eq(poll_id))
            .select((
                comments::poll_id,
                accounts::id,
                accounts::name,
                choices::name.nullable(),
                choices::color.nullable(),
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
    pub choice_name: Option<&'a str>,
    pub content: &'a str,
}

#[derive(Queryable)]
pub struct QueriedComment {
    pub poll_id: Uuid,
    pub account_id: Uuid,
    pub account_name: String,
    pub choice_name: Option<String>,
    pub choice_color: Option<String>,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
