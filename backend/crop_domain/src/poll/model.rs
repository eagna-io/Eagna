use chrono::{DateTime, Duration, Utc};
use crop_primitive::string::String;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use crate::account::model::AccountName;

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Poll {
    // Immutable
    pub id: Id,
    pub title: String,
    pub end_at: DateTime<Utc>,
    pub choices: HashMap<ChoiceName, ChoiceColor>,

    // Mutable
    pub status: Status,
    #[serde(skip)]
    pub user_choice: HashMap<AccountName, ChoiceName>,
    #[serde(skip)]
    pub comments: VecDeque<Comment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<Stats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ChoiceName>,
}

#[derive(Debug, Clone, Copy, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct Id(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Open,
    Closed,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct ChoiceName(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct ChoiceColor(pub String);

impl Poll {
    pub fn new(title: String, choices: HashMap<ChoiceName, ChoiceColor>) -> Poll {
        Poll {
            id: Id::new(),
            title,
            end_at: Utc::now() + Duration::seconds(30),
            choices,
            status: Status::Open,
            user_choice: HashMap::new(),
            comments: VecDeque::with_capacity(20),
            stats: None,
            resolved: None,
        }
    }

    pub fn is_open(&self) -> bool {
        self.status == Status::Open
    }

    pub fn is_closed(&self) -> bool {
        self.status == Status::Closed
    }

    pub fn update_choice(&mut self, account: AccountName, choice: ChoiceName) {
        if self.is_open() {
            self.user_choice.insert(account, choice);
        }
    }

    pub fn add_comment(&mut self, account: AccountName, comment_str: String) -> &Comment {
        let color = self
            .user_choice
            .get(&account)
            .and_then(|choice| self.choices.get(choice).cloned())
            .unwrap_or_else(|| ChoiceColor("#888888".into()));
        let comment = Comment {
            account: account,
            comment: comment_str,
            color,
        };

        self.comments.push_back(comment);
        if self.comments.len() > 20 {
            self.comments.pop_front();
        }

        self.comments.back().unwrap()
    }

    pub fn close_or_ignore(&mut self) -> bool {
        if self.is_open() && self.is_closable() {
            self.status = Status::Closed;
            self.stats = Some(self.compute_stats());
            true
        } else {
            false
        }
    }

    fn is_closable(&self) -> bool {
        self.end_at < Utc::now()
    }

    fn compute_stats(&self) -> Stats {
        let mut vote_per_choice = self
            .choices
            .keys()
            .map(|c| (c.clone(), 0))
            .collect::<HashMap<ChoiceName, usize>>();

        // 各Choiceの総得票数を計算
        self.user_choice
            .values()
            .for_each(|choice| *vote_per_choice.get_mut(choice).unwrap() += 1);

        Stats {
            total_votes: self.user_choice.len(),
            vote_per_choice,
        }
    }

    pub fn resolve(&mut self, choice: ChoiceName) -> anyhow::Result<()> {
        if !self.is_closed() {
            Err(anyhow::anyhow!("Poll is not closed"))
        } else if self.resolved.is_some() {
            Err(anyhow::anyhow!("Poll is already resolved"))
        } else if !self.choices.contains_key(&choice) {
            Err(anyhow::anyhow!(
                "Given choice is not a part of current poll"
            ))
        } else {
            self.resolved = Some(choice);
            Ok(())
        }
    }
}

impl Id {
    pub fn new() -> Id {
        Id(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Comment {
    pub account: AccountName,
    pub comment: String,
    pub color: ChoiceColor,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub total_votes: usize,
    pub vote_per_choice: HashMap<ChoiceName, usize>,
}
