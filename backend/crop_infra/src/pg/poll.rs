use super::{schema::polls, types::PollStatus, Connection};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

pub trait PollTable {
    fn conn(&self) -> &Connection;

    fn save<'a>(&self, poll: NewPoll<'a>) -> anyhow::Result<()> {
        diesel::insert_into(polls::table)
            .values(poll)
            .execute(self.conn())?;
        Ok(())
    }

    fn query_not_resolved_by_contest_id(&self, id: &Uuid) -> anyhow::Result<Option<QueriedPoll>> {
        Ok(polls::table
            .filter(
                polls::contest_id
                    .eq(id)
                    .and(polls::resolved_choice_name.is_null()),
            )
            .select((
                polls::id,
                polls::contest_id,
                polls::status,
                polls::title,
                polls::created_at,
                polls::duration_sec,
                polls::resolved_at,
                polls::resolved_choice_name,
            ))
            .first::<QueriedPoll>(self.conn())
            .optional()?)
    }

    fn update_status(&self, id: &Uuid, new_status: PollStatus) -> anyhow::Result<()> {
        diesel::update(polls::table.filter(polls::id.eq(id)))
            .set(polls::status.eq(new_status))
            .execute(self.conn())?;
        Ok(())
    }

    fn update_resolved_choice_name(
        &self,
        id: &Uuid,
        resolved_choice_name: &str,
    ) -> anyhow::Result<()> {
        diesel::update(polls::table.filter(polls::id.eq(id)))
            .set((
                polls::resolved_choice_name.eq(resolved_choice_name),
                polls::resolved_at.eq(Utc::now()),
            ))
            .execute(self.conn())?;
        Ok(())
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
    pub created_at: &'a DateTime<Utc>,
    pub duration_sec: Option<i32>,
}

#[derive(Queryable)]
pub struct QueriedPoll {
    pub id: Uuid,
    pub contest_id: Uuid,
    pub status: PollStatus,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub duration_sec: Option<i32>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_choice_name: Option<String>,
}
