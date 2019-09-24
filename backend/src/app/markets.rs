pub mod orders;
pub use get::{get, get_list};
pub use post::post;
pub use put::put;

mod get {
    use crate::app::{
        get_param, get_params, validate_bearer_header, FailureResponse, InfraManager,
    };
    use crate::domain::market::*;
    use arrayvec::ArrayVec;
    use rouille::{Request, Response};

    pub fn get(
        infra: &InfraManager,
        _req: &Request,
        market_id: MarketId,
    ) -> Result<Response, FailureResponse> {
        let postgres = infra.get_postgres()?;
        let market_repo = MarketRepository::from(postgres);

        let market = match market_repo.query_market(&market_id)? {
            Some(market) => market,
            None => return Err(FailureResponse::ResourceNotFound),
        };

        Ok(Response::json(&GetMarketResponse::from(market)))
    }

    pub fn get_list(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
        let market_ids = query_market_ids(infra, req)?;
        let markets =
            MarketRepository::from(infra.get_postgres()?).query_markets(market_ids.as_slice())?;
        let resp_data: Vec<_> = markets.into_iter().map(GetMarketResponse::from).collect();

        Ok(Response::json(&resp_data))
    }

    fn query_market_ids(
        infra: &InfraManager,
        req: &Request,
    ) -> Result<Vec<MarketId>, FailureResponse> {
        if let Some("true") = get_param(req, "participated") {
            // ユーザーが参加している/参加したマーケット一覧を取得
            let access_token = validate_bearer_header(infra, req)?;
            Ok(MarketRepository::from(infra.get_postgres()?)
                .query_market_ids_participated_by_user(&access_token.user_id)?)
        } else {
            // 指定されたstatusのマーケット一覧を取得
            query_market_ids_by_status(infra, req)
        }
    }

    fn query_market_ids_by_status(
        infra: &InfraManager,
        req: &Request,
    ) -> Result<Vec<MarketId>, FailureResponse> {
        let mut statuses = ArrayVec::<[MarketStatus; 4]>::new();
        get_params(req, "status").for_each(|s| match s {
            "upcoming" => {
                let _ = statuses.try_push(MarketStatus::Upcoming);
            }
            "open" => {
                let _ = statuses.try_push(MarketStatus::Open);
            }
            "closed" => {
                let _ = statuses.try_push(MarketStatus::Closed);
            }
            "resolved" => {
                let _ = statuses.try_push(MarketStatus::Resolved);
            }
            _ => {
                log::info!("Received invalid status query : [{}]", s);
            }
        });
        if statuses.len() == 0 {
            statuses.push(MarketStatus::Upcoming);
            statuses.push(MarketStatus::Open);
            statuses.push(MarketStatus::Closed);
            statuses.push(MarketStatus::Resolved);
        }

        Ok(MarketRepository::from(infra.get_postgres()?)
            .query_market_ids_with_status(statuses.as_slice())?)
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct GetMarketResponse {
        id: MarketId,
        #[serde(flatten)]
        attrs: MarketAttrs,
        status: MarketStatus,
        token_distribution: TokenDistribution,
        #[serde(skip_serializing_if = "Option::is_none")]
        resolved_token_name: Option<TokenName>,
    }

    impl From<Market> for GetMarketResponse {
        fn from(market: Market) -> GetMarketResponse {
            let FlattenMarket {
                id,
                attrs,
                resolved_token_name,
                status,
                token_distribution,
                ..
            } = market.flatten();
            GetMarketResponse {
                id,
                status,
                attrs,
                token_distribution,
                resolved_token_name,
            }
        }
    }
}

mod post {
    use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
    use crate::domain::{market::*, organizer::*, user::*};
    use crate::infra::postgres::{transaction, PostgresInfra};
    use rouille::{input::json::json_input, Request, Response};

    pub fn post(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
        let access_token = validate_bearer_header(infra, req)?;

        let postgres = infra.get_postgres()?;
        let market_id = transaction(postgres, || {
            authorize(postgres, &access_token.user_id)?;

            let req_market = json_input::<PostMarketRequest>(req).map_err(|e| {
                log::info!("Invalid payload error : {:?}", e);
                FailureResponse::InvalidPayload
            })?;

            let organizer = query_organizer(postgres, &req_market.attrs.organizer_id)?;

            let new_market = Market::new(
                req_market.attrs.title,
                &organizer,
                req_market.attrs.description,
                req_market.attrs.lmsr_b,
                req_market.attrs.open,
                req_market.attrs.close,
                req_market.attrs.tokens,
                req_market.attrs.prizes,
            );
            let market_id = new_market.id().clone();

            let market_repo = MarketRepository::from(postgres);
            market_repo.save_market(&new_market)?;

            Ok::<_, FailureResponse>(market_id)
        })?;

        Ok(Response::json(&market_id).with_status_code(201))
    }

    // マーケットを作成する権限があるかチェック
    fn authorize(postgres: &dyn PostgresInfra, user_id: &UserId) -> Result<(), FailureResponse> {
        let user_repo = UserRepository::from(postgres);

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
    #[serde(rename_all = "camelCase")]
    struct PostMarketRequest {
        #[serde(flatten)]
        attrs: MarketAttrs,
    }
}

mod put {
    use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
    use crate::domain::{market::*, user::*};
    use crate::infra::postgres::{transaction, PostgresInfra};
    use rouille::{input::json::json_input, Request, Response};

    pub fn put(
        infra: &InfraManager,
        req: &Request,
        market_id: MarketId,
    ) -> Result<Response, FailureResponse> {
        let access_token = validate_bearer_header(infra, req)?;

        let postgres = infra.get_postgres()?;
        transaction(postgres, || {
            authorize(postgres, &access_token.user_id)?;

            let req_data = json_input::<PutMarketRequest>(req).map_err(|e| {
                log::warn!("Invalid payload : {:?}", e);
                FailureResponse::InvalidPayload
            })?;

            if req_data.status != MarketStatus::Resolved {
                log::info!("Only resolving operation is supported");
                return Err(FailureResponse::InvalidPayload);
            }

            if req_data.resolved_token_name.is_none() {
                log::info!("resolved_token_name is not set");
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

            let resolved_market = match closed_market.resolve(req_data.resolved_token_name.unwrap())
            {
                Ok(m) => m,
                Err(e) => {
                    log::info!("Failed to resolve market : {:?}", e);
                    return Err(FailureResponse::InvalidPayload);
                }
            };

            market_repo.save_market(&Market::from(resolved_market))?;

            Ok(())
        })?;

        Ok(Response::json(&market_id).with_status_code(201))
    }

    // マーケットを作成する権限があるかチェック
    fn authorize(postgres: &dyn PostgresInfra, user_id: &UserId) -> Result<(), FailureResponse> {
        let user_repo = UserRepository::from(postgres);

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

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct PutMarketRequest {
        status: MarketStatus,
        resolved_token_name: Option<TokenName>,
    }
}
