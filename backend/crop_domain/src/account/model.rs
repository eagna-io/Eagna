use crop_primitive::string::String;
use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub name: AccountName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct AccountName(pub String);

impl Account {
    pub fn new(name: AccountName) -> Account {
        Account { name }
    }
}
