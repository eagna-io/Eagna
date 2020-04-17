use crate::account::AccountId;
use crate::contest::poll::{self, Choice, New as NewPoll, Poll, PollId};
use chrono::{DateTime, Duration, Utc};
use schemars::JsonSchema;
use serde::Serialize;
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

mod archived;
mod brief;
mod closed;
mod detailed;
mod new;
mod poll_added;

pub use archived::Archived;
pub use brief::BriefContest;
pub use closed::Closed;
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
    fn id(&self) -> &ContestId;

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
        Self: WithCurrentPoll,
    {
        self._current_poll()
    }

    /// 何問のPollが出題されたか
    fn num_polls(&self) -> usize
    where
        Self: WithCurrentPoll,
        <Self as WithCurrentPoll>::Poll: poll::WithAttrs,
    {
        self.current_poll().map(|poll| poll.idx()).unwrap_or(0)
    }

    fn polls(&self) -> &[Self::Poll]
    where
        Self: WithPolls,
    {
        self._polls()
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
        choices: Vec<Choice>,
    ) -> anyhow::Result<PollAdded<&Self>>
    where
        Self: WithAttrs + WithCurrentPoll,
        <Self as WithCurrentPoll>::Poll: poll::WithAttrs,
    {
        if self.status() != ContestStatus::Open {
            return Err(anyhow::anyhow!("You can't add a poll to non-open contest"));
        }

        let idx = self.num_polls() + 1;

        let new_poll = NewPoll {
            id: PollId::new(),
            title,
            created_at: Utc::now(),
            duration,
            idx,
            choices,
        };
        Ok(PollAdded {
            contest: self,
            poll: new_poll,
        })
    }

    #[must_use]
    fn close(self) -> anyhow::Result<Closed<Self>>
    where
        Self: WithAttrs + WithCurrentPoll + Sized,
        <Self as WithCurrentPoll>::Poll: poll::WithAttrs,
    {
        if self.status() != ContestStatus::Open {
            return Err(anyhow::anyhow!("Contest status is not open"));
        }

        if let Some(poll) = self.current_poll() {
            if poll.resolved_choice().is_none() {
                return Err(anyhow::anyhow!("Contest has active poll"));
            }
        }

        Ok(Closed { contest: self })
    }

    #[must_use]
    fn archive(self) -> anyhow::Result<Archived<Self>>
    where
        Self: WithAttrs + Sized,
    {
        if self.status() != ContestStatus::Closed {
            return Err(anyhow::anyhow!("Contest status is not closed"));
        }

        Ok(Archived { contest: self })
    }

    /// 各アカウントの正解数を計算する
    fn compute_account_scores(&self) -> HashMap<AccountId, usize>
    where
        Self: WithPolls,
        <Self as WithPolls>::Poll: poll::WithAttrs + poll::WithUserChoices,
    {
        self.polls().iter().flat_map(Poll::correct_accounts).fold(
            HashMap::new(),
            |mut score_map, account| {
                *score_map.entry(account).or_insert(0) += 1;
                score_map
            },
        )
    }
}

pub trait WithAttrs: Contest {
    fn _status(&self) -> ContestStatus;

    fn _title(&self) -> &str;

    fn _category(&self) -> &str;

    fn _event_start_at(&self) -> Option<&DateTime<Utc>>;
}

pub trait WithCurrentPoll: Contest {
    type Poll: Poll;

    fn _current_poll(&self) -> Option<&Self::Poll>;
}

pub trait WithPolls: Contest {
    type Poll: Poll;

    fn _polls(&self) -> &[Self::Poll];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, JsonSchema)]
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
    fn id(&self) -> &ContestId {
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

impl<'a, C> WithCurrentPoll for &'a C
where
    C: WithCurrentPoll,
{
    type Poll = C::Poll;

    fn _current_poll(&self) -> Option<&Self::Poll> {
        C::_current_poll(self)
    }
}
