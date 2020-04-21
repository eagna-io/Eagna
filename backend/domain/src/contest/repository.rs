use super::{Contest, ContestId};
use async_trait::async_trait;

#[async_trait]
pub trait ContestRepository {
    async fn find_by_id(&mut self, id: &ContestId) -> anyhow::Result<Option<Contest>>;

    async fn save(&mut self, contest: &Contest) -> anyhow::Result<()>;
}
