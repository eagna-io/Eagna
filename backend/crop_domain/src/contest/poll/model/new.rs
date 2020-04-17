use super::{Choice, ChoiceName, Poll, PollId, PollStatus, WithAttrs, WithUserChoices};
use crate::account::AccountId;
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;

pub struct New {
    pub id: PollId,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub duration: Option<Duration>,
    pub idx: usize,
    pub choices: Vec<Choice>,
}

impl Poll for New {
    fn id(&self) -> &PollId {
        &self.id
    }
}

impl WithAttrs for New {
    fn _status(&self) -> PollStatus {
        PollStatus::Open
    }

    fn _title(&self) -> &str {
        self.title.as_str()
    }

    fn _created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    fn _duration(&self) -> Option<&Duration> {
        self.duration.as_ref()
    }

    fn _idx(&self) -> usize {
        self.idx
    }

    fn _choices(&self) -> &[Choice] {
        &self.choices.as_slice()
    }

    fn _resolved_choice(&self) -> Option<&ChoiceName> {
        None
    }
}

impl WithUserChoices for New {
    fn _user_choices(&self) -> &HashMap<AccountId, ChoiceName> {
        lazy_static::lazy_static! {
            static ref EMPTY: HashMap<AccountId, ChoiceName> = HashMap::new();
        }

        &EMPTY
    }
}
