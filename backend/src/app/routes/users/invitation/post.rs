use crate::app::{validate_bearer_header, FailureResponse, InfraManager};
use crate::domain::user::{repository::UserRepository, services::invitation::UserInviteService};
use crate::infra::mailgun::{send_mail, Mail};
use rouille::{input::json_input, Request, Response};

pub fn handler(infra: &InfraManager, req: &Request) -> Result<Response, FailureResponse> {
    let access_token = validate_bearer_header(infra, req)?;

    let user = UserRepository::from(infra.get_postgres()?)
        .query_user(&access_token.user_id)?
        .unwrap();
    if !user.is_admin() {
        return Err(FailureResponse::Unauthorized);
    }

    let ReqData {
        email,
        without_email,
    } = json_input(req).map_err(|_| FailureResponse::InvalidPayload)?;

    // token の生成
    let invitation_token = UserInviteService::publish_invitation_token(email.clone());

    // 招待メールの送信
    if !without_email {
        send_mail(Mail {
            from: "noreply@crop-pm.com".into(),
            to: email.into(),
            subject: "Cropへの招待が届きました!".into(),
            html: invitation_mail_html(invitation_token.as_str()).into(),
        })?;
    }

    Ok(Response::json(&ResData {
        token: invitation_token.as_str(),
    })
    .with_status_code(200))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqData {
    email: String,
    #[serde(default)] // false
    without_email: bool,
}

#[derive(Serialize)]
struct ResData<'a> {
    token: &'a str,
}

fn invitation_mail_html(token: &str) -> String {
    format!(
        r#"
        管理者からCropへの招待が届きました! <br />
        <a href="https://crop-pm.com/signup/{}">こちらのリンク</a> から参加登録をお願い致します!<br />
        <br />
        Crop運営
        "#,
        token
    )
}
