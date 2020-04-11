use super::model::{BriefContest, Contest, WithPolls};
use crop_infra::pg::Connection;

pub trait ContestRepository {
    fn conn(&self) -> &Connection;

    // 新規作成されたContestを保存する
    // 新規作成されたContestにはPollはまだ設定されて
    // いないはずなので、Pollの保存は試みない
    fn save_new<C>(&self, contest: &C) -> anyhow::Result<()>
    where
        C: Contest + WithPolls,
    {
        use crop_infra::pg::contest::{ContestTable as _, NewContest};

        assert!(Contest::polls(contest).is_empty());

        let new_contest = NewContest {
            id: &contest.id().0,
            title: contest.title(),
            category: contest.category(),
            event_start_at: contest.event_start_at(),
        };
        self.conn().save(new_contest)
    }

    /// ArchivedでないContestを全て取得する
    fn query_not_archived(&self) -> anyhow::Result<Vec<BriefContest>> {
        use crop_infra::pg::contest::ContestTable;

        Ok(ContestTable::query_not_archived(self.conn())?
            .into_iter()
            .map(BriefContest::from)
            .collect())
    }
}

impl ContestRepository for Connection {
    fn conn(&self) -> &Connection {
        self
    }
}
