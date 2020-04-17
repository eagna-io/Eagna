use crate::{
    context::Context,
    error::Error,
    filters::auth,
    response::{self, Response},
};
use crop_domain::account::Authenticated;
use crop_domain::contest::poll::{BriefPoll, ChoiceName, Poll, PollId};
use crop_domain::contest::{Contest, ContestId, ContestRepository as _, DetailedContest};
use http::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;
use warp::{reject::Rejection, Filter};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReqBody {
    choice: ChoiceName,
}

pub fn route(ctx: Context) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("contests" / ContestId / "polls" / PollId / "my_choice")
        .and(warp::filters::method::put())
        .and(auth::account())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |contest_id, poll_id, account, body| {
            ctx.clone()
                .handle_request(move |ctx| inner(contest_id, poll_id, account, body, ctx))
        })
        .recover(Error::recover)
        .unify()
}

async fn inner(
    contest_id: ContestId,
    poll_id: PollId,
    account: Authenticated,
    body: ReqBody,
    ctx: Context,
) -> Result<Response, Error> {
    ctx.pg
        .with_conn::<Result<Response, Error>, _>(move |conn| {
            let contest = conn
                .query_by_id::<DetailedContest<BriefPoll>>(&contest_id)?
                .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;
            let poll = contest
                .current_poll()
                .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest has no poll"))?;
            if *poll.id() == poll_id {
                let updated = poll
                    .update_account_choice(&account, body.choice)
                    .map_err(|e| {
                        log::info!("Failed to update account choice : {:?}", e);
                        Error::new(StatusCode::BAD_REQUEST, "Failed to update")
                    })?;
                conn.save(&updated)?;
                Ok(response::new(StatusCode::OK, &"updated"))
            } else {
                Err(Error::new(StatusCode::NOT_FOUND, "poll id mismatch"))
            }
        })
        .await?
}
