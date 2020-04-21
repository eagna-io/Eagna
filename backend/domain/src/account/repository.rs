use super::Account;
use async_trait::async_trait;

#[async_trait]
pub trait AccountRepository {
    async fn save(&mut self, account: &Account) -> anyhow::Result<()>;
}
