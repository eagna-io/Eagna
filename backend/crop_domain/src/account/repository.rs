use crate::account::AccountId;
use crop_infra::pg::Connection;

pub trait AccountRepository {
    fn conn(&self) -> &Connection;

    fn save<A>(&self, account: &A) -> anyhow::Result<()>
    where
        A: Updatable,
    {
        account.save(self.conn())
    }
}

impl AccountRepository for Connection {
    fn conn(&self) -> &Connection {
        self
    }
}

pub trait Updatable {
    fn save(&self, conn: &Connection) -> anyhow::Result<()>;
}

pub trait Queryable: Sized {
    fn query_by_id(conn: &Connection, id: &AccountId) -> anyhow::Result<Option<Self>>;
}
