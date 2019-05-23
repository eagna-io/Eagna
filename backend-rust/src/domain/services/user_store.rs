use crate::domain::{
    models::user::{User, UserId},
    services::Store,
};

pub trait UserStore: Store {
    fn query_user(&mut self, user_id: &UserId) -> Result<Option<User>, Self::Error>;

    fn save_user<'a>(&mut self, new_user: NewUser<'a>) -> Result<(), Self::Error>;

    /// 現在存在する全てのUserのUserIdを取得する
    fn query_all_user_ids(&mut self) -> Result<Vec<UserId>, Self::Error>;
}

pub struct NewUser<'a> {
    pub id: &'a UserId,
    pub name: &'a str,
    pub email: &'a str,
}
