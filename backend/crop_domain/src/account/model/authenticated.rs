use super::{Account, AccountId};

pub struct Authenticated {
    pub id: AccountId,
}

impl Account for Authenticated {
    fn id(&self) -> &AccountId {
        &self.id
    }
}
