use super::AccessToken;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Admin {
    pub id: AdminId,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct AdminId(pub Uuid);

impl From<AccessToken> for Admin {
    fn from(token: AccessToken) -> Self {
        Admin { id: token.admin_id }
    }
}
