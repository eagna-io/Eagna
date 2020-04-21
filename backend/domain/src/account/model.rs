use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
    pub name: AccountName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccountId(Uuid);

impl AccountId {
    fn new() -> Self {
        AccountId(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountName(String);

impl Account {
    pub fn new(name: String) -> anyhow::Result<Account> {
        if name.is_empty() {
            return Err(anyhow::anyhow!("Account name is empty"));
        }

        Ok(Account {
            id: AccountId::new(),
            name: AccountName(name),
        })
    }
}
