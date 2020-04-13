use crate::contest::poll::{ChoiceName, Poll};
use crate::contest::Updatable;
use crop_infra::pg::{poll::PollTable as _, Connection};

#[must_use]
pub struct Resolved<P> {
    pub(super) poll: P,
    pub(super) resolved: ChoiceName,
}

impl<P> Updatable for Resolved<P>
where
    P: Poll,
{
    fn save(&self, conn: &Connection) -> anyhow::Result<()> {
        conn.update_resolved_choice_name(&self.poll.id().0, self.resolved.0.as_str())
    }
}
