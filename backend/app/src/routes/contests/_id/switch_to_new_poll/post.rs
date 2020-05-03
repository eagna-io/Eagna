use crate::{
    context::Context,
    filters::auth,
    res::{handler_fn, response, Error, Response},
};
use domain::contest::{Contest, ContestId, ContestStatus};
use http::StatusCode;
use infra::pg::contest::ContestRepository;
use schemars::JsonSchema;
use serde::Deserialize;
use warp::Filter as _;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReqBody {
    title: String,
    duration_sec: usize,
    choices: Vec<Choice>,
}

pub fn route(ctx: Context) -> warp::filters::BoxedFilter<(Response,)> {
    warp::path!("contests" / ContestId / "switch_to_new_poll")
        .and(warp::filters::method::post())
        .and(auth::admin())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(move |contest_id, _token, body| handler_fn(move || inner(ctx, body, contest_id)))
        .recover(Error::recover)
        .unify()
        .boxed()
}

async fn inner(ctx: Context, body: ReqBody, contest_id: ContestId) -> Result<Response, Error> {
    let mut contest_repo = ContestRepository::new();

    let mut contest = contest_repo
        .query_by_id(contest_id)
        .await?
        .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"))?;

    let duration = Duration::seconds(body.duration_sec);
    contest
        .switch_to_new_poll(title, duration.clone(), choices)
        .map_err(|e| {
            log::info!("Failed to switch to new poll because of {:?}", e);
            Error::new(StatusCode::BAD_REQUEST, "Failed to switch to new poll")
        })?;

    contest_repo.save(contest).await?;

    tokio::spawn(start_close_timer(ctx.clone(), duration, contest_id));

    // TODO
    // 新しいPollをブロードキャストする

    Ok(response::new(StatusCode::OK, &"switched"))
}

async fn start_close_timer(ctx: Context, dur: Duration, contest_id: ContestId) {
    // Closeするまで待つ
    let tokio_dur = tokio::time::Duration::from_secs(dur.num_seconds() as u64);
    tokio::time::delay_for(tokio_dur).await;

    // Close処理をする
    let mut contest_repo = ContestRepository::new();

    let mut contest = contest_repo
        .query_by_id(contest_id)
        .await?
        .expect("Contest must exist");

    contest.close_poll().unwrap();

    // TODO
    // CloseしたPollをブロードキャストする
}
