use crate::{
    context::Context,
    error::Error,
    filters::auth,
    response::{self, Response},
    routes::ws::contests::_id::OutgoingMsg,
};
use crop_domain::contest::poll::{ChoiceName, DetailedPoll, Poll, PollId};
use crop_domain::contest::{Contest, ContestId, ContestRepository as _, DetailedContest};
use http::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;
use warp::{filters::ws::Message, reject::Rejection, Filter};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReqBody {
    resolved_choice: ChoiceName,
}

pub fn route(ctx: Context) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("contests" / ContestId / "polls" / PollId)
        .and(warp::filters::method::patch())
        .and(auth::admin())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |contest_id, poll_id, _admin, body| {
            ctx.clone()
                .handle_request(move |ctx| inner(contest_id, poll_id, ctx, body))
        })
        .recover(Error::recover)
        .unify()
}

async fn inner(
    contest_id: ContestId,
    poll_id: PollId,
    ctx: Context,
    body: ReqBody,
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
            if poll.id() != poll_id {
                return Err(Error::new(StatusCode::NOT_FOUND, "poll id mismatch"));
            }

            let resolved = poll.resolve(body.resolved_choice).map_err(|e| {
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
