use crate::app::{FailureResponse, InfraManager};
use crate::domain::user::{
    models::{NewUser, User as _, UserEmail, UserName},
    repository::{access_token::AccessTokenRepository, UserRepository},
    services::{
        auth::UserAuthService,
        invitation::{Invitation, InvitationToken, UserInviteService},
    },
};
use crate::infra::mailgun::{send_mail, Mail};
use crate::primitive::NonEmptyString;
use rouille::{input::json_input, Request, Response};

pub fn handler(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let req_data = json_input::<ReqData>(req).map_err(|_| FailureResponse::InvalidPayload)?;
    let invitation_token = InvitationToken::from(req_data.invitation_token);

    // token のvalidation
    let Invitation { email } = UserInviteService::validate_invitation_token(&invitation_token)
        .map_err(|_| FailureResponse::Unauthorized)?;

    // User情報の登録
    let cred = UserAuthService::derive_credentials(req_data.password.as_str());
    let new_user = NewUser::new(
        UserName::from(req_data.name),
        UserEmail::from_str(email)?,
        cred,
    );
    UserRepository::from(infra.get_postgres()?)
        .save_user(&new_user)
        .map_err(|e| {
            log::warn!("Failed to save a new user.");
            log::warn!("----  {:?} : {:?}", new_user.name(), new_user.email());
            log::warn!("----  {:?}", e);
            FailureResponse::Conflict
        })?;

    // アクセストークンの発行
    let access_token = new_user.new_access_token();
    AccessTokenRepository::from(infra.get_redis()?).save(&access_token)?;

    if !req_data.without_email {
        // 登録成功メールの送信
        let user_email = <NewUser as Into<(_, _, UserEmail, _)>>::into(new_user).2;
        send_mail(Mail {
            from: "noreply@crop-pm.com".into(),
            to: user_email.into_string().into(),
            subject: "Cropへのご登録ありがとうございます!".into(),
            html: signup_mail_html().into(),
        })?;
    }

    Ok(Response::json(&ResData {
        token: access_token.id.as_str(),
    })
    .with_status_code(200))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqData {
    name: NonEmptyString,
    password: NonEmptyString,
    invitation_token: String,
    #[serde(default)] // false
    without_email: bool,
}

#[derive(Serialize)]
struct ResData<'a> {
    token: &'a str,
}

fn signup_mail_html() -> String {
    format!(
        r#"
        Cropへのご登録ありがとうございます! <br />
        <a href="https://crop-pm.com/account/">マイページへ</a><br />
        <br />
        Crop運営
        "#
    )
}
