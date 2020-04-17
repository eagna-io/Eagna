use crate::{
    context::Context,
    error::Error,
    filters::auth,
    response::{self, Response},
    routes::ws::contests::_id::PollMsgSource,
};
use chrono::Duration;
use crop_domain::contest::poll::{self, Choice, DetailedPoll, Poll, PollId};
use crop_domain::contest::{Contest, ContestId, ContestRepository as _, DetailedContest};
use http::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use warp::{reject::Rejection, Filter};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReqBody {
    title: String,
    duration_sec: Option<i32>,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct ResBody<'a>(&'a PollId);

pub fn route(ctx: Context) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("contests" / ContestId / "polls")
        .and(warp::filters::method::post())
        .and(auth::admin())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |contest_id, _admin, body| {
            ctx.clone()
                .handle_request(move |ctx| inner(ctx, contest_id, body))
        })
        .recover(Error::recover)
        .unify()
}

async fn inner(ctx: Context, contest_id: ContestId, body: ReqBody) -> Result<Response, Error> {
    // PollをDBに追加する
    let poll = ctx
        .pg
        .with_conn::<Result<poll::New, Error>, _>(move |conn| {
            let duration = body.duration_sec.map(|s| Duration::seconds(s as i64));
            let contest = conn
                .query_by_id::<DetailedContest<DetailedPoll>>(&contest_id)?
                .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;
            let added = contest.add_poll(body.title, duration, body.choices)?;
            conn.save(&added)?;
            Ok(added.poll)
        })
        .await??;

    let poll_id = *poll.id();

    // 指定時間後にPollをCloseする
    if let Some(dur) = poll.duration() {
        let _ = tokio::spawn(start_close_timer(ctx.clone(), *dur, contest_id, poll_id));
    }

    // 追加したPollをブロードキャストする
    let msg_source = PollMsgSource::from(poll);
    ctx.contest_manager
        .broadcast_msg(contest_id, msg_source)
        .await;

    Ok(response::new(StatusCode::CREATED, &ResBody(&poll_id)))
}

async fn start_close_timer(ctx: Context, dur: Duration, contest_id: ContestId, poll_id: PollId) {
    // Closeするまで待つ
    let tokio_dur = tokio::time::Duration::from_secs(dur.num_seconds() as u64);
    tokio::time::delay_for(tokio_dur).await;

    close_poll(ctx, contest_id, poll_id).await;
}

async fn close_poll(ctx: Context, contest_id: ContestId, poll_id: PollId) {
    let msg_source = ctx
        .pg
        .with_conn(move |conn| {
            // TODO: DetailedContestである必要ない。MinumumContestでいい。
            let contest = conn
                .query_by_id::<DetailedContest<DetailedPoll>>(&contest_id)
                .unwrap()
                .unwrap();
            let poll = contest.current_poll().unwrap();

            if *poll.id() != poll_id {
                panic!("A new poll is created while prev poll is not closed");
            }

            let closed = poll.clone().close().unwrap();
            conn.save(&closed).unwrap();

            PollMsgSource::from(closed)
        })
        .await
        .unwrap();

    // CloseMsgをブロードキャスト
    ctx.contest_manager
        .broadcast_msg(contest_id, msg_source)
        .await;
}
