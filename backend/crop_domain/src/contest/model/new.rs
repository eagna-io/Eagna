use super::{Contest, ContestId, ContestStatus, WithAttrs, WithPoll};
use crate::contest::poll::model::{Poll, PollId};
use chrono::{DateTime, Utc};

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
