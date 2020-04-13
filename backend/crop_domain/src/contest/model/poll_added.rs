use crate::contest::poll::{New, Poll as _};
use crate::contest::{Contest, Updatable};
use crop_infra::pg::Connection;

#[must_use]
pub struct PollAdded<C> {
    pub(crate) contest: C,
    pub(crate) poll: New,
}

impl<C> PollAdded<C> {
    pub fn contest(&self) -> &C {
        &self.contest
    }

    pub fn poll(&self) -> &New {
        &self.poll
    }
}

impl<C> Updatable for PollAdded<C>
where
    C: Contest,
{
    fn save(&self, conn: &Connection) -> anyhow::Result<()> {
        use crop_infra::pg::choice::{ChoiceTable, NewChoice};
        use crop_infra::pg::poll::{NewPoll, PollTable};

        let poll_id = self.poll.id().0;

        let new_poll = NewPoll {
            id: &poll_id,
            contest_id: &self.contest.id().0,
            title: self.poll.title(),
            created_at: self.poll.created_at(),
            duration_sec: self.poll.duration().map(|d| d.num_seconds() as i32),
        };
        PollTable::save(conn, new_poll)?;

        let new_choices = self
            .poll
            .choices()
            .iter()
            .map(|(choice_name, choice_color)| {
                NewChoice {
                    poll_id: &poll_id,
                    name: choice_name.0.as_str(),
                    color: choice_color.0.as_str(),
                    idx: 0, // TODO
                }
            })
            .collect();
        ChoiceTable::save_all(conn, &new_choices)
    }
}
