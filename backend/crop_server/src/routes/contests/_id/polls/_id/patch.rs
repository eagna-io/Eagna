use crate::{
    context::Context,
    error::Error,
    filters::auth,
    response::{self, Response},
    routes::ws::contests::_id::OutgoingMsg,
};
use crop_domain::contest::poll::{ChoiceName, DetailedPoll, Poll, PollId, PollStatus};
use crop_domain::contest::{Contest, ContestId, ContestRepository as _, DetailedContest};
use http::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;
use warp::{filters::ws::Message, reject::Rejection, Filter};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReqBody {
    status: Option<PollStatus>,
    resolved_choice: Option<ChoiceName>,
}

pub fn route(ctx: Context) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("contests" / ContestId / "polls" / PollId)
        .and(warp::filters::method::patch())
        .and(auth::admin())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |contest_id, poll_id, _admin, body| {
            ctx.clone()
                .handle_request(move |ctx| inner(contest_id, poll_id, body, ctx))
        })
        .recover(Error::recover)
        .unify()
}

async fn inner(
    contest_id: ContestId,
    poll_id: PollId,
    body: ReqBody,
    ctx: Context,
) -> Result<Response, Error> {
    match (body.status, body.resolved_choice) {
        (Some(PollStatus::Closed), None) => close_poll(contest_id, poll_id, ctx).await,
        (None, Some(resolved_choice)) => {
            resolve_poll(contest_id, poll_id, ctx, resolved_choice).await
        }
        _ => Err(Error::new(StatusCode::BAD_REQUEST, "Invalid body format")),
    }
}

async fn close_poll(
    contest_id: ContestId,
    poll_id: PollId,
    ctx: Context,
) -> Result<Response, Error> {
    let msg = ctx
        .pg
        .with_conn::<Result<Message, Error>, _>(move |conn| {
            // TODO: DetailedContestである必要ない。MinumumContestでいい。
            let contest = conn
                .query_by_id::<DetailedContest<DetailedPoll>>(&contest_id)?
                .ok_or(Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;
            let poll = contest
                .current_poll()
                .ok_or(Error::new(StatusCode::NOT_FOUND, "Contest has no poll"))?;
            if *poll.id() != poll_id {
                return Err(Error::new(StatusCode::NOT_FOUND, "poll id mismatch"));
            }

            let closed = poll.close().map_err(|e| {
                log::info!("Failed to close poll : {:?}", e);
                Error::new(StatusCode::BAD_REQUEST, "Failed to close poll")
            })?;
            conn.save(&closed)?;

            let msg = OutgoingMsg::from(&closed).into();

            Ok(msg)
        })
        .await??;

    ctx.contest_manager.broadcast_msg(contest_id, msg).await;

    Ok(response::new(StatusCode::OK, &"resolved"))
}

async fn resolve_poll(
    contest_id: ContestId,
    poll_id: PollId,
    ctx: Context,
    resolved_choice: ChoiceName,
) -> Result<Response, Error> {
    let msg = ctx
        .pg
        .with_conn::<Result<Message, Error>, _>(move |conn| {
            // TODO: DetailedContestである必要ない。MinumumContestでいい。
            let contest = conn
                .query_by_id::<DetailedContest<DetailedPoll>>(&contest_id)?
                .ok_or(Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;
            let poll = contest
                .current_poll()
                .ok_or(Error::new(StatusCode::NOT_FOUND, "Contest has no poll"))?;
            if *poll.id() != poll_id {
                return Err(Error::new(StatusCode::NOT_FOUND, "poll id mismatch"));
            }

            let resolved = poll.resolve(resolved_choice).map_err(|e| {
                log::info!("Failed to resolve poll : {:?}", e);
                Error::new(StatusCode::BAD_REQUEST, "Failed to resolve poll")
            })?;
            conn.save(&resolved)?;

            let msg = OutgoingMsg::from(&resolved).into();

            Ok(msg)
        })
        .await??;

    ctx.contest_manager.broadcast_msg(contest_id, msg).await;

    Ok(response::new(StatusCode::OK, &"resolved"))
}
