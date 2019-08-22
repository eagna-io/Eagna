use crate::app::{FailureResponse, InfraManager};
use crate::domain::market::*;
use crate::infra::postgres::transaction;
use rouille::{Request, Response};

pub fn get(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    validate_request(req)?;

    let close_markets = check_close(infra)?;
    let open_markets = check_open(infra)?;

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

/// ## Reference
/// https://cloud.google.com/appengine/docs/flexible/custom-runtimes/scheduling-jobs-with-cron-yaml?hl=ja#validating_cron_requests
fn validate_request(req: &Request) -> Result<(), FailureResponse> {
    req.header("X-Appengine-Cron")
        .ok_or(FailureResponse::ResourceNotFound)
        .map(|_| ())
}

fn check_open(infra: &InfraManager) -> Result<Vec<MarketId>, FailureResponse> {
    // オープン可能なマーケットのid一覧を取得
    let openable_market_ids = {
        let repo = MarketRepository::from(infra.get_postgres()?);
        repo.query_market_ids_ready_to_open()?
    };
    let mut opened_market_ids = Vec::with_capacity(openable_market_ids.len());

    let postgres = infra.get_postgres()?;

    // 各マーケットをオープンしていく
    for market_id in openable_market_ids {
        // マーケットをロックする期間を短くするため、各マーケット毎にトランザクションを発行する
        transaction(postgres, || {
            let market_repo = MarketRepository::from(postgres);

            market_repo.lock_market(&market_id)?;

            if let Market::Upcoming(m) = market_repo.query_market(&market_id)?.unwrap() {
                let opened_market = m.try_open().unwrap();
                market_repo.save_market(&Market::from(opened_market))?;
                opened_market_ids.push(market_id);
            };

            Ok::<_, FailureResponse>(())
        })?;
    }

    Ok(opened_market_ids)
}

fn check_close(infra: &InfraManager) -> Result<Vec<MarketId>, FailureResponse> {
    // クローズ可能なマーケットのid一覧を取得
    let closable_market_ids = {
        let repo = MarketRepository::from(infra.get_postgres()?);
        repo.query_market_ids_ready_to_close()?
    };
    let mut closed_market_ids = Vec::with_capacity(closable_market_ids.len());

    let postgres = infra.get_postgres()?;

    // 各マーケットをクローズしていく
    for market_id in closable_market_ids {
        // マーケットをロックする期間を短くするため、各マーケット毎にトランザクションを発行する
        transaction(postgres, || {
            let market_repo = MarketRepository::from(postgres);
            market_repo.lock_market(&market_id)?;

            if let Market::Open(m) = market_repo.query_market(&market_id)?.unwrap() {
                let closed_market = m.try_close().unwrap();
                market_repo.save_market(&Market::from(closed_market))?;
                closed_market_ids.push(market_id);
            }

            Ok::<_, FailureResponse>(())
        })?;
    }

    Ok(closed_market_ids)
}
