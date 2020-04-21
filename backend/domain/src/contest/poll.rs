use crate::account::AccountId;
use chrono::Duration;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poll {
    pub id: PollId,
    pub status: PollStatus,
    pub idx: usize,
    pub title: String,
    pub duration: Duration,
    pub choices: Vec<Choice>,
    pub resolved_choice_name: Option<ChoiceName>,
    pub account_choices: HashMap<AccountId, ChoiceName>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PollId(Uuid);

impl PollId {
    fn new() -> Self {
        PollId(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PollStatus {
    Open,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    pub name: ChoiceName,
    pub color: ChoiceColor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoiceName(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoiceColor(String);

impl Poll {
    pub(crate) fn new(
        idx: usize,
        title: String,
        duration: Duration,
        mut choices: Vec<Choice>,
    ) -> anyhow::Result<Poll> {
        if title.is_empty() {
            return Err(anyhow::anyhow!("title is empty"));
        }

        choices.dedup_by(|a, b| a.name == b.name);

        Ok(Poll {
            id: PollId::new(),
            status: PollStatus::Open,
            idx,
            title,
            duration,
            choices,
            resolved_choice_name: None,
            account_choices: HashMap::new(),
        })
    }

    pub(crate) fn is_resolved(&self) -> bool {
        self.resolved_choice_name.is_some()
    }

    pub(crate) fn contains_choice(&self, target: &ChoiceName) -> bool {
        self.choices
            .iter()
            .find(|choice| &choice.name == target)
            .is_some()
    }

    pub(crate) fn update_account_choice(
        &mut self,
        account_id: AccountId,
        choice: ChoiceName,
    ) -> anyhow::Result<()> {
        if !self.contains_choice(&choice) {
            return Err(anyhow::anyhow!("Invalid choice name"));
        }

        self.account_choices.insert(account_id, choice);
        Ok(())
    }

    pub(crate) fn resolve(&mut self, resolved_choice_name: ChoiceName) -> anyhow::Result<()> {
        if self.is_resolved() {
            return Err(anyhow::anyhow!("Poll is already resolved"));
        }

        if !self.contains_choice(&resolved_choice_name) {
            return Err(anyhow::anyhow!("Invalid resolved_choice_name"));
        }

        self.resolved_choice_name = Some(resolved_choice_name);
        Ok(())
    }
}
