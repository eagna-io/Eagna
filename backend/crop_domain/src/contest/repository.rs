use super::model::Contest;
use crop_infra::pg::Connection;

// 新規作成されたContestを保存する
// 新規作成されたContestにはPollはまだ設定されて
// いないはずなので、Pollの保存は試みない
pub fn save_new(conn: &Connection, contest: &Contest) -> anyhow::Result<()> {
    use crop_infra::pg::contest::{ContestTable as _, NewContest};

    assert!(contest.polls.is_empty());

    let new_contest = NewContest { id: &contest.id.0 };
    conn.save(new_contest)
}

// Contestに新しく追加したPollを保存する
pub fn save_new_poll(conn: &Connection, contest: &Contest) -> anyhow::Result<()> {
    use crop_infra::pg::poll::{NewPoll, PollTable as _};

    assert!(!contest.polls.is_empty());

    let poll = contest.polls.last().unwrap();
    let new_poll = NewPoll {
        id: &poll.id.0,
        contest_id: &contest.id.0,
        title: poll.title.as_str(),
        end_at: &poll.end_at,
    };
    conn.save(new_poll)
}
