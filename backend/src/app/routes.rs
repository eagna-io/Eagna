//! # Develop Design Note
//! APIサーバーのルーティングに対する知識をもつ。
//! このモジュール以下に、各ルートのハンドラを記述する。
//! 基本的に、各ルート1ファイルで記述する。
//! 例えば、GET /users/ で1ファイル、POST /users/ で1ファイル。
//!
//! また、各ルートの Request や Response パラメーターは、
//! 基本的に primitive な型のみで構築されるべき。
//! なぜならそれらは外部アプリとの接点であるため、
//! どんなパラメータなのかが明示的にわからなければならない。
mod cronjob;
mod markets;
mod users;

use super::{FailureResponse, InfraManager};

use rouille::{router, Request, Response};
use uuid::Uuid;

pub fn routing(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    router!(req,
        (POST) (/users/invitation/) => {
            users::invitation::post::handler(infra, req)
        },
        (POST) (/users/me/access_token/) => {
            users::me::access_token::post::handler(infra, req)
        },
        (DELETE) (/users/me/access_token/) => {
            users::me::access_token::delete::handler(infra, req)
        },
        (POST) (/users/me/) => {
            users::me::post::handler(infra, req)
        },
        (GET) (/users/me/) => {
            users::me::get(infra, req)
        },
        (GET) (/markets/) => {
            markets::get_list(infra, req)
        },
        (POST) (/markets/) => {
            markets::post(infra, req)
        },
        (GET) (/markets/{id: Uuid}/) => {
            markets::get(infra, req, id)
        },
        (PUT) (/markets/{id: Uuid}/) => {
            markets::put(infra, req, id)
        },
        (GET) (/markets/{id: Uuid}/orders/) => {
            markets::orders::get_list(infra, req, id)
        },
        (POST) (/markets/{id: Uuid}/orders/) => {
            markets::orders::post(infra, req, id)
        },
        (GET) (/cronjob/check_markets/) => {
            cronjob::check_markets::get(infra, req)
        },
        _ => Err(FailureResponse::ResourceNotFound)
    )
}
