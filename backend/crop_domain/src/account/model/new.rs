use super::{Account, AccountId};

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
