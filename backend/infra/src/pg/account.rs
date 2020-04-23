use super::{schema::accounts, Connection};
use diesel::prelude::*;
use domain::account::Account;
use uuid::Uuid;

pub(crate) fn save_account(conn: &Connection, account: &Account) -> anyhow::Result<()> {
    let new = NewAccount {
        id: account.id.as_ref(),
        name: account.name.as_str(),
    };
    diesel::insert_into(accounts::table)
        .values(new)
        .execute(conn)?;
    Ok(())
}

#[derive(Insertable)]
#[table_name = "accounts"]
struct NewAccount<'a> {
    id: &'a Uuid,
    name: &'a str,
}
