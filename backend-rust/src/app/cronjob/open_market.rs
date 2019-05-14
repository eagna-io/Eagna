use crate::{
    app::FailureResponse,
    domain::{
        models::market::MarketId,
        services::{market_store::UpdateMarketStatusToOpenResult, MarketStore, UserStore},
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

    let prepared_markets = store.query_markets_ready_to_open().map_err(|e| {
        dbg!(e);
        FailureResponse::ServerError
    })?;
    if prepared_markets.is_empty() {
        return Ok(Response::text("No market is opened"));
    }

    let user_ids = store.query_all_user_ids().map_err(|e| {
        dbg!(e);
        FailureResponse::ServerError
    })?;

    let open_market_ids: Vec<MarketId> = prepared_markets.iter().map(|m| m.base.id).collect();

    for prepared_market in prepared_markets {
        let open_market = prepared_market.open_uncheck(&user_ids);
        match store.update_market_status_to_open(&open_market) {
            UpdateMarketStatusToOpenResult::Success => {}
            UpdateMarketStatusToOpenResult::NotPrepared => panic!(
                "Logic Error : try to open unprepared market {:?}",
                open_market.base.id
            ),
            UpdateMarketStatusToOpenResult::Error(e) => {
                dbg!(e);
                return Err(FailureResponse::ServerError);
            }
        }
    }

    Ok(Response::json(&open_market_ids))
}
