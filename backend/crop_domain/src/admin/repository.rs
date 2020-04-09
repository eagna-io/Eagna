use super::{
    model::{Admin, AdminId},
    service::auth::verify_credentials,
};
use crop_infra::pg::{admin::AdminTable as _, Connection};
use uuid::Uuid;

pub fn query_unauthenticated(
    conn: &Connection,
    email: &str,
) -> anyhow::Result<Option<Unauthenticated>> {
    if let Some(admin) = conn.query_credentials_by_email(email)? {
        Ok(Some(Unauthenticated {
            id: admin.id,
            cred: admin.cred,
            salt: admin.salt,
        }))
    } else {
        Ok(None)
    }
}

/// まだauthenticateされてないので、
/// これはAdminを表すモデルではない
pub struct Unauthenticated {
    id: Uuid,
    cred: Vec<u8>,
    salt: Vec<u8>,
}

impl Unauthenticated {
    pub fn authenticate(&self, pass: &str) -> anyhow::Result<AuthenticatedAdmin> {
        verify_credentials(self.salt.as_slice(), self.cred.as_slice(), pass)?;

        Ok(AuthenticatedAdmin {
            id: AdminId(self.id),
        })
    }
}

pub struct AuthenticatedAdmin {
    pub id: AdminId,
}

impl Admin for AuthenticatedAdmin {
    fn id(&self) -> &AdminId {
        &self.id
    }
}
