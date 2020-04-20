pub struct Poll {
    id: PollId,
    status: PollStatus,
    idx: usize,
    title: String,
    duration: Duration,
    choices: Vec<Choice>,
    resolved_choice_name: Option<ChoiceName>,
    account_answers: HashMap<AccountId, ChoiceName>,
}

impl Poll {
    pub(in crate::contest) fn new(
        title: String,
        duration: Duration,
        idx: usize,
        mut choices: Vec<Choice>,
    ) -> anyhow::Result<Poll> {
        if title.is_empty() {
            return Err(anyhow::anyhow!("title is empty"));
        }

        choices.dedup_by_key(|choice| choice.name);

        Poll {
            id: PollId::new(),
            status: PollStatus::Open,
            idx,
            title,
            duration,
            choices,
            resolved_choice_name: None,
        }
    }
}
