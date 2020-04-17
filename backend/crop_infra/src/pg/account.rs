use super::{schema::accounts, Connection};
use diesel::prelude::*;
use uuid::Uuid;

pub trait AccountTable {
    fn conn(&self) -> &Connection;

    fn save<'a>(&self, new_account: NewAccount<'a>) -> anyhow::Result<()> {
        diesel::insert_into(accounts::table)
            .values(new_account)
            .execute(self.conn())?;
        Ok(())
    }

    fn query_by_id(&self, id: &Uuid) -> anyhow::Result<Option<QueriedAccount>> {
        Ok(accounts::table
            .filter(accounts::id.eq(id))
            .select((accounts::id, accounts::name))
            .first::<QueriedAccount>(self.conn())
            .optional()?)
    }
}

impl AccountTable for Connection {
    fn conn(&self) -> &Connection {
        self
    }
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
}

#[derive(Queryable)]
pub struct QueriedAccount {
    pub id: Uuid,
    pub name: String,
}
