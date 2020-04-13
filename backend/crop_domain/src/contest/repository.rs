use crate::contest::ContestId;
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
    fn query_not_archived<C>(&self) -> anyhow::Result<Vec<C>>
    where
        C: ListQueryable,
    {
        C::query_not_archived(self.conn())
    }

    fn query_by_id<C>(&self, id: &ContestId) -> anyhow::Result<Option<C>>
    where
        C: Queryable,
    {
        C::query_by_id(self.conn(), id)
    }
}

impl ContestRepository for Connection {
    fn conn(&self) -> &Connection {
        self
    }
}

pub trait Updatable {
    fn save(&self, conn: &Connection) -> anyhow::Result<()>;
}

pub trait Queryable: Sized {
    fn query_by_id(conn: &Connection, id: &ContestId) -> anyhow::Result<Option<Self>>;
}

pub trait ListQueryable: Sized {
    fn query_not_archived(conn: &Connection) -> anyhow::Result<Vec<Self>>;
}
