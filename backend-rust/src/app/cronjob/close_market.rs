use crate::{
    app::FailureResponse,
    domain::{
        models::market::MarketId,
        services::{market_store::UpdateMarketStatusResult, MarketStore, UserStore},
    },
};
use rouille::{Request, Response};

pub fn get<S>(store: &S, req: &Request) -> Result<Response, FailureResponse>
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

    let closing_markets = store.query_markets_ready_to_close().map_err(|e| {
        dbg!(e);
        FailureResponse::ServerError
    })?;
    if closing_markets.is_empty() {
        return Ok(Response::text("No market is opened"));
    }

    let closed_market_ids: Vec<MarketId> = closing_markets.iter().map(|m| m.base.id).collect();

    for closing_market in closing_markets {
        let closed_market = closing_market.close_uncheck();
        match store.update_market_status_to_closed(&closed_market) {
            UpdateMarketStatusResult::Success => {}
            UpdateMarketStatusResult::MarketNotFound => panic!(
                "logic error : try to close invalid market {:?}",
                closed_market.base.id
            ),
            UpdateMarketStatusResult::Error(e) => {
                dbg!(e);
                return Err(FailureResponse::ServerError);
            }
        }
    }
    Ok(Response::json(&closed_market_ids))
}
