use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::Serialize;
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use crate::account::model::AccountName;

#[derive(Debug, Clone)]
pub struct Poll {
    pub id: Id,
    pub end_at: DateTime<Utc>,
    pub choices: HashMap<ChoiceName, ChoiceColor>,
    pub resolved: Option<ChoiceName>,

    pub user_choice: HashMap<AccountName, ChoiceName>,
    pub comments: VecDeque<Comment>,
}

#[derive(Debug, Clone)]
pub struct Id(Uuid);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ChoiceName(pub String);

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct ChoiceColor(pub String);

impl Poll {
    pub fn new(end_at: DateTime<Utc>, choices: HashMap<ChoiceName, ChoiceColor>) -> Poll {
        Poll {
            id: Id::new(),
            end_at,
            choices,
            resolved: None,
            user_choice: HashMap::new(),
            comments: VecDeque::with_capacity(20),
        }
    }

    pub fn update_choice(&mut self, account: AccountName, choice: ChoiceName) {
        self.user_choice.insert(account, choice);
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
