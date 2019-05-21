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

    let closing_market_ids = store.query_market_ids_ready_to_close()?;
    if closing_market_ids.is_empty() {
        return Ok(Response::text("No market is opened"));
    }

    let mut closed_market_ids = Vec::with_capacity(closing_market_ids.len());

    for market_id in closing_market_ids.iter() {
        let mut locked_store = store.lock_market(market_id)?;
        // lockを獲得し、Open状態であることを保証する
        match locked_store.query_market(market_id)?.unwrap() {
            Market::Open(m) => {
                let closed_market = m.close_uncheck();
                locked_store.update_market_status_to_closed(&closed_market)?;
                closed_market_ids.push(market_id);
            }
            _ => {
                // query_market_ids_ready_to_close からここまでの間に
                // 他のプロセスによってClose処理がされていた場合
                continue;
            }
        };
    }
    store.commit()?;

    Ok(Response::json(&closed_market_ids))
}
