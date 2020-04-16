use super::{
    ChoiceColor, ChoiceName, Poll, PollId, PollStatus, WithAttrs, WithComments, WithUserChoices,
};
use crate::account::AccountId;
use crate::contest::Updatable;
use chrono::{DateTime, Duration, Utc};
use crop_infra::pg::{poll::PollTable, Connection};
use std::collections::HashMap;

#[must_use]
pub struct Closed<P> {
    pub poll: P,
}

impl<P> Poll for Closed<P>
where
    P: Poll,
{
    fn id(&self) -> PollId {
        self.poll.id()
    }
}

impl<P> WithAttrs for Closed<P>
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

    fn _choices(&self) -> &HashMap<ChoiceName, ChoiceColor> {
        self.poll._choices()
    }

    fn _resolved_choice(&self) -> Option<&ChoiceName> {
        self.poll._resolved_choice()
    }
}

impl<P> WithUserChoices for Closed<P>
where
    P: WithUserChoices,
{
    fn _user_choices(&self) -> &HashMap<AccountId, ChoiceName> {
        self.poll._user_choices()
    }
}

impl<P> WithComments for Closed<P>
where
    P: WithComments,
{
    type Comment = P::Comment;

    fn _comments(&self) -> &[Self::Comment] {
        self.poll._comments()
    }
}

impl<P> Updatable for Closed<P>
where
    P: Poll,
{
    fn save(&self, conn: &Connection) -> anyhow::Result<()> {
        PollTable::update_status(conn, &self.id().0, PollStatus::Closed)
    }
}
