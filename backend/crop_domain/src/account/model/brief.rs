use super::{Account, AccountId, WithAttrs};

pub struct BriefAccount {
    id: AccountId,
    name: String,
}

impl Account for BriefAccount {
    fn id(&self) -> &AccountId {
        &self.id
    }
}

impl WithAttrs for BriefAccount {
    fn _name(&self) -> &str {
        self.name.as_str()
    }
}
