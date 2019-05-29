use crate::domain::models::{
    access_token::{AccessToken, AccessTokenId},
    user::UserId,
};

const API_BASE: &str = "https://www.googleapis.com/identitytoolkit/v3/relyingparty";

pub fn get_user(
    api_key: &str,
    access_token_id: &AccessTokenId,
) -> Result<Option<AccessToken>, failure::Error> {
    #[derive(Serialize)]
    struct ReqData<'a> {
        #[serde(rename = "idToken")]
        id_token: &'a str,
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

    let url = format!("{}/getAccountInfo?key={}", API_BASE, api_key);
    log::debug!("Request to firebase : {}", url);

    let client = reqwest::Client::new();
    let mut res = client
        .post(&url)
        .json(&ReqData {
            id_token: access_token_id.as_str(),
        })
        .send()?;

    log::debug!("Get response from firebase : {:?}", res);

    if !res.status().is_success() {
        return Ok(None);
    }

    let uid = &res.json::<ResData>()?.users[0].local_id;

    Ok(Some(AccessToken {
        id: access_token_id.clone(),
        user_id: UserId::from_str(uid.as_str()),
    }))
}
