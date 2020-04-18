use super::{Contest, ContestId, ContestStatus, WithAttrs, WithCurrentPoll};
use crate::contest::poll::{BriefPoll, DetailedPoll, Poll};
use crate::contest::Queryable;
use chrono::{DateTime, Utc};
use crop_infra::pg::{
    account_choice::AccountChoiceTable, choice::ChoiceTable, comment::CommentTable,
    contest::ContestTable, poll::PollTable, Connection,
};
use fallible_iterator::{convert, FallibleIterator as _};
use schemars::JsonSchema;
use serde::Serialize;

/*
 * ===============
 * DetailedContest
 * ===============
 */
#[derive(Debug, Serialize, JsonSchema)]
pub struct DetailedContest<P> {
    pub(super) id: ContestId,
    pub(super) status: ContestStatus,
    pub(super) title: String,
    pub(super) category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) event_start_at: Option<DateTime<Utc>>,
    pub(super) polls: Vec<P>,
}

impl<P> Contest for DetailedContest<P> {
    fn id(&self) -> &ContestId {
        &self.id
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

impl<P> WithCurrentPoll for DetailedContest<P>
where
    P: Poll,
{
    type Poll = P;

    fn _current_poll(&self) -> Option<&P> {
        self.polls.last()
    }
}

impl Queryable for DetailedContest<BriefPoll> {
    fn query_by_id(conn: &Connection, id: &ContestId) -> anyhow::Result<Option<Self>> {
        let contest = match ContestTable::query_by_id(conn, &id.0)? {
            Some(contest) => contest,
            None => return Ok(None),
        };

        let polls = convert(
            PollTable::query_by_contest_id(conn, &id.0)?
                .into_iter()
                .map::<anyhow::Result<_>, _>(|poll| {
                    let choices = ChoiceTable::query_by_poll_id(conn, &poll.id)?;
                    Ok(BriefPoll::from((poll, choices)))
                }),
        )
        .collect::<Vec<_>>()?;

        Ok(Some(DetailedContest {
            id: *id,
            status: contest.status,
            title: contest.title,
            category: contest.category,
            event_start_at: contest.event_start_at,
            polls,
        }))
    }
}

impl Queryable for DetailedContest<DetailedPoll> {
    fn query_by_id(conn: &Connection, id: &ContestId) -> anyhow::Result<Option<Self>> {
        let contest = match ContestTable::query_by_id(conn, &id.0)? {
            Some(contest) => contest,
            None => return Ok(None),
        };

        let polls = convert(
            PollTable::query_by_contest_id(conn, &id.0)?
                .into_iter()
                .map::<anyhow::Result<_>, _>(|poll| {
                    let choices = ChoiceTable::query_by_poll_id(conn, &poll.id)?;
                    let account_choices = AccountChoiceTable::query_by_poll_id(conn, &poll.id)?;
                    let comments = CommentTable::query_recent_by_poll_id(conn, &poll.id)?;
                    Ok(DetailedPoll::from((
                        poll,
                        choices,
                        account_choices,
                        comments,
                    )))
                }),
        )
        .collect::<Vec<_>>()?;

        Ok(Some(DetailedContest {
            id: *id,
            status: contest.status,
            title: contest.title,
            category: contest.category,
            event_start_at: contest.event_start_at,
            polls,
        }))
    }
}
