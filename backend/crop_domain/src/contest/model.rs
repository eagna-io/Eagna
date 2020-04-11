use crate::contest::poll::model::Poll;
use chrono::{DateTime, Utc};
use crop_infra::pg::contest::QueriedContest;
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

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
            polls: Vec::new(),
        }
    }

    fn id(&self) -> ContestId;

    fn status(&self) -> ContestStatus;

    fn title(&self) -> &str;

    fn category(&self) -> &str;

    fn event_start_at(&self) -> Option<&DateTime<Utc>>;

    fn polls(&self) -> &[Poll]
    where
        Self: WithPolls,
    {
        self._polls()
    }

    fn current_poll(&self) -> Option<&Poll>
    where
        Self: WithPolls,
    {
        self._polls().last()
    }

    fn current_poll_mut(&mut self) -> Option<&mut Poll>
    where
        Self: WithPolls,
    {
        self._polls_mut().last_mut()
    }

    fn add_poll(&mut self, poll: Poll) -> &Poll
    where
        Self: WithPolls,
    {
        self._polls_mut().push(poll);
        self.current_poll().unwrap()
    }
}

pub trait WithPolls: Contest {
    fn _polls(&self) -> &Vec<Poll>;

    fn _polls_mut(&mut self) -> &mut Vec<Poll>;
}

#[derive(Debug, Clone, Copy, Serialize, JsonSchema)]
pub struct ContestId(pub Uuid);

impl ContestId {
    pub fn new() -> ContestId {
        ContestId(Uuid::new_v4())
    }
}

pub type ContestStatus = crop_infra::pg::types::ContestStatus;

/*
 * ============
 * BriefContest
 * ============
 */
// QueriedContestに直接Contestを実装することも可能だが、
// その場合infra層の実装をapplication層まで伝搬することになりちょっとよくない。
// 特に、SerializeやJsonSchemaの実装をinfra層のモデルに対して行うことになってしまう。
// また、このモデルが表現したい本質的内容は、「DBからQueryしたContest」ではなく
// 「簡潔なContest」である。
// そのため、ここでQueriedContestのラッパーとしてBriefContestを実装する。
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct BriefContest {
    id: ContestId,
    status: ContestStatus,
    title: String,
    category: String,
    event_start_at: Option<DateTime<Utc>>,
}

impl Contest for BriefContest {
    fn id(&self) -> ContestId {
        self.id
    }

    fn status(&self) -> ContestStatus {
        self.status
    }

    fn title(&self) -> &str {
        self.title.as_str()
    }

    fn category(&self) -> &str {
        self.category.as_str()
    }

    fn event_start_at(&self) -> Option<&DateTime<Utc>> {
        self.event_start_at.as_ref()
    }
}

impl From<QueriedContest> for BriefContest {
    fn from(queried: QueriedContest) -> Self {
        BriefContest {
            id: ContestId(queried.id),
            status: queried.status,
            title: queried.title,
            category: queried.category,
            event_start_at: queried.event_start_at,
        }
    }
}

/*
 * ===============
 * DetailedContest
 * ===============
 */
pub struct DetailedContest {
    id: ContestId,
    status: ContestStatus,
    title: String,
    category: String,
    event_start_at: Option<DateTime<Utc>>,
    polls: Vec<Poll>,
}

impl Contest for DetailedContest {
    fn id(&self) -> ContestId {
        self.id
    }

    fn status(&self) -> ContestStatus {
        self.status
    }

    fn title(&self) -> &str {
        self.title.as_str()
    }

    fn category(&self) -> &str {
        self.category.as_str()
    }

    fn event_start_at(&self) -> Option<&DateTime<Utc>> {
        self.event_start_at.as_ref()
    }
}

impl WithPolls for DetailedContest {
    fn _polls(&self) -> &Vec<Poll> {
        &self.polls
    }

    fn _polls_mut(&mut self) -> &mut Vec<Poll> {
        &mut self.polls
    }
}
