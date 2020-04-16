use crate::account::{Account, AccountId};
use crate::contest::comment::{BriefComment, Comment, CommentId};
use chrono::{DateTime, Duration, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

mod brief;
mod choice_updated;
mod comment_added;
mod detailed;
mod new;
mod resolved;

pub use brief::BriefPoll;
pub use choice_updated::ChoiceUpdated;
pub use comment_added::CommentAdded;
pub use detailed::DetailedPoll;
pub use new::New;
pub use resolved::Resolved;

pub(crate) fn new(
    title: String,
    duration: Option<Duration>,
    choices: HashMap<ChoiceName, ChoiceColor>,
) -> New {
    New {
        id: PollId::new(),
        title,
        created_at: Utc::now(),
        duration,
        choices,
    }
}

pub trait Poll {
    fn id(&self) -> PollId;

    fn title(&self) -> &str
    where
        Self: WithAttrs,
    {
        self._title()
    }

    /// Pollが作成された日時
    fn created_at(&self) -> &DateTime<Utc>
    where
        Self: WithAttrs,
    {
        self._created_at()
    }

    /// Pollが開いている長さ
    fn duration(&self) -> Option<&Duration>
    where
        Self: WithAttrs,
    {
        self._duration()
    }

    /// PollがClosed（回答不可）になる日時
    fn closed_at(&self) -> Option<DateTime<Utc>>
    where
        Self: WithAttrs,
    {
        if let Some(dur) = self.duration() {
            Some(*self.created_at() + *dur)
        } else {
            None
        }
    }

    /// PollがOpen（回答可能）かどうか
    fn is_open(&self) -> bool
    where
        Self: WithAttrs,
    {
        if let Some(closed_at) = self.closed_at() {
            Utc::now() < closed_at
        } else {
            self.resolved_choice().is_none()
        }
    }

    /// PollがClosedかどうか
    fn is_closed(&self) -> bool
    where
        Self: WithAttrs,
    {
        !self.is_open()
    }

    fn choices(&self) -> &HashMap<ChoiceName, ChoiceColor>
    where
        Self: WithAttrs,
    {
        self._choices()
    }

    fn resolved_choice(&self) -> Option<&ChoiceName>
    where
        Self: WithAttrs,
    {
        self._resolved_choice()
    }

    #[must_use]
    fn resolve(&self, choice: ChoiceName) -> anyhow::Result<Resolved<&Self>>
    where
        Self: WithAttrs,
    {
        if self.is_open() {
            Err(anyhow::anyhow!("Poll is still open"))
        } else if self.resolved_choice().is_some() {
            Err(anyhow::anyhow!("Poll is already resolved"))
        } else if !self.choices().contains_key(&choice) {
            Err(anyhow::anyhow!("Given choice is not a part of this poll"))
        } else {
            Ok(Resolved {
                poll: self,
                resolved: choice,
            })
        }
    }

    fn user_choices(&self) -> &HashMap<AccountId, ChoiceName>
    where
        Self: WithUserChoices,
    {
        self._user_choices()
    }

    #[must_use]
    fn update_account_choice<A>(
        &self,
        account: &A,
        choice: ChoiceName,
    ) -> anyhow::Result<ChoiceUpdated<&Self>>
    where
        Self: WithAttrs,
        A: Account,
    {
        if !self.is_open() {
            Err(anyhow::anyhow!("You can't update choice of closed poll"))
        } else if !self.choices().contains_key(&choice) {
            Err(anyhow::anyhow!("Given choice is not a part of this poll"))
        } else {
            Ok(ChoiceUpdated {
                poll: self,
                account_id: *account.id(),
                choice,
            })
        }
    }

    #[must_use]
    fn compute_stats(&self) -> Stats
    where
        Self: WithAttrs + WithUserChoices,
    {
        let mut vote_per_choice = self
            .choices()
            .keys()
            .map(|c| (c.clone(), 0))
            .collect::<HashMap<ChoiceName, usize>>();

        // 各Choiceの総得票数を計算
        self.user_choices()
            .values()
            .for_each(|choice| *vote_per_choice.get_mut(choice).unwrap() += 1);

        Stats {
            total_votes: self.user_choices().len(),
            vote_per_choice,
        }
    }

    fn comments(&self) -> &[Self::Comment]
    where
        Self: WithComments,
    {
        self._comments()
    }

    #[must_use]
    fn add_comment<A>(&self, account: A, comment_str: String) -> CommentAdded<&Self>
    where
        Self: WithAttrs + WithUserChoices,
        A: Account,
    {
        let choice = self.user_choices().get(account.id()).cloned();

        let comment = BriefComment {
            id: CommentId::new(),
            account_id: *account.id(),
            choice_name: choice,
            created_at: Utc::now(),
            comment: comment_str,
        };

        CommentAdded {
            poll: self,
            comment,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct PollId(pub Uuid);

impl PollId {
    pub fn new() -> PollId {
        PollId(Uuid::new_v4())
    }
}

impl std::str::FromStr for PollId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PollId(Uuid::parse_str(s)?))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct ChoiceName(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct ChoiceColor(pub String);

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub total_votes: usize,
    pub vote_per_choice: HashMap<ChoiceName, usize>,
}

pub trait WithAttrs: Poll {
    fn _title(&self) -> &str;

    fn _created_at(&self) -> &DateTime<Utc>;

    fn _duration(&self) -> Option<&Duration>;

    fn _choices(&self) -> &HashMap<ChoiceName, ChoiceColor>;

    fn _resolved_choice(&self) -> Option<&ChoiceName>;
}

pub trait WithUserChoices: Poll {
    fn _user_choices(&self) -> &HashMap<AccountId, ChoiceName>;
}

pub trait WithComments: Poll {
    type Comment: Comment;

    fn _comments(&self) -> &[Self::Comment];
}

impl<'a, P> Poll for &'a P
where
    P: Poll,
{
    fn id(&self) -> PollId {
        P::id(self)
    }
}

impl<'a, P> WithAttrs for &'a P
where
    P: WithAttrs,
{
    fn _title(&self) -> &str {
        P::_title(self)
    }

    fn _created_at(&self) -> &DateTime<Utc> {
        P::_created_at(self)
    }

    fn _duration(&self) -> Option<&Duration> {
        P::_duration(self)
    }

    fn _choices(&self) -> &HashMap<ChoiceName, ChoiceColor> {
        P::_choices(self)
    }

    fn _resolved_choice(&self) -> Option<&ChoiceName> {
        P::_resolved_choice(self)
    }
}

impl<'a, P> WithUserChoices for &'a P
where
    P: WithUserChoices,
{
    fn _user_choices(&self) -> &HashMap<AccountId, ChoiceName> {
        P::_user_choices(self)
    }
}

impl<'a, P> WithComments for &'a P
where
    P: WithComments,
{
    type Comment = P::Comment;

    fn _comments(&self) -> &[Self::Comment] {
        P::_comments(self)
    }
}
