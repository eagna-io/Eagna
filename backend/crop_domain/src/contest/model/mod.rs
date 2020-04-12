use crate::contest::poll::model::Poll;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

mod brief;
mod detailed;

pub use brief::BriefContest;
pub use detailed::DetailedContest;

pub trait Contest {
    fn new(
        title: String,
        category: String,
        event_start_at: Option<DateTime<Utc>>,
    ) -> DetailedContest {
        DetailedContest {
            id: ContestId::new(),
            status: ContestStatus::Upcoming,
            title,
            category,
            event_start_at,
            poll: None,
        }
    }

    fn id(&self) -> ContestId;

    fn status(&self) -> ContestStatus
    where
        Self: WithAttrs,
    {
        self._status()
    }

    fn title(&self) -> &str
    where
        Self: WithAttrs,
    {
        self._title()
    }

    fn category(&self) -> &str
    where
        Self: WithAttrs,
    {
        self._category()
    }

    fn event_start_at(&self) -> Option<&DateTime<Utc>>
    where
        Self: WithAttrs,
    {
        self._event_start_at()
    }

    fn current_poll(&self) -> Option<&Poll>
    where
        Self: WithPoll,
    {
        self._current_poll()
    }

    fn add_poll(&self, poll: Poll) -> anyhow::Result<PollAdded<&Self>>
    where
        Self: WithAttrs,
    {
        if self.status() == ContestStatus::Open {
            Ok(PollAdded {
                contest: self,
                added_poll: poll,
            })
        } else {
            Err(anyhow::anyhow!("You can't add a poll to non-open contest"))
        }
    }
}

pub trait WithAttrs: Contest {
    fn _status(&self) -> ContestStatus;

    fn _title(&self) -> &str;

    fn _category(&self) -> &str;

    fn _event_start_at(&self) -> Option<&DateTime<Utc>>;
}

pub trait WithPoll: Contest {
    fn _current_poll(&self) -> Option<&Poll>;
}

#[derive(Debug, Clone, Copy, Serialize, JsonSchema)]
pub struct ContestId(pub Uuid);

impl ContestId {
    pub fn new() -> ContestId {
        ContestId(Uuid::new_v4())
    }
}

pub type ContestStatus = crop_infra::pg::types::ContestStatus;

impl<'a, C> Contest for &'a C
where
    C: Contest,
{
    fn id(&self) -> ContestId {
        C::id(self)
    }
}

impl<'a, C> WithAttrs for &'a C
where
    C: WithAttrs,
{
    fn _status(&self) -> ContestStatus {
        C::_status(self)
    }

    fn _title(&self) -> &str {
        C::_title(self)
    }

    fn _category(&self) -> &str {
        C::_category(self)
    }

    fn _event_start_at(&self) -> Option<&DateTime<Utc>> {
        C::_event_start_at(self)
    }
}

impl<'a, C> WithPoll for &'a C
where
    C: WithPoll,
{
    fn _current_poll(&self) -> Option<&Poll> {
        C::_current_poll(self)
    }
}

/*
 * ============
 * PollAdded
 * ============
 */
pub struct PollAdded<C> {
    pub contest: C,
    pub added_poll: Poll,
}
