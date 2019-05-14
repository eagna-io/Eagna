use crate::domain::models::user::{User, UserId};

pub trait UserStore {
    type Error: std::fmt::Debug;

    fn query_user(&self, user_id: &UserId) -> Result<Option<User>, Self::Error>;

    fn query_user_by_email_and_hashed_pass(
        &self,
        email: &str,
        hashed_pass: &str,
    ) -> Result<Option<User>, Self::Error>;

    /// 現在存在する全てのUserのUserIdを取得する
    fn query_all_user_ids(&self) -> Result<Vec<UserId>, Self::Error>;
}
