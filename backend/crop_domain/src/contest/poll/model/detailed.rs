use super::{
    BriefPoll, ChoiceColor, ChoiceName, Poll, PollId, WithAttrs, WithComments, WithUserChoices,
};
use crate::account::AccountId;
use crate::contest::comment::BriefComment;
use chrono::{DateTime, Duration, Utc};
use crop_infra::pg::{
    account_choice::QueriedAccountChoice, choice::QueriedChoice, comment::QueriedComment,
    poll::QueriedPoll,
};
use schemars::JsonSchema;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, JsonSchema)]
pub struct DetailedPoll {
    #[serde(flatten)]
    inner: BriefPoll,
    account_choices: HashMap<AccountId, ChoiceName>,
    comments: Vec<BriefComment>,
}

impl Poll for DetailedPoll {
    fn id(&self) -> PollId {
        self.inner.id()
    }
}

impl WithAttrs for DetailedPoll {
    fn _title(&self) -> &str {
        self.inner._title()
    }

    fn _created_at(&self) -> &DateTime<Utc> {
        self.inner._created_at()
    }

    fn _duration(&self) -> Option<&Duration> {
        self.inner._duration()
    }

    fn _choices(&self) -> &HashMap<ChoiceName, ChoiceColor> {
        &self.inner._choices()
    }

    fn _resolved_choice(&self) -> Option<&ChoiceName> {
        self.inner._resolved_choice()
    }
}

impl WithUserChoices for DetailedPoll {
    fn _user_choices(&self) -> &HashMap<AccountId, ChoiceName> {
        &self.account_choices
    }
}

impl WithComments for DetailedPoll {
    type Comment = BriefComment;

    fn _comments(&self) -> &[BriefComment] {
        self.comments.as_slice()
    }
}

impl
    From<(
        QueriedPoll,
        Vec<QueriedChoice>,
        Vec<QueriedAccountChoice>,
        Vec<QueriedComment>,
    )> for DetailedPoll
{
    fn from(
        queried: (
            QueriedPoll,
            Vec<QueriedChoice>,
            Vec<QueriedAccountChoice>,
            Vec<QueriedComment>,
        ),
    ) -> Self {
        let (poll, choices, account_choices, comments) = queried;
        let brief_poll = BriefPoll::from((poll, choices));
        DetailedPoll {
            inner: brief_poll,
            account_choices: account_choices
                .into_iter()
                .map(|record| (AccountId(record.account_id), ChoiceName(record.choice_name)))
                .collect(),
            comments: comments.into_iter().map(BriefComment::from).collect(),
        }
    }
}
