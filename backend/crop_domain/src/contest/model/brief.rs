use crate::contest::{ListQueryable, Queryable};
use chrono::{DateTime, Utc};
use crop_infra::pg::{contest::ContestTable, Connection};
use schemars::JsonSchema;
use serde::Serialize;

use super::{Contest, ContestId, ContestStatus, WithAttrs};

/*
 * ============
 * BriefContest
 * ============
 */
// QueriedContestに直接Contestを実装することも可能だが、
// その場合infra層の実装をapplication層まで伝搬することになりちょっとよくない。
// 特に、SerializeやJsonSchemaの実装をinfra層のモデルに対して行うことになってしまう。
// また、このモデルが表現したい本質的内容は、「DBからQueryしたContest」ではなく
// 「簡潔なContest」である。
// そのため、ここでQueriedContestのラッパーとしてBriefContestを実装する。
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct BriefContest {
    id: ContestId,
    status: ContestStatus,
    title: String,
    category: String,
    event_start_at: Option<DateTime<Utc>>,
}

impl Contest for BriefContest {
    fn id(&self) -> ContestId {
        self.id
    }
}

impl WithAttrs for BriefContest {
    fn _status(&self) -> ContestStatus {
        self.status
    }

    fn _title(&self) -> &str {
        self.title.as_str()
    }

    fn _category(&self) -> &str {
        self.category.as_str()
    }

    fn _event_start_at(&self) -> Option<&DateTime<Utc>> {
        self.event_start_at.as_ref()
    }
}

impl Queryable for BriefContest {
    fn query_by_id(conn: &Connection, id: &ContestId) -> anyhow::Result<Option<Self>> {
        if let Some(queried) = ContestTable::query_by_id(conn, &id.0)? {
            Ok(Some(BriefContest {
                id: ContestId(queried.id),
                status: queried.status,
                title: queried.title,
                category: queried.category,
                event_start_at: queried.event_start_at,
            }))
        } else {
            Ok(None)
        }
    }
}

impl ListQueryable for BriefContest {
    fn query_not_archived(conn: &Connection) -> anyhow::Result<Vec<Self>> {
        Ok(ContestTable::query_not_archived(conn)?
            .into_iter()
            .map(|queried| BriefContest {
                id: ContestId(queried.id),
                status: queried.status,
                title: queried.title,
                category: queried.category,
                event_start_at: queried.event_start_at,
            })
            .collect())
    }
}
