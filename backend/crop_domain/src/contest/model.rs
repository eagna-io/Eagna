use crate::contest::poll::model::Poll;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Contest<P> {
    pub id: ContestId,
    pub status: ContestStatus,
    pub title: String,
    pub category: String,
    pub event_start_at: Option<DateTime<Utc>>,
    pub polls: P,
}

pub struct ContestId(pub Uuid);

pub type ContestStatus = crop_infra::pg::types::ContestStatus;

pub struct Unknown;

pub struct Polls(Vec<Poll>);

impl Contest<Polls> {
    pub fn new(
        title: impl Into<String>,
        category: impl Into<String>,
        event_start_at: Option<DateTime<Utc>>,
    ) -> Contest<Polls> {
        Contest {
            id: ContestId::new(),
            status: ContestStatus::Upcoming,
            title: title.into(),
            category: category.into(),
            event_start_at,
            polls: Polls(Vec::new()),
        }
    }

    pub fn polls(&self) -> &Vec<Poll> {
        &self.polls.0
    }

    pub fn current_poll(&self) -> Option<&Poll> {
        self.polls.0.last()
    }

    pub fn current_poll_mut(&mut self) -> Option<&mut Poll> {
        self.polls.0.last_mut()
    }

    pub fn add_poll(&mut self, poll: Poll) -> &Poll {
        self.polls.0.push(poll);
        self.current_poll().unwrap()
    }
}

impl ContestId {
    pub fn new() -> ContestId {
        ContestId(Uuid::new_v4())
    }
}
