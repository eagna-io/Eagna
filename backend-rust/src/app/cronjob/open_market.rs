use crate::{
    app::FailureResponse,
    domain::{
        models::market::MarketId,
        services::{market_store::UpdateMarketStatusErrorKind, MarketStore, UserStore},
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

    let prepared_markets = store.query_markets_ready_to_open()?;
    if prepared_markets.is_empty() {
        return Ok(Response::text("No market is opened"));
    }

    let user_ids = store.query_all_user_ids()?;
    let open_market_ids: Vec<MarketId> = prepared_markets.iter().map(|m| m.base.id).collect();

    for prepared_market in prepared_markets {
        let open_market = prepared_market.open_uncheck(&user_ids);
        match store.update_market_status_to_open(&open_market) {
            Ok(()) => {}
            Err(UpdateMarketStatusErrorKind::MarketNotFound) => panic!(
                "Logic Error : the store returns unprepared market as open market : {:?}",
                open_market.base.id
            ),
            Err(UpdateMarketStatusErrorKind::Error(e)) => {
                return Err(FailureResponse::from(e));
            }
        }
    }
    store.commit()?;

    Ok(Response::json(&open_market_ids))
}
