use super::{ChoiceName, Poll, PollId};

pub struct Resolved<P> {
    pub(super) poll: P,
    pub(super) resolved: ChoiceName,
}
