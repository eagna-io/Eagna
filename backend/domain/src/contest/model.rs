pub struct Contest {
    id: ContestId,
    status: ContestStatus,
    title: String,
    category: String,
    event_start_at: Option<DateTime<Utc>>,
    current_poll: Option<Poll>,
}

impl Contest {
    pub fn new(
        title: String,
        category: String,
        event_start_at: DateTime<Utc>,
    ) -> anyhow::Result<Contest> {
        if title.is_empty() {
            return Err(anyhow::anyhow!("title is empty"));
        }

        if category.is_empty() {
            return Err(anyhow::anyhow!("category is empty"));
        }

        Contest {
            id: ContestId::new(),
            status: ContestStatus::Upcoming,
            title,
            category,
            event_start_at,
            current_poll: None,
        }
    }

    pub fn add_comment(&self, account_id: AccountId, comment: String) -> anyhow::Result<Comment> {}

    /// イベントの開始時間は気にしない
    pub fn open(&mut self) -> anyhow::Result<()> {
        if self.status != ContestStatus::Upcoming {
            return Err(anyhow::anyhow!("Status is not upcoming"));
        }

        self.status = ContestStatus::Open;
    }

    pub fn add_poll(
        &mut self,
        title: String,
        duration: Duration,
        choices: Vec<Choice>,
    ) -> anyhow::Result<()> {
        if self.status != ContestStatus::Open {
            return Err(anyhow::anyhow!("Status is not open"));
        }

        if let Some(current_poll) = self.current_poll.as_ref() {
            if current_poll.status.resolved_choice_name.is_none() {
                return Err(anyhow::anyhow!("Current poll is not resolved"));
            }
        }

        let idx = self.current_poll.map(|poll| poll.idx + 1).unwrap_or(0);

        let poll = Poll::new(idx, title, duration, choices)?;

        self.current_poll = Some(poll);
    }
}
