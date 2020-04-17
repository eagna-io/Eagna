use crate::account::{Account, AccountId, Queryable, WithAttrs};
use crop_infra::pg::{account::AccountTable, Connection};

pub struct BriefAccount {
    id: AccountId,
    name: String,
}

impl Account for BriefAccount {
    fn id(&self) -> &AccountId {
        &self.id
    }
}

impl WithAttrs for BriefAccount {
    fn _name(&self) -> &str {
        self.name.as_str()
    }
}

impl Queryable for BriefAccount {
    fn query_by_id(conn: &Connection, id: &AccountId) -> anyhow::Result<Option<Self>> {
        if let Some(queried) = AccountTable::query_by_id(conn, &id.0)? {
            Ok(Some(BriefAccount {
                id: AccountId(queried.id),
                name: queried.name,
            }))
        } else {
            Ok(None)
        }
    }
}
