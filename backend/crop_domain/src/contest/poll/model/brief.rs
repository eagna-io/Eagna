use super::{ChoiceColor, ChoiceName, Poll, PollId, WithAttrs};
use chrono::{DateTime, Duration, Utc};
use crop_infra::pg::{choice::QueriedChoice, poll::QueriedPoll};
use schemars::JsonSchema;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, JsonSchema)]
pub struct BriefPoll {
    pub(super) id: PollId,
    pub(super) title: String,
    pub(super) created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_duration")]
    // https://github.com/GREsau/schemars/issues/15#issuecomment-593006526
    #[schemars(with = "Option<i64>")]
    pub(super) duration: Option<Duration>,
    pub(super) choices: HashMap<ChoiceName, ChoiceColor>,
    pub(super) resolved_choice: Option<ChoiceName>,
}

fn serialize_duration<S>(value: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    value
        .as_ref()
        .map(|d| d.num_seconds())
        .serialize(serializer)
}

impl Poll for BriefPoll {
    fn id(&self) -> PollId {
        self.id
    }
}

impl WithAttrs for BriefPoll {
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
        self.resolved_choice.as_ref()
    }
}

impl From<(QueriedPoll, Vec<QueriedChoice>)> for BriefPoll {
    fn from(queried: (QueriedPoll, Vec<QueriedChoice>)) -> Self {
        let (poll, choices) = queried;
        BriefPoll {
            id: PollId(poll.id),
            title: poll.title,
            created_at: poll.created_at,
            duration: poll.duration_sec.map(|s| Duration::seconds(s as i64)),
            resolved_choice: poll.resolved_choice_name.map(|s| ChoiceName(s)),
            choices: choices
                .into_iter()
                .map(|choice| (ChoiceName(choice.name), ChoiceColor(choice.color)))
                .collect(),
        }
    }
}