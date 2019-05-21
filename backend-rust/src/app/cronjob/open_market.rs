use crate::{
    app::FailureResponse,
    domain::{
        models::market::Market,
        services::{MarketStore, UserStore},
    },
};
use rouille::{Request, Response};

pub fn get<S>(mut store: S, req: &Request) -> Result<Response, FailureResponse>
where
    S: MarketStore + UserStore,
{
    // 特定のソースからのリクエストかチェック
    // gcp app engine によるcron jobリクエストは10.0.0.1から
    // 開発環境によるcron jobリクエストはloopbackアドレスから
    let source = req.remote_addr().ip();
    if !source.is_loopback() && source != std::net::Ipv4Addr::new(10, 0, 0, 1) {
        return Err(FailureResponse::ResourceNotFound);
    }

    let prepared_market_ids = store.query_market_ids_ready_to_open()?;
    if prepared_market_ids.is_empty() {
        return Ok(Response::text("No market is opened"));
    }

    let user_ids = store.query_all_user_ids()?;

    let mut open_market_ids = Vec::with_capacity(prepared_market_ids.len());

    for market_id in prepared_market_ids {
        let mut locked_store = store.lock_market(&market_id)?;
        match locked_store.query_market(&market_id)?.unwrap() {
            Market::Preparing(m) => {
                let open_market = m.open_uncheck(&user_ids);
                locked_store.update_market_status_to_open(&open_market)?;
                open_market_ids.push(market_id);
            }
            _ => {
                // query_market_ids_ready_to_open からここまでの間に
                // 他のプロセスによってOpen処理がなされていた場合
                continue;
            }
        };
    }
    store.commit()?;

    Ok(Response::json(&open_market_ids))
}
