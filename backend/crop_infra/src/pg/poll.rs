use super::{schema::polls, Connection};
use chrono::{DateTime, Utc};
use diesel::{prelude::*, result::Error as PgError};
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
            .filter(polls::id.eq(id))
            .select((
                polls::id,
                polls::contest_id,
                polls::title,
                polls::created_at,
                polls::end_at,
                polls::resolved_choice_id,
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
    pub end_at: &'a DateTime<Utc>,
}

#[derive(Queryable)]
pub struct QueriedPoll {
    pub id: Uuid,
    pub contest_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub resolved_choice_id: Option<String>,
}
