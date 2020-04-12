use super::model::{BriefContest, Contest, WithAttrs, WithPoll};
use crop_infra::pg::Connection;

pub trait ContestRepository {
    fn conn(&self) -> &Connection;

    // 新規作成されたContestを保存する
    // 新規作成されたContestにはPollはまだ設定されて
    // いないはずなので、Pollの保存は試みない
    fn save_new<C>(&self, contest: &C) -> anyhow::Result<()>
    where
        C: Contest + WithAttrs + WithPoll,
    {
        use crop_infra::pg::contest::{ContestTable as _, NewContest};

        assert!(contest.current_poll().is_none());

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

    /*
    fn query_by_id(&self, id: &ContestId) -> anyhow::Result<Option<DetailedContest>> {
        use crop_infra::pg::{contest::ContestTable, poll::PollTable};

        if let Some(contest) = ContestTable::query_by_id(self.conn(), &id.0)? {
            let polls = PollTable::query_by_contest_id(self.conn(), &id.0)?;
        }
    }
    */
}

impl ContestRepository for Connection {
    fn conn(&self) -> &Connection {
        self
    }
}
