use super::{AccessToken, Account, AccountId};

pub struct Authenticated {
    pub id: AccountId,
}

impl Account for Authenticated {
    fn id(&self) -> &AccountId {
        &self.id
    }
}

impl From<AccessToken> for Authenticated {
    fn from(token: AccessToken) -> Authenticated {
        Authenticated {
            id: token.account_id,
        }
    }
}
