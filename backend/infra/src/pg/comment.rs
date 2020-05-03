use super::{
    schema::comments,
    {Connection, Postgres, GLOBAL_PG},
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use domain::comment::Comment;
use uuid::Uuid;

pub struct CommentRepository {
    pg: Postgres,
}

impl CommentRepository {
    pub fn new() -> Self {
        CommentRepository {
            pg: GLOBAL_PG.as_ref().clone(),
        }
    }
}

#[async_trait]
impl domain::comment::CommentRepository for CommentRepository {
    async fn save(&mut self, comment: Comment) -> anyhow::Result<()> {
        self.pg
            .try_with_conn(move |conn| save(&conn, &comment))
            .await
    }
}

pub fn save(conn: &Connection, comment: &Comment) -> anyhow::Result<()> {
    let new = NewComment {
        id: &comment.id.0,
        contest_id: &comment.contest_id.0,
        account_id: &comment.account_id.0,
        answer_id: comment.answer_id.as_ref().map(|ans| &ans.0),
        content: comment.comment.as_str(),
        created_at: &comment.created_at,
    };
    diesel::insert_into(comments::table)
        .values(new)
        .execute(conn)?;
    Ok(())
}

#[derive(Insertable)]
#[table_name = "comments"]
struct NewComment<'a> {
    id: &'a Uuid,
    contest_id: &'a Uuid,
    account_id: &'a Uuid,
    answer_id: Option<&'a Uuid>,
    content: &'a str,
    created_at: &'a DateTime<Utc>,
}
