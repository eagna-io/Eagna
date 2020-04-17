use crate::{
    context::Context,
    error::Error,
    filters::auth,
    response::{self, Response},
};
use crop_domain::account::{Account as _, AccountRepository, Authenticated, BriefAccount};
use crop_domain::contest::comment::{BriefComment, Comment as _, CommentId};
use crop_domain::contest::poll::{DetailedPoll, Poll as _, PollId};
use crop_domain::contest::{Contest as _, ContestId, ContestRepository, DetailedContest};
use http::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use warp::{reject::Rejection, Filter};

#[derive(Deserialize, JsonSchema)]
pub struct ReqBody {
    comment: String,
}

#[derive(Serialize, JsonSchema)]
pub struct ResBody(CommentId);

pub fn route(ctx: Context) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("contests" / ContestId / "polls" / PollId / "comments")
        .and(warp::filters::method::post())
        .and(auth::account())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |contest_id, poll_id, account, body| {
            ctx.clone()
                .handle_request(move |ctx| inner(ctx, contest_id, poll_id, account, body))
        })
        .recover(Error::recover)
        .unify()
}

async fn inner(
    ctx: Context,
    contest_id: ContestId,
    poll_id: PollId,
    account: Authenticated,
    body: ReqBody,
) -> Result<Response, Error> {
    let (comment, brief_account) = ctx
        .pg
        .with_conn::<Result<(BriefComment, BriefAccount), Error>, _>(move |conn| {
            let contest = ContestRepository::query_by_id::<DetailedContest<DetailedPoll>>(
                &conn,
                &contest_id,
            )?
            .ok_or(Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;

            let poll = contest
                .current_poll()
                .ok_or(Error::new(StatusCode::NOT_FOUND, "Contest has no poll"))?;

            if *poll.id() != poll_id {
                return Err(Error::new(StatusCode::NOT_FOUND, "poll id mismatch"));
            }

            // コメントを追加する
            let comment_added = poll.add_comment(&account, body.comment);
            ContestRepository::save(&conn, &comment_added)?;

            // アカウント名を取得するためにアカウントを取得する
            let brief_account =
                AccountRepository::query_by_id::<BriefAccount>(&conn, account.id())?.unwrap();

            Ok((comment_added.comment, brief_account))
        })
        .await??;

    ctx.contest_manager
        .notify_update(contest_id, (&comment, &brief_account))
        .await;

    Ok(response::new(StatusCode::CREATED, &ResBody(*comment.id())))
}
