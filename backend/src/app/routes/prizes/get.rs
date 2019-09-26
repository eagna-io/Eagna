use super::ResPrize;
use crate::app::{FailureResponse, InfraManager};
use crate::domain::prize::PrizeRepository;
use rouille::{Request, Response};

pub fn get_list(infra: &InfraManager, _req: &Request) -> Result<Response, FailureResponse> {
    let repo = PrizeRepository::from(infra.get_postgres()?);
    let prizes = repo.query_all_prizes()?;
    let res_prizes: Vec<ResPrize> = prizes.iter().map(ResPrize::from).collect();
    Ok(Response::json(&res_prizes))
}
