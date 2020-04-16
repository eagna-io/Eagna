use crate::account::AccountId;
use crate::contest::poll::{
    Choice, ChoiceName, Poll, PollId, PollStatus, WithAttrs, WithUserChoices,
};
use crate::contest::Updatable;
use chrono::{DateTime, Duration, Utc};
use crop_infra::pg::{poll::PollTable as _, Connection};
use std::collections::HashMap;

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

impl<P> Poll for Resolved<P>
where
    P: Poll,
{
    fn id(&self) -> PollId {
        self.poll.id()
    }
}

impl<P> WithAttrs for Resolved<P>
where
    P: WithAttrs,
{
    fn _status(&self) -> PollStatus {
        PollStatus::Closed
    }

    fn _title(&self) -> &str {
        self.poll._title()
    }

    fn _created_at(&self) -> &DateTime<Utc> {
        self.poll._created_at()
    }

    fn _duration(&self) -> Option<&Duration> {
        self.poll._duration()
    }

    fn _idx(&self) -> usize {
        self.poll._idx()
    }

    fn _choices(&self) -> &[Choice] {
        self.poll._choices()
    }

    fn _resolved_choice(&self) -> Option<&ChoiceName> {
        Some(&self.resolved)
    }
}

impl<P> WithUserChoices for Resolved<P>
where
    P: WithUserChoices,
{
    fn _user_choices(&self) -> &HashMap<AccountId, ChoiceName> {
        self.poll._user_choices()
    }
}
