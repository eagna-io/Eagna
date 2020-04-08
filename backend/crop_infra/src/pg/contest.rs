use super::{schema::contests, Connection};
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
            .select((contests::id,))
            .first::<QueriedContest>(self.conn())
        {
            Ok(contest) => Ok(Some(contest)),
            Err(PgError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Insertable)]
#[table_name = "contests"]
pub struct NewContest<'a> {
    pub id: &'a Uuid,
}

#[derive(Queryable)]
pub struct QueriedContest {
    pub id: Uuid,
}
