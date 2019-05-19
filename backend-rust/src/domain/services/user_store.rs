use crate::domain::{
    models::user::{User, UserId},
    services::Store,
};

pub trait UserStore: Store {
    fn query_user(&mut self, user_id: &UserId) -> Result<Option<User>, Self::Error>;

    fn query_user_by_email_and_hashed_pass(
        &mut self,
        email: &str,
        hashed_pass: &str,
    ) -> Result<Option<User>, Self::Error>;

    /// 現在存在する全てのUserのUserIdを取得する
    fn query_all_user_ids(&mut self) -> Result<Vec<UserId>, Self::Error>;
}
