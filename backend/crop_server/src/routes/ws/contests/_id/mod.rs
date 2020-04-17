use crate::{context::Context, error::Error, filters::auth};
use crop_domain::account::{self, Account};
use crop_domain::contest::poll::DetailedPoll;
use crop_domain::contest::{Contest, ContestId, ContestRepository as _, DetailedContest};
use futures::{
    sink::{Sink, SinkExt as _},
    stream::{StreamExt as _, TryStreamExt as _},
};
use http::StatusCode;
use warp::{filters::ws::Message, reject::Rejection, reply::Reply, Filter};

mod msg;

pub use msg::*;

pub fn route(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("contests" / ContestId)
        .and(auth::account())
        .and(warp::filters::ws::ws())
        .and_then(move |contest_id, account, ws| inner(ws, ctx.clone(), contest_id, account))
}

async fn inner(
    ws: warp::filters::ws::Ws,
    ctx: Context,
    contest_id: ContestId,
    account: account::Authenticated,
) -> Result<impl Reply, Rejection> {
    let contest = query_contest(ctx.clone(), contest_id)
        .await
        .map_err(Into::<Rejection>::into)?;
    Ok(ws.on_upgrade(move |ws| ws_handler(ws, ctx, contest, account)))
}

async fn query_contest(
    ctx: Context,
    contest_id: ContestId,
) -> Result<DetailedContest<DetailedPoll>, Error> {
    ctx.pg
        .with_conn(move |conn| conn.query_by_id::<DetailedContest<DetailedPoll>>(&contest_id))
        .await??
        .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "Contest not found"))
}

// 1. 現在のPollを送信する
// 2. Pollのupdateが起こるたびにPollを送信できるように
//    subscribeする。
async fn ws_handler(
    ws: warp::filters::ws::WebSocket,
    ctx: Context,
    contest: DetailedContest<DetailedPoll>,
    account: account::Authenticated,
) {
    let mut msg_sink = ws.sink_err_into::<anyhow::Error>();

    send_initial_poll_msg(&contest, &account, &mut msg_sink).await;

    subscribe_msgs(ctx, &contest, &account, &mut msg_sink).await;
}

// Poll msgを送信する
async fn send_initial_poll_msg(
    contest: &DetailedContest<DetailedPoll>,
    account: &account::Authenticated,
    msg_sink: &mut (impl Sink<Message, Error = anyhow::Error> + Unpin),
) {
    if let Some(poll) = contest.current_poll() {
        let msg = PollMsgSource::from(poll).to_msg(account.id());
        msg_sink
            .send_all(&mut futures::stream::once(futures::future::ok(msg)))
            .await
            .unwrap_or_else(|e| log::debug!("{:?}", e))
    }
}

// Pollの更新通知を受け取る
async fn subscribe_msgs(
    ctx: Context,
    contest: &DetailedContest<DetailedPoll>,
    account: &account::Authenticated,
    msg_sink: &mut (impl Sink<Message, Error = anyhow::Error> + Unpin + Send),
) {
    if let Some(subscriber) = ctx.contest_manager.subscribe(contest.id()).await {
        let account_id = *account.id();
        let mut stream = subscriber
            .err_into::<anyhow::Error>()
            .map_ok(move |msg_source| msg_source.to_msg(&account_id))
            .boxed(); // このboxedをなくすと、なぜかコンパイルエラーが出る
                      // async環境でclosureを使うのまだ深く理解してない
                      // コンパイラもまだ未発達で、
                      // 有意なエラーを返してくれない

        msg_sink
            .send_all(&mut stream)
            .await
            .unwrap_or_else(|e| log::debug!("{:?}", e))
    }
}
