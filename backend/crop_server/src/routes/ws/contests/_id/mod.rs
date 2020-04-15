use crate::{context::Context, error::Error};
use crop_domain::contest::poll::DetailedPoll;
use crop_domain::contest::{Contest, ContestId, ContestRepository as _, DetailedContest};
use futures::{sink::SinkExt as _, stream::TryStreamExt as _};
use http::StatusCode;
use warp::{reject::Rejection, reply::Reply, Filter};

mod msg;

pub use msg::OutgoingMsg;

pub fn route(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("contests" / ContestId)
        .and(warp::filters::ws::ws())
        .and_then(move |contest_id, ws| inner(ws, ctx.clone(), contest_id))
}

async fn inner(
    ws: warp::filters::ws::Ws,
    ctx: Context,
    contest_id: ContestId,
) -> Result<impl Reply, Rejection> {
    let contest = query_contest(ctx.clone(), contest_id)
        .await
        .map_err(|e| Into::<Rejection>::into(e))?;
    Ok(ws.on_upgrade(move |ws| ws_handler(ws, ctx, contest)))
}

async fn query_contest(
    ctx: Context,
    contest_id: ContestId,
) -> Result<DetailedContest<DetailedPoll>, Error> {
    ctx.pg
        .with_conn(move |conn| conn.query_by_id::<DetailedContest<DetailedPoll>>(&contest_id))
        .await??
        .ok_or(Error::new(StatusCode::NOT_FOUND, "Contest not found"))
}

// 1. 現在のPollを送信する
// 2. Pollのupdateが起こるたびにPollを送信できるように
//    subscribeする。
async fn ws_handler(
    ws: warp::filters::ws::WebSocket,
    ctx: Context,
    contest: DetailedContest<DetailedPoll>,
) {
    let mut msg_sink = ws.sink_err_into::<anyhow::Error>();

    // Poll msgを送信する
    if let Some(poll) = contest.current_poll() {
        let msg = OutgoingMsg::from(poll).into();
        if let Err(e) = msg_sink
            .send_all(&mut futures::stream::once(futures::future::ok(msg)))
            .await
        {
            log::error!("{:?}", e);
        }
    }

    // Pollの更新通知を受け取る
    if let Some(mut msg_stream) = ctx
        .contest_manager
        .subscribe(contest.id())
        .await
        .map(|stream| stream.err_into::<anyhow::Error>())
    {
        msg_sink
            .send_all(&mut msg_stream)
            .await
            .unwrap_or_else(|e| log::debug!("{:?}", e))
    }
}
