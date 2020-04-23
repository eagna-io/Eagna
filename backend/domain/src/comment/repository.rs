use super::Comment;
use async_trait::async_trait;

#[async_trait]
pub trait CommentRepository {
    async fn save(&mut self, comment: Comment) -> anyhow::Result<()>;
}
