use super::InfraFactory;
use std::sync::Arc;

const API_BASE: &str = "https://www.googleapis.com/identitytoolkit/v3/relyingparty";

pub trait FirebaseInfra: Send + 'static {
    fn query_user_by_access_token(
        &self,
        access_token_id: &str,
    ) -> Result<Option<String>, failure::Error>;
}

/// 生成には `FirebaseFactory` を使用する
pub struct Firebase {
    api_key: Arc<String>,
}

impl FirebaseInfra for Firebase {
    fn query_user_by_access_token(
        &self,
        access_token_id: &str,
    ) -> Result<Option<String>, failure::Error> {
        #[derive(Serialize)]
        struct ReqData<'a> {
            #[serde(rename = "idToken")]
            id_token: &'a str,
        }

        let url = format!("{}/getAccountInfo?key={}", API_BASE, self.api_key);
        log::debug!("Request to firebase : {}", url);

        let client = reqwest::Client::new();
        let mut res = client
            .post(&url)
            .json(&ReqData {
                id_token: access_token_id,
            })
            .send()?;

        log::debug!("Get response from firebase : {:?}", res);

        if !res.status().is_success() {
            return Ok(None);
        }

        Ok(Some(res.json::<ResData>()?.users.pop().unwrap().local_id))
    }
}

#[derive(Deserialize)]
struct ResData {
    users: Vec<ResUser>,
}

#[derive(Deserialize)]
struct ResUser {
    #[serde(rename = "localId")]
    local_id: String,
}

#[derive(Debug, Clone)]
pub struct FirebaseFactory {
    api_key: Arc<String>,
}

impl FirebaseFactory {
    pub fn new(api_key: String) -> FirebaseFactory {
        FirebaseFactory {
            api_key: Arc::new(api_key),
        }
    }
}

impl InfraFactory<Firebase> for FirebaseFactory {
    /// never error
    fn create(&self) -> Result<Firebase, failure::Error> {
        Ok(Firebase {
            api_key: self.api_key.clone(),
        })
    }
}
