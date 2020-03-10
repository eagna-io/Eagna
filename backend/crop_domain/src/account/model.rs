use arrayvec::{ArrayString, CapacityError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Account {
    pub name: AccountName,
}

impl Account {
    pub fn new(name: AccountName) -> Account {
        Account { name }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccountName(ArrayString<[u8; 64]>);

impl AccountName {
    pub fn from(s: &str) -> Result<Self, CapacityError<&str>> {
        Ok(AccountName(ArrayString::from(s)?))
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
