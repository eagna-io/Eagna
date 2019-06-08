use arrayvec::ArrayString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
/// Firebase が発行するuidは現在28文字。
/// しかし将来的に増える可能性がある（Firebaseはuidの長さについて言及していない）ので、
/// 48文字まで対応できるようにしている。
/// もしFirebaseのuidが36文字以上になってきたら、48以上にすることを検討すべき
pub struct UserId(ArrayString<[u8; 48]>);

impl UserId {
    pub fn from_str(s: &str) -> UserId {
        UserId(ArrayString::from(s).unwrap())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
