use super::ApiOrderModel;
use crate::app::{get_params, validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::market::*;
use rouille::{Request, Response};

pub fn get_list(
    infra: &InfraManager,
    req: &Request,
    market_id: MarketId,
) -> Result<Response, FailureResponse> {
    let postgres = infra.get_postgres()?;
    let market_repo = MarketRepository::from(postgres);

    let market = match market_repo.query_market(&market_id)? {
        Some(m) => m,
        None => return Err(FailureResponse::ResourceNotFound),
    };

    if let Some("true") = get_params(req, "mine").next() {
        let access_token = validate_bearer_header(infra, req)?;
        let my_orders = market
            .orders()
            .iter_related_to_user(&access_token.user_id)
            .map(|o| ApiOrderModel::from(o.clone()))
            .collect();
        Ok(Response::json(&RespBody { orders: my_orders }))
    } else {
        let orders = market
            .orders()
            .filter_normal_orders()
            .map(|o| ApiOrderModel::from(Order::from(o.clone()))) // TODO : stop clone
            .collect();

        Ok(Response::json(&RespBody { orders: orders }))
    }
}

#[derive(Debug, Serialize)]
struct RespBody {
    orders: Vec<ApiOrderModel>,
}
