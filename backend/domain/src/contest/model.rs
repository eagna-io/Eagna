use super::poll::{Choice, ChoiceName, Poll};
use crate::account::AccountId;
use chrono::{DateTime, Duration, Utc};
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Contest {
    pub id: ContestId,
    pub status: ContestStatus,
    pub title: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_start_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_poll: Option<Poll>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct ContestId(pub Uuid);

impl ContestId {
    fn new() -> ContestId {
        ContestId(Uuid::new_v4())
    }

    pub fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ContestStatus {
    Upcoming,
    Open,
    Closed,
    Archived,
}

impl Contest {
    pub fn new(
        title: String,
        category: String,
        event_start_at: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Contest> {
        if title.is_empty() {
            return Err(anyhow::anyhow!("title is empty"));
        }

        if category.is_empty() {
            return Err(anyhow::anyhow!("category is empty"));
        }

        Ok(Contest {
            id: ContestId::new(),
            status: ContestStatus::Upcoming,
            title,
            category,
            event_start_at,
            current_poll: None,
        })
    }

    fn current_poll_mut_or_err(&mut self) -> anyhow::Result<&mut Poll> {
        self.current_poll
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Poll is not available"))
    }

    /// イベントの開始時間は気にしない
    pub fn open(&mut self) -> anyhow::Result<()> {
        if self.status != ContestStatus::Upcoming {
            return Err(anyhow::anyhow!("Status is not upcoming"));
        }

        self.status = ContestStatus::Open;
        Ok(())
    }

    pub fn switch_to_new_poll(
        &mut self,
        title: String,
        duration: Duration,
        choices: Vec<Choice>,
    ) -> anyhow::Result<()> {
        if self.status != ContestStatus::Open {
            return Err(anyhow::anyhow!("Status is not open"));
        }

        if let Some(current_poll) = self.current_poll.as_ref() {
            if !current_poll.is_resolved() {
                return Err(anyhow::anyhow!("Current poll is not resolved"));
            }
        }

        let idx = self
            .current_poll
            .as_ref()
            .map(|poll| poll.idx + 1)
            .unwrap_or(0);

        let poll = Poll::new(self.id, idx, title, duration, choices)?;

        self.current_poll = Some(poll);
        Ok(())
    }

    pub fn update_answer(
        &mut self,
        account_id: AccountId,
        choice: ChoiceName,
    ) -> anyhow::Result<()> {
        self.current_poll_mut_or_err()?
            .update_final_answer(account_id, choice)
    }

    pub fn close_poll(&mut self) -> anyhow::Result<()> {
        self.current_poll_mut_or_err()?.close()
    }

    pub fn resolve_poll(&mut self, resolved_choice_name: ChoiceName) -> anyhow::Result<()> {
        self.current_poll_mut_or_err()?
            .resolve(resolved_choice_name)
    }

    pub fn close(&mut self) -> anyhow::Result<()> {
        if self.status != ContestStatus::Open {
            return Err(anyhow::anyhow!("Status is not open"));
        }

        if let Some(current_poll) = self.current_poll.as_ref() {
            if !current_poll.is_resolved() {
                return Err(anyhow::anyhow!("Current poll is not resolved"));
            }
        }

        self.status = ContestStatus::Closed;
        Ok(())
    }

    pub fn archive(&mut self) -> anyhow::Result<()> {
        if self.status != ContestStatus::Closed {
            return Err(anyhow::anyhow!("Status is not closed"));
        }

        self.status = ContestStatus::Archived;
        Ok(())
    }
}
