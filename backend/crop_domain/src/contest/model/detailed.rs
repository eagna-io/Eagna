use crate::contest::poll::model::Poll;
use chrono::{DateTime, Utc};

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
