use super::{ChoiceColor, ChoiceName, Poll, PollId, WithAttrs};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;

pub struct New {
    pub(crate) id: PollId,
    pub(crate) title: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) duration: Option<Duration>,
    pub(crate) choices: HashMap<ChoiceName, ChoiceColor>,
}

impl Poll for New {
    fn id(&self) -> PollId {
        self.id
    }
}

impl WithAttrs for New {
    fn _title(&self) -> &str {
        self.title.as_str()
    }

    fn _created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    fn _duration(&self) -> Option<&Duration> {
        self.duration.as_ref()
    }

    fn _choices(&self) -> &HashMap<ChoiceName, ChoiceColor> {
        &self.choices
    }

    fn _resolved_choice(&self) -> Option<&ChoiceName> {
        None
    }
}
