use super::ResOrder;
use crate::{get_params, validate_bearer_header, FailureResponse, InfraManager};
use crop_domain::market::{
    models::{Market as _, MarketId},
    repository::MarketRepository,
};
use rouille::{Request, Response};
use uuid::Uuid;

pub fn get_list(
    infra: &InfraManager,
    req: &Request,
    market_id: Uuid,
) -> Result<Response, FailureResponse> {
    let postgres = infra.get_postgres()?;
    let market_repo = MarketRepository::from(postgres);

    let market = match market_repo.query_market(&MarketId::from(market_id))? {
        Some(m) => m,
        None => return Err(FailureResponse::ResourceNotFound),
    };

    if let Some("true") = get_params(req, "mine").next() {
        let access_token = validate_bearer_header(req)?;
        let my_orders = market
            .orders()
            .iter_related_to_user(&access_token.user_id)
            .map(ResOrder::from)
            .collect();
        Ok(Response::json(&RespBody { orders: my_orders }))
    } else {
        let orders = market.orders().iter().map(ResOrder::from).collect();

        Ok(Response::json(&RespBody { orders }))
    }
}

#[derive(Debug, Serialize)]
struct RespBody<'a> {
    orders: Vec<ResOrder<'a>>,
}
