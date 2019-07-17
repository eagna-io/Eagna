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

    let orders = market.orders();

    let resp_orders = orders
        .filter_normal_orders()
        .map(|o| ApiOrderModel::from(Order::from(o.clone()))) // TODO : stop clone
        .collect();

    let mut resp = RespBody {
        orders: resp_orders,
        my_orders: None,
    };

    if let Some("mine") = get_params(req, "contains").next() {
        let access_token = validate_bearer_header(infra, req)?;
        let my_orders = orders
            .iter_related_to_user(&access_token.user_id)
            .map(|o| ApiOrderModel::from(o.clone()))
            .collect();
        resp.my_orders = Some(my_orders);
    }

    Ok(Response::json(&resp))
}

#[derive(Debug, Serialize)]
struct RespBody {
    orders: Vec<ApiOrderModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    my_orders: Option<Vec<ApiOrderModel>>,
}
