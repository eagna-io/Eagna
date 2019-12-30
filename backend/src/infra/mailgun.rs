use hyper::{client::HttpConnector, Body, Client, Request, StatusCode};
use hyper_tls::HttpsConnector;
use std::borrow::Cow;

lazy_static::lazy_static! {
    static ref MAILGUN_DOMAIN: String = std::env::var("MAILGUN_DOMAIN").unwrap();
    static ref MAILGUN_API_KEY: String = std::env::var("MAILGUN_API_KEY").unwrap();
    static ref MAILGUN_MESSAGE_API_URI: String = format!(
        "https://api:{}@api.mailgun.net/v3/{}/messages",
        MAILGUN_API_KEY.as_str(), MAILGUN_DOMAIN.as_str()
    );

    static ref CLIENT: Client<HttpsConnector<HttpConnector>, Body> = {
        let https = HttpsConnector::new();
        Client::builder().build::<_, Body>(https)
    };
}

#[tokio::main]
pub async fn send_mail_inner(mail: Mail) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let req = Request::post(MAILGUN_MESSAGE_API_URI.as_str())
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&mail)?))?;

    let res = CLIENT.request(req).await?;
    if res.status() == StatusCode::OK {
        return Ok(());
    } else {
        log::warn!("Failed to send mail. StatusCode is {:?}", res.status());
        return Err("Failed to send mail".into());
    }
}

#[derive(Serialize)]
pub struct Mail {
    pub from: Cow<'static, str>,
    pub to: Cow<'static, str>,
    pub subject: Cow<'static, str>,
    pub html: Cow<'static, str>,
}
