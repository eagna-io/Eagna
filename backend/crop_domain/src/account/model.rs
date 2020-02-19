use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccountId(Uuid);
