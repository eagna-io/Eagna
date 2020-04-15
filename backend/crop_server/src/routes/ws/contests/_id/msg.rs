use chrono::{DateTime, Utc};
use crop_domain::contest::poll::{
    self, ChoiceColor, ChoiceName, DetailedPoll, Poll, PollId, Stats,
};
use schemars::JsonSchema;
use serde::Serialize;
use std::collections::HashMap;
use warp::filters::ws::Message;

#[derive(Debug, Serialize, JsonSchema)]
pub enum OutgoingMsg<'a> {
    Poll(PollMsg<'a>),
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct PollMsg<'a> {
    id: PollId,
    title: &'a str,
    created_at: &'a DateTime<Utc>,
    duration_sec: Option<i64>,
    choices: &'a HashMap<ChoiceName, ChoiceColor>,
    resolved_choice: Option<&'a ChoiceName>,
    stats: Option<Stats>,
}

impl<'a> Into<Message> for OutgoingMsg<'a> {
    fn into(self) -> Message {
        Message::text(serde_json::to_string(&self).unwrap())
    }
}

impl<'a> From<&'a DetailedPoll> for OutgoingMsg<'a> {
    fn from(poll: &'a DetailedPoll) -> OutgoingMsg<'a> {
        let stats = if poll.is_closed() {
            Some(poll.compute_stats())
        } else {
            None
        };
        OutgoingMsg::Poll(PollMsg {
            id: poll.id(),
            title: poll.title(),
            created_at: poll.created_at(),
            duration_sec: poll.duration().map(|d| d.num_seconds()),
            choices: poll.choices(),
            resolved_choice: poll.resolved_choice(),
            stats,
        })
    }
}

impl<'a> From<&'a poll::New> for OutgoingMsg<'a> {
    fn from(poll: &'a poll::New) -> OutgoingMsg<'a> {
        OutgoingMsg::Poll(PollMsg {
            id: poll.id(),
            title: poll.title(),
            created_at: poll.created_at(),
            duration_sec: poll.duration().map(|d| d.num_seconds()),
            choices: poll.choices(),
            resolved_choice: None,
            stats: None,
        })
    }
}
