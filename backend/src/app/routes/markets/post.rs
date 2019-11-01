use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::{lmsr, market::*, organizer::*, point::*, user::*};
use crate::infra::postgres::{transaction, PostgresInfra};
use crate::primitive::{NonEmptyString, NonEmptyVec};

use chrono::{DateTime, Utc};
use rouille::{input::json::json_input, Request, Response};
use uuid::Uuid;

pub fn post(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(infra, req)?;

    let postgres = infra.get_postgres()?;
    let new_market = transaction(postgres, || {
        let user_repo = UserRepository::from(postgres);
        authorize(user_repo, &access_token.user_id)?;

        let req_market = json_input::<ReqPostMarket>(req).map_err(|e| {
            log::info!("Invalid payload error : {:?}", e);
            FailureResponse::InvalidPayload
        })?;

        let organizer = query_organizer(postgres, &req_market.organizer_id)?;
        let new_market = create_new_market(req_market, &organizer);

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

fn query_organizer(
    postgres: &dyn PostgresInfra,
    organizer_id: &Uuid,
) -> Result<Organizer, FailureResponse> {
    let organizer_repo = OrganizerRepository::from(postgres);

    match organizer_repo.query_organizer(&OrganizerId::from(*organizer_id))? {
        Some(organizer) => Ok(organizer),
        None => {
            log::warn!("Client try to create a new market with invalid organizer id");
            Err(FailureResponse::InvalidPayload)
        }
    }
}

fn create_new_market(req: ReqPostMarket, organizer: &Organizer) -> Market {
    let new_tokens = NonEmptyVec::from_vec(
        req.tokens
            .into_iter()
            .map(create_new_market_token)
            .collect(),
    )
    .unwrap();
    let new_prizes = NonEmptyVec::from_vec(
        req.prizes
            .into_iter()
            .enumerate()
            .map(|(i, prize)| create_new_market_prize(i as i32, prize))
            .collect(),
    )
    .unwrap();
    Market::new(
        req.title,
        organizer,
        req.description,
        lmsr::B::from(req.lmsr_b),
        Point::from(req.total_reward_point),
        req.open,
        req.close,
        new_tokens,
        new_prizes,
    )
}

fn create_new_market_token(req: ReqMarketToken) -> MarketToken {
    MarketToken::new(req.name, req.description, req.thumbnail_url)
}

fn create_new_market_prize(id: i32, req: ReqMarketPrize) -> MarketPrize {
    MarketPrize::new(id, req.name, req.thumbnail_url, req.target)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqPostMarket {
    title: NonEmptyString,
    organizer_id: Uuid,
    description: String,
    lmsr_b: u32,
    total_reward_point: u32,
    open: DateTime<Utc>,
    close: DateTime<Utc>,
    tokens: NonEmptyVec<ReqMarketToken>,
    prizes: NonEmptyVec<ReqMarketPrize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqMarketToken {
    name: NonEmptyString,
    description: String,
    thumbnail_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqMarketPrize {
    name: NonEmptyString,
    thumbnail_url: String,
    target: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResPostMarket<'a> {
    id: &'a Uuid,
}
