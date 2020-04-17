use crate::contest::{Contest, Updatable};
use crop_infra::pg::{contest::ContestTable, types::ContestStatus, Connection};

#[must_use]
pub struct Closed<C> {
    pub(crate) contest: C,
}

impl<C> Updatable for Closed<C>
where
    C: Contest,
{
    fn save(&self, conn: &Connection) -> anyhow::Result<()> {
        ContestTable::update_status(conn, &self.contest.id().0, ContestStatus::Closed)
    }
}
