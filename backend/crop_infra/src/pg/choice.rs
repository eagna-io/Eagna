use super::{schema::choices, Connection};
use diesel::prelude::*;
use uuid::Uuid;

pub trait ChoiceTable {
    fn conn(&self) -> &Connection;

    #[allow(clippy::ptr_arg)]
    fn save_all<'a>(&self, choices: &Vec<NewChoice<'a>>) -> anyhow::Result<()> {
        diesel::insert_into(choices::table)
            .values(choices)
            .execute(self.conn())?;
        Ok(())
    }

    fn query_by_poll_id(&self, poll_id: &Uuid) -> anyhow::Result<Vec<QueriedChoice>> {
        Ok(choices::table
            .filter(choices::poll_id.eq(poll_id))
            .select((
                choices::poll_id,
                choices::name,
                choices::color,
                choices::idx,
            ))
            .load::<QueriedChoice>(self.conn())?)
    }
}

impl ChoiceTable for Connection {
    fn conn(&self) -> &Connection {
        self
    }
}

#[derive(Insertable)]
#[table_name = "choices"]
pub struct NewChoice<'a> {
    pub poll_id: &'a Uuid,
    pub name: &'a str,
    pub color: &'a str,
    pub idx: i32,
}

#[derive(Queryable)]
pub struct QueriedChoice {
    pub poll_id: Uuid,
    pub name: String,
    pub color: String,
    pub idx: i32,
}
