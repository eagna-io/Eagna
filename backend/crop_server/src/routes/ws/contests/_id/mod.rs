use crate::{context::Context, error::Error};
use crop_domain::account::{self, AccessToken, Account};
use crop_domain::contest::poll::DetailedPoll;
use crop_domain::contest::{Contest, ContestId, ContestRepository as _, DetailedContest};
use futures::prelude::*;
use http::StatusCode;
use warp::{filters::ws::Message, reject::Rejection, reply::Reply, Filter};

mod msg;

pub use msg::*;

pub fn route(ctx: Context) -> warp::filters::BoxedFilter<(impl Reply,)> {
    warp::path!("ws" / "contests" / ContestId / AccessToken)
        .and(warp::filters::ws::ws())
        .and_then(move |contest_id, access_token, ws| {
            inner(ws, ctx.clone(), contest_id, access_token)
        })
        .boxed()
}

async fn inner(
    ws: warp::filters::ws::Ws,
    ctx: Context,
    contest_id: ContestId,
    access_token: AccessToken,
) -> Result<impl Reply, Rejection> {
    let account = account::Authenticated::from(access_token);
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

    let stream1 = subscribe_msgs_stream(&ctx, contest.id(), *account.id());
    let stream2 = ping_stream();
    let mut merged_stream = futures::stream::select(stream1, stream2);

    msg_sink
        .send_all(&mut merged_stream)
        .await
        .unwrap_or_else(|e| log::debug!("{:?}", e))
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

// 定期的にPingを送信するStream
fn ping_stream() -> impl Stream<Item = anyhow::Result<Message>> + Unpin {
    tokio::time::interval(tokio::time::Duration::from_secs(5)).map(|_| Ok(Message::ping("hello")))
}

// Pollの更新通知を受け取るStream
fn subscribe_msgs_stream<'a>(
    ctx: &'a Context,
    contest_id: &'a ContestId,
    account_id: account::AccountId,
) -> impl Stream<Item = anyhow::Result<Message>> + Unpin + 'a {
    ctx.contest_manager
        .subscribe(contest_id) // Future<Option<Stream<Result<MsgSource, _>>>>
        .into_stream() // Stream<Option<Stream<_>>>
        .map(futures::stream::iter) // Stream<Stream<Stream<_>>
        .flatten() // Stream<Stream<Result<_, _>>>
        .flatten() // Stream<Result<_, _>>
        .err_into::<anyhow::Error>() // Stream<anyhow::Result<_>>
        .map_ok(move |msg_source| msg_source.to_msg(&account_id))
        .boxed()
}
