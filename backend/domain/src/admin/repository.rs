use super::{
    model::{Admin, AdminId},
    service::auth::verify_credentials,
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait AdminRepository {
    async fn query_unauthenticated(
        &mut self,
        email: &str,
    ) -> anyhow::Result<Option<Unauthenticated>>;
}

pub struct Unauthenticated {
    pub id: Uuid,
    pub cred: Vec<u8>,
    pub salt: Vec<u8>,
}

impl Unauthenticated {
    pub fn authenticate(&self, pass: &str) -> anyhow::Result<Admin> {
        verify_credentials(self.salt.as_slice(), self.cred.as_slice(), pass)?;

        Ok(Admin {
            id: AdminId(self.id),
        })
    }
}
