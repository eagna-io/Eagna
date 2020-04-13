use super::poll::{self, ChoiceColor, ChoiceName};
use crate::contest::poll::model::Poll;
use chrono::{DateTime, Duration, Utc};
use schemars::JsonSchema;
use serde::Serialize;
use std::{collections::HashMap, str::FromStr};
use uuid::Uuid;

mod brief;
mod detailed;
mod new;
mod poll_added;

pub use brief::BriefContest;
pub use detailed::DetailedContest;
pub use new::New;
pub use poll_added::PollAdded;

pub fn new(title: String, category: String, event_start_at: Option<DateTime<Utc>>) -> New {
    New {
        id: ContestId::new(),
        title,
        category,
        event_start_at,
    }
}

pub trait Contest {
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

    fn current_poll(&self) -> Option<&Self::Poll>
    where
        Self: WithPoll,
    {
        self._current_poll()
    }

    /// Contestに新しいPollを追加する。
    /// ContestがOpenのときのみ追加できる。
    ///
    /// ## TODO
    /// Contestで現在un-resolvedなPollが存在するときには追加できないようにする
    #[must_use]
    fn add_poll(
        &self,
        title: String,
        duration: Option<Duration>,
        choices: HashMap<ChoiceName, ChoiceColor>,
    ) -> anyhow::Result<PollAdded<&Self>>
    where
        Self: WithAttrs,
    {
        if self.status() == ContestStatus::Open {
            let new_poll = poll::new(title, duration, choices);
            Ok(PollAdded {
                contest: self,
                poll: new_poll,
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
    type Poll: Poll;

    fn _current_poll(&self) -> Option<&Self::Poll>;
}

#[derive(Debug, Clone, Copy, Serialize, JsonSchema)]
pub struct ContestId(pub Uuid);

impl ContestId {
    pub fn new() -> ContestId {
        ContestId(Uuid::new_v4())
    }
}

impl FromStr for ContestId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ContestId(Uuid::parse_str(s)?))
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
    type Poll = C::Poll;

    fn _current_poll(&self) -> Option<&Self::Poll> {
        C::_current_poll(self)
    }
}
