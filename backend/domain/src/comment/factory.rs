pub fn create(
    contest: &Contest,
    account_id: AccoundId,
    comment: String,
) -> anyhow::Result<Comment> {
    if contest.status == ContestStatus::Archived {
        return Err(anyhow::anyhow!("Contest status is archived"));
    }

    let choice_name = contest
        .current_poll
        .as_ref()
        .and_then(|poll| poll.answers.get(&account_id))
        .cloned();

    Ok(Comment {
        contest_id: contest.id,
        account_id,
        comment,
        choice_name,
    })
}
