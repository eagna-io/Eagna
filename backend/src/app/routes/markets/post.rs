use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::lmsr;
use crate::domain::market::{
    models::{Market as _, MarketToken},
    repository::MarketRepository,
    services::manager::{MarketManager, NewMarket},
};
use crate::domain::user::*;
use crate::infra::postgres::transaction;
use crate::primitive::{NonEmptyString, NonEmptyVec};
use chrono::{DateTime, Utc};
use rouille::{input::json::json_input, Request, Response};
use uuid::Uuid;

pub fn post(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(req)?;

    let postgres = infra.get_postgres()?;
    let new_market = transaction(postgres, || {
        let user_repo = UserRepository::from(postgres);
        authorize(user_repo, &access_token.user_id)?;

        let req_market = json_input::<ReqPostMarket>(req).map_err(|e| {
            log::info!("Invalid payload error : {:?}", e);
            FailureResponse::InvalidPayload
        })?;

        let new_market = create_new_market(req_market);

        let market_repo = MarketRepository::from(postgres);
        market_repo.save_market(&new_market)?;

        Ok::<_, FailureResponse>(new_market)
    })?;

    Ok(Response::json(&ResPostMarket {
        id: new_market.id().as_uuid(),
    })
    .with_status_code(201))
}

// マーケットを作成する権限があるかチェック
fn authorize(user_repo: UserRepository, user_id: &UserId) -> Result<(), FailureResponse> {
    match user_repo.query_user(user_id)? {
        Some(user) => {
            if user.is_admin() {
                Ok(())
            } else {
                Err(FailureResponse::Unauthorized)
            }
        }
        None => {
            log::error!("User does not exists, but AccessToken exists");
            Err(FailureResponse::ServerError)
        }
    }
}

fn create_new_market(req: ReqPostMarket) -> NewMarket {
    let new_tokens = NonEmptyVec::from_vec(
        req.tokens
            .into_iter()
            .map(create_new_market_token)
            .collect(),
    )
    .unwrap();
    MarketManager::create(
        req.title,
        req.description,
        lmsr::B::from(req.lmsr_b),
        req.open,
        req.close,
        new_tokens,
    )
}

fn create_new_market_token(req: ReqMarketToken) -> MarketToken {
    MarketToken::new(req.name, req.description, req.thumbnail_url)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqPostMarket {
    title: NonEmptyString,
    description: String,
    lmsr_b: u32,
    open: DateTime<Utc>,
    close: DateTime<Utc>,
    tokens: NonEmptyVec<ReqMarketToken>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqMarketToken {
    name: NonEmptyString,
    description: String,
    thumbnail_url: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResPostMarket<'a> {
    id: &'a Uuid,
}
