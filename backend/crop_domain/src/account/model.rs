use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
}

impl Account {
    pub fn new() -> Account {
        Account {
            id: AccountId::new(),
        }
    }

    pub fn from_uuid(uuid: Uuid) -> Account {
        Account {
            id: AccountId(uuid),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccountId(pub Uuid);

impl AccountId {
    pub fn new() -> AccountId {
        AccountId(Uuid::new_v4())
    }
}
