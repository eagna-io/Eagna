use super::model::{BriefContest, ContestId};
use super::poll::Poll;
use crop_infra::pg::Connection;

pub trait ContestRepository {
    fn conn(&self) -> &Connection;

    // 新規作成されたContestを保存する
    // 新規作成されたContestにはPollはまだ設定されて
    // いないはずなので、Pollの保存は試みない
    fn save<C>(&self, contest: &C) -> anyhow::Result<()>
    where
        C: Updatable,
    {
        contest.save(self.conn())
    }

    /// ArchivedでないContestを全て取得する
    fn query_not_archived(&self) -> anyhow::Result<Vec<BriefContest>> {
        use crop_infra::pg::contest::ContestTable;

        Ok(ContestTable::query_not_archived(self.conn())?
            .into_iter()
            .map(BriefContest::from)
            .collect())
    }

    fn query_brief_by_id(&self, id: &ContestId) -> anyhow::Result<Option<BriefContest>> {
        use crop_infra::pg::contest::ContestTable;

        if let Some(queried) = ContestTable::query_by_id(self.conn(), &id.0)? {
            Ok(Some(BriefContest::from(queried)))
        } else {
            Ok(None)
        }
    }
}

impl ContestRepository for Connection {
    fn conn(&self) -> &Connection {
        self
    }
}

/*
 * ==========
 * Updatable
 * ==========
 */
pub trait Updatable {
    fn save(&self, conn: &Connection) -> anyhow::Result<()>;
}
