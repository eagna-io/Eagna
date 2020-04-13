use crate::contest::poll::{Poll, PollId};
use crate::contest::Updatable;
use chrono::{DateTime, Utc};
use crop_infra::pg::Connection;

use super::{Contest, ContestId, ContestStatus, WithAttrs, WithPoll};

pub struct New {
    pub(super) id: ContestId,
    pub(super) title: String,
    pub(super) category: String,
    pub(super) event_start_at: Option<DateTime<Utc>>,
}

impl Contest for New {
    fn id(&self) -> ContestId {
        self.id
    }
}

impl WithAttrs for New {
    fn _status(&self) -> ContestStatus {
        ContestStatus::Upcoming
    }

    fn _title(&self) -> &str {
        self.title.as_str()
    }

    fn _category(&self) -> &str {
        self.category.as_str()
    }

    fn _event_start_at(&self) -> Option<&DateTime<Utc>> {
        self.event_start_at.as_ref()
    }
}

impl WithPoll for New {
    type Poll = NeverPoll;

    fn _current_poll(&self) -> Option<&Self::Poll> {
        None
    }
}

pub enum NeverPoll {}

impl Poll for NeverPoll {
    fn id(&self) -> PollId {
        panic!("never call");
    }
}

impl Updatable for New {
    fn save(&self, conn: &Connection) -> anyhow::Result<()> {
        use crop_infra::pg::contest::{ContestTable, NewContest};

        let new_contest = NewContest {
            id: &self.id().0,
            title: self.title(),
            category: self.category(),
            event_start_at: self.event_start_at(),
        };
        ContestTable::save(conn, new_contest)
    }
}
