use crate::contest::poll::model::Poll;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Contest {
    pub id: ContestId,
    pub title: String,
    pub category: String,
    pub event_start_at: Option<DateTime<Utc>>,
    pub polls: Vec<Poll>,
}

pub struct ContestId(pub Uuid);

impl Contest {
    pub fn new(
        title: impl Into<String>,
        category: impl Into<String>,
        event_start_at: Option<DateTime<Utc>>,
    ) -> Contest {
        Contest {
            id: ContestId::new(),
            title: title.into(),
            category: category.into(),
            event_start_at,
            polls: Vec::new(),
        }
    }

    pub fn current_poll(&self) -> Option<&Poll> {
        self.polls.last()
    }

    pub fn current_poll_mut(&mut self) -> Option<&mut Poll> {
        self.polls.last_mut()
    }

    pub fn add_poll(&mut self, poll: Poll) -> &Poll {
        self.polls.push(poll);
        self.current_poll().unwrap()
    }
}

impl ContestId {
    pub fn new() -> ContestId {
        ContestId(Uuid::new_v4())
    }
}
