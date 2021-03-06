use super::{schema::account_choices, Connection};
use diesel::prelude::*;
use uuid::Uuid;

pub trait AccountChoiceTable {
    fn conn(&self) -> &Connection;

    fn upsert<'a>(&self, account_choice: NewAccountChoice<'a>) -> anyhow::Result<()> {
        diesel::insert_into(account_choices::table)
            .values(account_choice)
            .on_conflict((account_choices::account_id, account_choices::poll_id))
            .do_update()
            .set(account_choices::choice_name.eq(account_choice.choice_name))
            .execute(self.conn())?;
        Ok(())
    }

    fn query_by_poll_id(&self, poll_id: &Uuid) -> anyhow::Result<Vec<QueriedAccountChoice>> {
        Ok(account_choices::table
            .filter(account_choices::poll_id.eq(poll_id))
            .select((
                account_choices::poll_id,
                account_choices::account_id,
                account_choices::choice_name,
            ))
            .load::<QueriedAccountChoice>(self.conn())?)
    }
}

impl AccountChoiceTable for Connection {
    fn conn(&self) -> &Connection {
        self
    }
}

#[derive(Insertable, Clone, Copy)]
#[table_name = "account_choices"]
pub struct NewAccountChoice<'a> {
    pub poll_id: &'a Uuid,
    pub account_id: &'a Uuid,
    pub choice_name: &'a str,
}

#[derive(Queryable, Clone)]
pub struct QueriedAccountChoice {
    pub poll_id: Uuid,
    pub account_id: Uuid,
    pub choice_name: String,
}
