use super::{schema::admins, Connection};
use diesel::{prelude::*, result::Error as PgError};
use uuid::Uuid;

pub trait AdminTable {
    fn conn(&self) -> &Connection;

    fn save<'a>(&self, new_admin: NewAdmin<'a>) -> anyhow::Result<()> {
        diesel::insert_into(admins::table)
            .values(new_admin)
            .execute(self.conn())?;
        Ok(())
    }

    fn query_credentials_by_email(
        &self,
        email: &str,
    ) -> anyhow::Result<Option<QueriedAdminCredentials>> {
        match admins::table
            .filter(admins::email.eq(email))
            .select((admins::id, admins::credential, admins::salt))
            .first::<QueriedAdminCredentials>(self.conn())
        {
            Ok(cred) => Ok(Some(cred)),
            Err(PgError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Insertable)]
#[table_name = "admins"]
pub struct NewAdmin<'a> {
    pub id: &'a Uuid,
    pub email: &'a str,
    pub cred: &'a [u8],
    pub salt: &'a [u8],
}

#[derive(Queryable)]
pub struct QueriedAdminCredentials {
    pub id: Uuid,
    pub cred: Vec<u8>,
    pub salt: Vec<u8>,
}
