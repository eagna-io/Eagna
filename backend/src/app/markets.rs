pub mod orders;
pub use get::{get, get_list};
pub use post::post;
pub use put::put;

mod get {
    use crate::app::{get_params, FailureResponse, InfraManager};
    use crate::domain::market::*;
    use rouille::{Request, Response};

    pub fn get(
        infra: InfraManager,
        _req: &Request,
        market_id: MarketId,
    ) -> Result<Response, FailureResponse> {
        let postgres = infra.get_postgres()?;
        let market_repo = MarketRepository::from(postgres);

        let market = match market_repo.query_market(&market_id)? {
            Some(market) => market,
            None => return Err(FailureResponse::ResourceNotFound),
        };
        let resolve_token_name = match &market {
            Market::Resolved(ref m) => Some(&m.resolve_token_name),
            _ => None,
        };

        Ok(Response::json(&GetMarketResponse::from(market)))
    }

    pub fn get_list(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
        let status_iter = get_params(req, "status").filter_map(|s| match s {
            "upcoming" => Some(MarketStatus::Preparing),
            "open" => Some(MarketStatus::Open),
            "closed" => Some(MarketStatus::Closed),
            "resolved" => Some(MarketStatus::Settled),
            _ => {
                log::info!("Received invalid status query : [{}]", s);
                None
            }
        });

        let market_repo = MarketRepository::from(infra.get_postgres()?);
        let markets = market_repo.query_markets_with_status(status_iter)?;
        let resp_data = markets.into_iter().map(GetMarketResponse::from).collect();

        Ok(Response::json(&resp_data))
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct GetMarketResponse {
        #[serde(flatten)]
        attrs: MarketAttrs,
        status: MarketStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        resolve_token_name: Option<TokenName>,
    }

    impl From<Market> for GetMarketResponse {
        fn from(market: Market) -> GetMarketResponse {
            let resolve_token_name = match &market {
                Market::Resolved(ref m) => Some(m.resolve_token_name.clone()),
                _ => None,
            };
            GetMarketResponse {
                status: market.status(),
                attrs: market.into_attrs(),
                resolve_token_name: resolve_token_name,
            }
        }
    }
}

mod post {
    use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
    use crate::domain::{market::*, organizer::*, user::*};
    use crate::infra::PostgresInfra;
    use rouille::{input::json::json_input, Request, Response};

    pub fn post(infra: InfraManager, req: &Request) -> Result<Response, FailureResponse> {
        let access_token = validate_bearer_header(&infra, req)?;

        let postgres = infra.get_postgres()?;
        let market_id = {
            let postgres = postgres.transaction();

            authorize(postgres, &access_token.user_id)?;

            let req_market = json_input::<PostMarketRequest>(req).map_err(|e| {
                log::info!("Invalid payload error : {:?}", e);
                FailureResponse::InvalidPayload
            })?;

            let organizer = query_organizer(postgres, &req_market.organizer_id)?;

            let new_market = Market::new(
                req_market.title,
                &organizer,
                req_market.desc,
                req_market.lmsr_b,
                req_market.open,
                req_market.close,
                req_market.tokens,
                req_market.prizes,
            );
            let market_id = new_market.market_id.clone();

            let market_repo = MarketRepository::from(postgres);
            market_repo.save(new_market)?;

            postgres.commit()?;

            market_id
        };

        Ok(Response::json(&market_id).with_status_code(201))
    }

    // マーケットを作成する権限があるかチェック
    fn authorize(postgres: &dyn PostgresInfra, user_id: &UserId) -> Result<(), FailureResponse> {
        let user_repo = UserRepository::from(postgres);

        match user_repo.query_user(user_id)? {
            Some(user) => {
                if user.is_admin {
                    Ok(())
                } else {
                    Err(FailureResponse::Unauthorized)
                }
            }
            None => {
                log::error!("User does not exists, but AccessToken exists");
                Err(FailureResponse::ServerError)
            }
        };
    }

    fn query_organizer(
        postgres: &dyn PostgresInfra,
        organizer_id: &OrganizerId,
    ) -> Result<Organizer, FailureResponse> {
        let organizer_repo = OrganizerRepository::from(postgres);

        match organizer_repo.query_organizer(organizer_id)? {
            Some(organizer) => Ok(organizer),
            None => {
                log::warn!("Client try to create a new market with invalid organizer id");
                Err(FailureResponse::InvalidPayload)
            }
        }
    }

    #[derive(Debug, Deserialize)]
    // `id` フィールドは無視される
    struct PostMarketRequest {
        #[serde(flatten)]
        attrs: MarketAttrs,
    }
}

mod put {
    use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
    use crate::domain::{market::*, user::*};
    use crate::infra::PostgresInfra;
    use rouille::{input::json::json_input, Request, Response};

    pub fn put(
        infra: InfraManager,
        req: &Request,
        market_id: MarketId,
    ) -> Result<Response, FailureResponse> {
        let access_token = validate_bearer_header(&infra, req)?;

        let postgres = infra.get_postgres()?;
        {
            let postgres = postgres.transaction();

            authorize(postgres, &access_token.user_id)?;

            let req_data = json_input::<PutMarketRequest>(req).map_err(|e| {
                log::info!("Invalid payload : {:?}", e);
                FailureResponse::InvalidPayload
            })?;

            if req_data.status != MarketStatus::Settled {
                log::info!("Only resolving operation is supported");
                return Err(FailureResponse::InvalidPayload);
            }

            if req_data.resolve_token_name.is_none() {
                log::info!("resolve_token_name is not set");
                return Err(FailureResponse::InvalidPayload);
            }

            let market_repo = MarketRepository::from(postgres);
            market_repo.lock_market(&market_id)?;

            let closed_market = match market_repo.query_market(&market_id)? {
                Some(Market::Closed(m)) => m,
                Some(_) => {
                    log::info!("Would resolve market is not closed.");
                    return Err(FailureResponse::ResourceNotFound);
                }
                None => return Err(FailureResponse::ResourceNotFound),
            };

            let resolved_market = match closed_market.resolve(&req_data.resolve_token_name.unwrap())
            {
                Ok(m) => Market::from(m),
                Err(e) => {
                    log::info!("Failed to resolve market : {:?}", e);
                    return Err(FailureResponse::InvalidPayload);
                }
            };

            market_repo.save(resolved_market)?;

            postgres.commit()?;
        };

        Ok(Response::json(&market_id).with_status_code(201))
    }

    // マーケットを作成する権限があるかチェック
    fn authorize(postgres: &dyn PostgresInfra, user_id: &UserId) -> Result<(), FailureResponse> {
        let user_repo = UserRepository::from(postgres);

        match user_repo.query_user(user_id)? {
            Some(user) => {
                if user.is_admin {
                    Ok(())
                } else {
                    Err(FailureResponse::Unauthorized)
                }
            }
            None => {
                log::error!("User does not exists, but AccessToken exists");
                Err(FailureResponse::ServerError)
            }
        };
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct PutMarketRequest {
        status: MarketStatus,
        resolve_token_name: Option<TokenName>,
    }
}
