use super::{Answer, ContestId};
use crate::account::AccountId;
use chrono::Duration;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poll {
    pub id: PollId,
    pub contest_id: ContestId,
    pub status: PollStatus,
    pub idx: usize,
    pub title: String,
    pub duration: Duration,
    pub choices: Vec<Choice>,
    pub resolved_choice_name: Option<ChoiceName>,
    pub final_answers: HashMap<AccountId, Answer>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PollId(pub Uuid);

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
pub struct ChoiceName(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoiceColor(pub String);

impl Poll {
    pub(crate) fn new(
        contest_id: ContestId,
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
            contest_id,
            status: PollStatus::Open,
            idx,
            title,
            duration,
            choices,
            resolved_choice_name: None,
            final_answers: HashMap::new(),
        })
    }

    pub(crate) fn is_resolved(&self) -> bool {
        self.resolved_choice_name.is_some()
    }

    pub(crate) fn check_contains_choice(&self, target: &ChoiceName) -> anyhow::Result<()> {
        if self
            .choices
            .iter()
            .find(|choice| &choice.name == target)
            .is_some()
        {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid choice name"))
        }
    }

    pub(crate) fn update_final_answer(
        &mut self,
        account_id: AccountId,
        choice: ChoiceName,
    ) -> anyhow::Result<()> {
        if self.status != PollStatus::Open {
            return Err(anyhow::anyhow!("Poll is not open"));
        }

        self.check_contains_choice(&choice)?;

        let answer = Answer::new(&account_id, &self.id, choice);
        self.final_answers.insert(account_id, answer);

        Ok(())
    }

    pub(crate) fn resolve(&mut self, resolved_choice_name: ChoiceName) -> anyhow::Result<()> {
        if self.is_resolved() {
            return Err(anyhow::anyhow!("Poll is already resolved"));
        }

        self.check_contains_choice(&resolved_choice_name)?;

        self.resolved_choice_name = Some(resolved_choice_name);
        Ok(())
    }
}
