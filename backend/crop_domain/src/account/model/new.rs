use crate::account::{Account, AccountId, Updatable};
use crop_infra::pg::{
    account::{AccountTable, NewAccount},
    Connection,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct New {
    id: AccountId,
    name: String,
}

impl New {
    pub(super) fn new(name: String) -> New {
        New {
            id: AccountId::new(),
            name,
        }
    }
}

impl Account for New {
    fn id(&self) -> &AccountId {
        &self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Updatable for New {
    fn save(&self, conn: &Connection) -> anyhow::Result<()> {
        let new = NewAccount {
            id: self.id(),
            name: self.name(),
        };
        AccountTable::save(conn, new)
    }
}
