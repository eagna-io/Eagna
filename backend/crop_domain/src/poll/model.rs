use chrono::{DateTime, Utc};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use crate::account::model::{Account, AccountName};

#[derive(Debug, Clone)]
pub struct Poll {
    pub id: Id,
    pub end_at: DateTime<Utc>,
    pub choices: Vec<Choice>,
    pub resolved: Option<Choice>,

    pub user_choice: HashMap<AccountName, Choice>,
    pub comments: VecDeque<Comment>,
}

#[derive(Debug, Clone)]
pub struct Id(Uuid);

#[derive(Debug, Clone)]
pub struct Choice {
    pub name: String,
    pub color: String,
}

impl Poll {
    pub fn new(end_at: DateTime<Utc>, choices: Vec<Choice>) -> Poll {
        Poll {
            id: Id::new(),
            end_at,
            choices,
            resolved: None,
            user_choice: HashMap::new(),
            comments: VecDeque::with_capacity(20),
        }
    }

    pub fn update_choice(&mut self, account: &Account, choice: Choice) {
        self.user_choice.insert(account.name.clone(), choice);
    }

    pub fn add_comment(&mut self, account: &Account, comment_str: String) -> &Comment {
        let color = self
            .user_choice
            .get(&account.name)
            .map(|choice| choice.color.clone())
            .unwrap_or_else(|| "#888888".into());
        let comment = Comment {
            account: account.name.clone(),
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

impl Choice {
    pub fn new(name: impl Into<String>, color: impl Into<String>) -> Choice {
        Choice {
            name: name.into(),
            color: color.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub account: AccountName,
    pub comment: String,
    pub color: String,
}
