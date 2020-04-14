use super::{schema::account_choices, Connection};
use diesel::prelude::*;
use uuid::Uuid;

pub trait AccountChoiceTable {
    fn conn(&self) -> &Connection;

    fn upsert<'a>(&self, account_choice: NewAccountChoice<'a>) -> anyhow::Result<()> {
        diesel::insert_into(account_choices::table)
            .values(account_choice)
            .on_conflict((account_choices::account_id, account_choices::choice_name))
            .do_update()
            .set(account_choices::choice_name.eq(account_choice.choice_name))
            .execute(self.conn())?;
        Ok(())
    }
}

#[derive(Insertable, Clone, Copy)]
#[table_name = "account_choices"]
pub struct NewAccountChoice<'a> {
    pub poll_id: &'a Uuid,
    pub account_id: &'a Uuid,
    pub choice_name: &'a str,
}
