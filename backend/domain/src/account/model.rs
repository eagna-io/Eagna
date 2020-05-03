use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
    pub name: AccountName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct AccountId(pub Uuid);

impl AccountId {
    fn new() -> Self {
        AccountId(Uuid::new_v4())
    }

    pub fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountName(pub String);

impl AccountName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

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
