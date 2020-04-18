use super::{schema::contests, types::ContestStatus, Connection};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

pub trait ContestTable {
    fn conn(&self) -> &Connection;

    fn save<'a>(&self, contest: NewContest<'a>) -> anyhow::Result<()> {
        diesel::insert_into(contests::table)
            .values(contest)
            .execute(self.conn())?;
        Ok(())
    }

    fn query_by_id(&self, id: &Uuid) -> anyhow::Result<Option<QueriedContest>> {
        Ok(contests::table
            .filter(contests::id.eq(id))
            .select((
                contests::id,
                contests::status,
                contests::title,
                contests::category,
                contests::event_start_at,
            ))
            .first::<QueriedContest>(self.conn())
            .optional()?)
    }

    fn query_not_archived(&self) -> anyhow::Result<Vec<QueriedContest>> {
        Ok(contests::table
            .filter(contests::status.ne(ContestStatus::Archived))
            .select((
                contests::id,
                contests::status,
                contests::title,
                contests::category,
                contests::event_start_at,
            ))
            .load::<QueriedContest>(self.conn())?)
    }

    fn update_status(&self, id: &Uuid, new_status: ContestStatus) -> anyhow::Result<()> {
        diesel::update(contests::table.filter(contests::id.eq(id)))
            .set(contests::status.eq(new_status))
            .execute(self.conn())?;
        Ok(())
    }
}

impl ContestTable for Connection {
    fn conn(&self) -> &Connection {
        self
    }
}

#[derive(Insertable)]
#[table_name = "contests"]
pub struct NewContest<'a> {
    pub id: &'a Uuid,
    pub status: ContestStatus,
    pub title: &'a str,
    pub category: &'a str,
    pub event_start_at: Option<&'a DateTime<Utc>>,
}

#[derive(Queryable)]
pub struct QueriedContest {
    pub id: Uuid,
    pub status: ContestStatus,
    pub title: String,
    pub category: String,
    pub event_start_at: Option<DateTime<Utc>>,
}
