use super::ChoiceName;
use crate::account::model::AccountName;

pub struct ChoiceUpdated<P> {
    pub(super) poll: P,
    pub(super) account: AccountName,
    pub(super) choice: ChoiceName,
}
