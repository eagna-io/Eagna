use failure::{err_msg, Fallible};
use hyper::{client::HttpConnector, Body, Client, Request, StatusCode};
use hyper_tls::HttpsConnector;
use std::borrow::Cow;

lazy_static::lazy_static! {
    static ref MAILGUN_DOMAIN: String = std::env::var("MAILGUN_DOMAIN").unwrap();
    static ref MAILGUN_API_KEY: String = std::env::var("MAILGUN_API_KEY").unwrap();
    static ref MAILGUN_MESSAGE_API_URI: String = format!(
        "https://api.mailgun.net/v3/{}/messages",
        MAILGUN_DOMAIN.as_str()
    );
    static ref AUTH_HEADER_VAL: String = {
        let combined = format!("api:{}", MAILGUN_API_KEY.as_str());
        let encoded = base64::encode(&combined);
        format!("Basic {}", encoded)
    };
    static ref CLIENT: Client<HttpsConnector<HttpConnector>, Body> = {
        let https = HttpsConnector::new();
        Client::builder().build::<_, Body>(https)
    };
}

#[tokio::main]
pub async fn send_mail(mail: Mail) -> Fallible<()> {
    let req = Request::post(MAILGUN_MESSAGE_API_URI.as_str())
        .header("content-type", "application/x-www-form-urlencoded")
        .header("authorization", AUTH_HEADER_VAL.as_str())
        .body(Body::from(serde_urlencoded::to_string(&mail)?))?;

    let res = CLIENT.request(req).await?;
    if res.status() == StatusCode::OK {
        return Ok(());
    } else {
        let status = res.status();
        let bytes = hyper::body::to_bytes(res).await?;
        let body = std::str::from_utf8(bytes.as_ref()).unwrap();
        log::warn!(
            "Failed to send mail. StatusCode is {:?}, Body is {}",
            status,
            body
        );
        return Err(err_msg("Failed to send mail"));
    }
}

#[derive(Serialize)]
pub struct Mail {
    pub from: Cow<'static, str>,
    pub to: Cow<'static, str>,
    pub subject: Cow<'static, str>,
    pub html: Cow<'static, str>,
}

impl Mail {
    fn to_tuple_array(&self) -> [(&'static str, &Cow<'static, str>); 4] {
        [
            ("from", &self.from),
            ("to", &self.to),
            ("subject", &self.subject),
            ("html", &self.html),
        ]
    }
}
