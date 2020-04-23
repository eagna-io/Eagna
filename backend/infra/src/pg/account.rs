use super::{
    schema::accounts,
    {Connection, Postgres, GLOBAL_PG},
};
use async_trait::async_trait;
use diesel::prelude::*;
use domain::account::Account;
use uuid::Uuid;

pub struct AccountRepository {
    pg: Postgres,
}

impl AccountRepository {
    pub fn new() -> Self {
        AccountRepository {
            pg: GLOBAL_PG.as_ref().clone(),
        }
    }
}

#[async_trait]
impl domain::account::AccountRepository for AccountRepository {
    async fn save(&mut self, account: Account) -> anyhow::Result<()> {
        self.pg
            .try_with_conn(move |conn| save(&conn, &account))
            .await
    }
}

fn save(conn: &Connection, account: &Account) -> anyhow::Result<()> {
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
