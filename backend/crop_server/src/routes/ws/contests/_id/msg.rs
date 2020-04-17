use chrono::{DateTime, Utc};
use crop_domain::account::{self, Account, AccountId};
use crop_domain::contest::comment::Comment;
use crop_domain::contest::poll::{self, Choice, ChoiceName, Poll, PollId, PollStatus, Stats};
use crop_domain::contest::{self, Contest};
use schemars::JsonSchema;
use serde::Serialize;
use std::collections::HashMap;
use warp::filters::ws::Message;

#[derive(Debug, Serialize, JsonSchema)]
pub enum OutgoingMsg<'a> {
    Poll(PollMsg<'a>),
    Comment(CommentMsg<'a>),
    /// Contestがcloseしたときに受け取るMsg
    /// 自分のスコア情報が載っている
    Closed(ClosedMsg),
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct PollMsg<'a> {
    id: &'a PollId,
    title: &'a str,
    created_at: &'a DateTime<Utc>,
    duration_sec: Option<i64>,
    idx: usize,
    #[schemars(with = "Vec<Choice>")]
    choices: &'a [Choice],
    resolved_choice: Option<&'a ChoiceName>,
    stats: Option<&'a Stats>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct CommentMsg<'a> {
    account_name: &'a str,
    comment: &'a str,
    choice: Option<&'a ChoiceName>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ClosedMsg {
    /// 何問のPollが出題されたか
    num_polls: usize,
    /// 何問正解したか
    account_score: Option<usize>,
}

impl<'a> OutgoingMsg<'a> {
    fn into_msg(&self) -> Message {
        Message::text(serde_json::to_string(self).unwrap())
    }
}

impl<'a> Into<Message> for OutgoingMsg<'a> {
    fn into(self) -> Message {
        self.into_msg()
    }
}

/*
 * ============
 * MsgSource
 * ============
 */
pub trait OutgoingMsgSource: Sync + Send {
    fn into_out_msg<'a>(&'a self, account_id: &'a AccountId) -> OutgoingMsg<'a>;

    fn into_msg(&self, account_id: &AccountId) -> Message {
        self.into_out_msg(account_id).into_msg()
    }
}

/*
 * ==========
 * PollMsgSource
 * ==========
 */
pub struct PollMsgSource<P> {
    poll: P,
    stats: Option<Stats>,
}

impl<P> From<P> for PollMsgSource<P>
where
    P: Poll + poll::WithAttrs + poll::WithUserChoices,
{
    fn from(poll: P) -> PollMsgSource<P> {
        let stats = match poll.status() {
            PollStatus::Open => None,
            PollStatus::Closed => Some(poll.compute_stats()),
        };
        PollMsgSource { poll, stats }
    }
}

impl<P> OutgoingMsgSource for PollMsgSource<P>
where
    P: Poll + poll::WithAttrs + poll::WithUserChoices + Send + Sync,
{
    fn into_out_msg<'a>(&'a self, _account_id: &'a AccountId) -> OutgoingMsg<'a> {
        OutgoingMsg::Poll(PollMsg {
            id: self.poll.id(),
            title: self.poll.title(),
            created_at: self.poll.created_at(),
            duration_sec: self.poll.duration().map(|d| d.num_seconds()),
            idx: self.poll.idx(),
            choices: self.poll.choices(),
            resolved_choice: self.poll.resolved_choice(),
            stats: self.stats.as_ref(),
        })
    }
}

/*
 * ==========
 * CommentMsgSource
 * ==========
 */
pub struct CommentMsgSource<C, A> {
    comment: C,
    account: A,
}

impl<C, A> From<(C, A)> for CommentMsgSource<C, A> {
    fn from(source: (C, A)) -> CommentMsgSource<C, A> {
        let (comment, account) = source;
        CommentMsgSource {
            comment: comment,
            account: account,
        }
    }
}

impl<C, A> OutgoingMsgSource for CommentMsgSource<C, A>
where
    C: Comment + Send + Sync,
    A: Account + account::WithAttrs + Send + Sync,
{
    fn into_out_msg<'a>(&'a self, _account_id: &'a AccountId) -> OutgoingMsg<'a> {
        OutgoingMsg::Comment(CommentMsg {
            account_name: self.account.name(),
            comment: self.comment.comment(),
            choice: self.comment.choice_name(),
        })
    }
}

/*
 * ===========
 * ClosedMsgSource
 * ===========
 */
pub struct ClosedMsgSource {
    num_polls: usize,
    account_scores: HashMap<AccountId, usize>,
}

impl<C> From<C> for ClosedMsgSource
where
    C: Contest + contest::WithAttrs + contest::WithPolls + contest::WithCurrentPoll,
    <C as contest::WithPolls>::Poll: poll::WithAttrs + poll::WithUserChoices,
    <C as contest::WithCurrentPoll>::Poll: poll::WithAttrs,
{
    fn from(contest: C) -> ClosedMsgSource {
        ClosedMsgSource {
            num_polls: contest.num_polls(),
            account_scores: contest.compute_account_scores(),
        }
    }
}

impl OutgoingMsgSource for ClosedMsgSource {
    fn into_out_msg<'a>(&'a self, account_id: &'a AccountId) -> OutgoingMsg<'a> {
        OutgoingMsg::Closed(ClosedMsg {
            num_polls: self.num_polls,
            account_score: self.account_scores.get(account_id).copied(),
        })
    }
}
