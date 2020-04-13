use crate::contest::poll::{BriefPoll, Poll};
use crate::contest::Queryable;
use chrono::{DateTime, Utc};
use crop_infra::pg::{choice::ChoiceTable, contest::ContestTable, poll::PollTable, Connection};

use super::{Contest, ContestId, ContestStatus, WithAttrs, WithPoll};

/*
 * ===============
 * DetailedContest
 * ===============
 */
pub struct DetailedContest<P> {
    pub(super) id: ContestId,
    pub(super) status: ContestStatus,
    pub(super) title: String,
    pub(super) category: String,
    pub(super) event_start_at: Option<DateTime<Utc>>,
    pub(super) poll: Option<P>,
}

impl<P> Contest for DetailedContest<P> {
    fn id(&self) -> ContestId {
        self.id
    }
}

impl<P> WithAttrs for DetailedContest<P> {
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

impl<P> WithPoll for DetailedContest<P>
where
    P: Poll,
{
    type Poll = P;

    fn _current_poll(&self) -> Option<&P> {
        self.poll.as_ref()
    }
}

impl Queryable for DetailedContest<BriefPoll> {
    fn query_by_id(conn: &Connection, id: &ContestId) -> anyhow::Result<Option<Self>> {
        let contest = match ContestTable::query_by_id(conn, &id.0)? {
            Some(contest) => contest,
            None => return Ok(None),
        };

        if let Some(poll) = PollTable::query_not_resolved_by_contest_id(conn, &id.0)? {
            let choices = ChoiceTable::query_by_poll_id(conn, &poll.id)?;
            Ok(Some(DetailedContest {
                id: *id,
                status: contest.status,
                title: contest.title,
                category: contest.category,
                event_start_at: contest.event_start_at,
                poll: Some(BriefPoll::from((poll, choices))),
            }))
        } else {
            Ok(Some(DetailedContest {
                id: *id,
                status: contest.status,
                title: contest.title,
                category: contest.category,
                event_start_at: contest.event_start_at,
                poll: None,
            }))
        }
    }
}
