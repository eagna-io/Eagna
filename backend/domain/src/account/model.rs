use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
    pub name: AccountName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccountId(Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountName(String);
