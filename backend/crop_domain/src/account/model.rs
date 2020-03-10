use crop_primitive::string::String;
use derive_more::Deref;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub name: AccountName,
}

impl Account {
    pub fn new(name: AccountName) -> Account {
        Account { name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deref)]
pub struct AccountName(pub String);
