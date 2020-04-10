use super::{schema::contests, types::ContestStatus, Connection};
use chrono::{DateTime, Utc};
use diesel::{prelude::*, result::Error as PgError};
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
        match contests::table
            .filter(contests::id.eq(id))
            .select((
                contests::id,
                contests::status,
                contests::title,
                contests::category,
                contests::event_start_at,
            ))
            .first::<QueriedContest>(self.conn())
        {
            Ok(contest) => Ok(Some(contest)),
            Err(PgError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
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
