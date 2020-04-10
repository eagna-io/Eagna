use super::model::Contest;
use crop_infra::pg::Connection;

pub trait ContestRepository {
    fn conn(&self) -> &Connection;

    // 新規作成されたContestを保存する
    // 新規作成されたContestにはPollはまだ設定されて
    // いないはずなので、Pollの保存は試みない
    fn save_new(&self, contest: &Contest) -> anyhow::Result<()> {
        use crop_infra::pg::contest::{ContestTable as _, NewContest};

        assert!(contest.polls.is_empty());

        let new_contest = NewContest {
            id: &contest.id.0,
            title: contest.title.as_str(),
            category: contest.category.as_str(),
            event_start_at: contest.event_start_at.as_ref(),
        };
        self.conn().save(new_contest)
    }

    // Contestに新しく追加したPollを保存する
    fn save_new_poll(&self, contest: &Contest) -> anyhow::Result<()> {
        use crop_infra::pg::poll::{NewPoll, PollTable as _};

        assert!(!contest.polls.is_empty());

        let poll = contest.polls.last().unwrap();
        let new_poll = NewPoll {
            id: &poll.id.0,
            contest_id: &contest.id.0,
            title: poll.title.as_str(),
            created_at: &poll.created_at,
            duration_sec: poll.duration_sec,
        };
        self.conn().save(new_poll)?;

        todo!("Chiceの保存")
    }
}
