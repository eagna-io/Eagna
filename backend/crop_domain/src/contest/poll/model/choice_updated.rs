use super::ChoiceName;
use crate::account::AccountId;

pub struct ChoiceUpdated<P> {
    pub(super) poll: P,
    pub(super) account: AccountId,
    pub(super) choice: ChoiceName,
}
