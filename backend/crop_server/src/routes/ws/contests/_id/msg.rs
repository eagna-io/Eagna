use chrono::{DateTime, Utc};
use crop_domain::account::{Account as _, BriefAccount};
use crop_domain::contest::comment::{BriefComment, Comment as _};
use crop_domain::contest::poll::{self, Choice, ChoiceName, Poll, PollId, PollStatus, Stats};
use schemars::JsonSchema;
use serde::Serialize;
use warp::filters::ws::Message;

#[derive(Debug, Serialize, JsonSchema)]
pub enum OutgoingMsg<'a> {
    Poll(PollMsg<'a>),
    Comment(CommentMsg<'a>),
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct PollMsg<'a> {
    id: PollId,
    title: &'a str,
    created_at: &'a DateTime<Utc>,
    duration_sec: Option<i64>,
    idx: usize,
    #[schemars(with = "Vec<Choice>")]
    choices: &'a [Choice],
    resolved_choice: Option<&'a ChoiceName>,
    stats: Option<Stats>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct CommentMsg<'a> {
    account_name: &'a str,
    comment: &'a str,
    choice: Option<&'a ChoiceName>,
}

impl<'a> Into<Message> for OutgoingMsg<'a> {
    fn into(self) -> Message {
        Message::text(serde_json::to_string(&self).unwrap())
    }
}

/*
 * ==========
 * PollMsg
 * ==========
 */
impl<'a, P> From<&'a P> for OutgoingMsg<'a>
where
    P: Poll + poll::WithAttrs + poll::WithUserChoices,
{
    fn from(poll: &'a P) -> OutgoingMsg<'a> {
        let stats = match poll.status() {
            PollStatus::Open => None,
            PollStatus::Closed => Some(poll.compute_stats()),
        };
        OutgoingMsg::Poll(PollMsg {
            id: poll.id(),
            title: poll.title(),
            created_at: poll.created_at(),
            duration_sec: poll.duration().map(|d| d.num_seconds()),
            idx: poll.idx(),
            choices: poll.choices(),
            resolved_choice: poll.resolved_choice(),
            stats,
        })
    }
}

/*
 * ==========
 * CommentMsg
 * ==========
 */
impl<'a> From<(&'a BriefComment, &'a BriefAccount)> for OutgoingMsg<'a> {
    fn from(source: (&'a BriefComment, &'a BriefAccount)) -> OutgoingMsg<'a> {
        let (comment, account) = source;
        OutgoingMsg::Comment(CommentMsg {
            account_name: account.name(),
            comment: comment.comment(),
            choice: comment.choice_name(),
        })
    }
}
