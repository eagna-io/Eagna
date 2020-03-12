use crop_primitive::string::String as MyString;
use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub name: AccountName,
}

impl Account {
    pub fn new(name: AccountName) -> Account {
        Account { name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deref, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct AccountName(pub MyString);
