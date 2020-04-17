use crate::account::AccountId;
use crate::contest::poll::{ChoiceName, Poll};
use crate::contest::Updatable;
use crop_infra::pg::{
    account_choice::{AccountChoiceTable, NewAccountChoice},
    Connection,
};

pub struct ChoiceUpdated<P> {
    pub(super) poll: P,
    pub(super) account_id: AccountId,
    pub(super) choice: ChoiceName,
}

impl<P> Updatable for ChoiceUpdated<P>
where
    P: Poll,
{
    fn save(&self, conn: &Connection) -> anyhow::Result<()> {
        let record = NewAccountChoice {
            poll_id: &self.poll.id().0,
            account_id: &self.account_id,
            choice_name: self.choice.0.as_str(),
        };
        AccountChoiceTable::upsert(conn, record)
    }
}
