use super::{
    schema::{choices, polls},
    Connection,
};
use chrono::{DateTime, Utc};
use diesel::{
    expression_methods::{NullableExpressionMethods as _, PgExpressionMethods as _},
    prelude::*,
    result::Error as PgError,
};
use uuid::Uuid;

pub trait PollTable {
    fn conn(&self) -> &Connection;

    fn save<'a>(&self, poll: NewPoll<'a>) -> anyhow::Result<()> {
        diesel::insert_into(polls::table)
            .values(poll)
            .execute(self.conn())?;
        Ok(())
    }

    fn query_by_id(&self, id: &Uuid) -> anyhow::Result<Option<QueriedPoll>> {
        match polls::table
            .left_join(
                choices::table
                    .on(polls::resolved_choice_id.is_not_distinct_from(choices::id.nullable())),
            )
            .filter(polls::id.eq(id))
            .select((
                polls::id,
                polls::contest_id,
                polls::title,
                polls::duration_sec,
                polls::created_at,
                polls::closed_at,
                choices::name.nullable(),
                choices::color.nullable(),
                choices::idx.nullable(),
            ))
            .first::<QueriedPoll>(self.conn())
        {
            Ok(poll) => Ok(Some(poll)),
            Err(PgError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

impl PollTable for Connection {
    fn conn(&self) -> &Connection {
        self
    }
}

#[derive(Insertable)]
#[table_name = "polls"]
pub struct NewPoll<'a> {
    pub id: &'a Uuid,
    pub contest_id: &'a Uuid,
    pub title: &'a str,
    pub duration_sec: Option<i32>,
}

#[derive(Queryable)]
pub struct QueriedPoll {
    pub id: Uuid,
    pub contest_id: Uuid,
    pub title: String,
    pub duration_sec: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub resolved_choice_name: Option<String>,
    pub resolved_choice_color: Option<String>,
    pub resolved_choice_idx: Option<i32>,
}
