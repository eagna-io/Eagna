use crate::{
    app::FailureResponse,
    domain::{
        models::market::{Market, MarketId},
        services::{MarketStore, UserStore},
    },
};
use rouille::{Request, Response};

pub fn get<S>(store: &mut S, req: &Request) -> Result<Response, FailureResponse>
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

    let close_markets = check_close(store)?;
    let open_markets = check_open(store)?;

    let resp_data = RespData {
        open_markets,
        close_markets,
    };

    Ok(Response::json(&resp_data))
}

#[derive(Serialize)]
struct RespData {
    open_markets: Vec<MarketId>,
    close_markets: Vec<MarketId>,
}

pub fn check_open<S>(store: &mut S) -> Result<Vec<MarketId>, FailureResponse>
where
    S: MarketStore + UserStore,
{
    let prepared_market_ids = store.query_market_ids_ready_to_open()?;

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

    Ok(open_market_ids)
}

pub fn check_close<S>(store: &mut S) -> Result<Vec<MarketId>, FailureResponse>
where
    S: MarketStore + UserStore,
{
    let closing_market_ids = store.query_market_ids_ready_to_close()?;

    let mut closed_market_ids = Vec::with_capacity(closing_market_ids.len());

    for market_id in closing_market_ids {
        let mut locked_store = store.lock_market(&market_id)?;
        // lockを獲得し、Open状態であることを保証する
        match locked_store.query_market(&market_id)?.unwrap() {
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

    Ok(closed_market_ids)
}
