use serde::{Deserialize, Serialize};
use smallvec::{Array, SmallVec};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GenericString<A: Array<Item = u8>>(#[serde(with = "ser_de_with")] SmallVec<A>);

/// Cropサービス内で主に利用する文字列型
pub type String = GenericString<[u8; 16]>;

impl<A> GenericString<A>
where
    A: Array<Item = u8>,
{
    pub fn from_str(s: &str) -> Self {
        let inner = SmallVec::from_slice(s.as_bytes());
        GenericString(inner)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.0.as_slice()).unwrap()
    }
}

impl<A> std::fmt::Debug for GenericString<A>
where
    A: Array<Item = u8>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.as_str())
    }
}

impl<S, A> PartialEq<S> for GenericString<A>
where
    A: Array<Item = u8>,
    str: PartialEq<S>,
{
    fn eq(&self, other: &S) -> bool {
        self.as_str().eq(other)
    }
}

mod ser_de_with {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S, A>(vec: &SmallVec<A>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        A: Array<Item = u8>,
    {
        std::str::from_utf8(vec.as_slice())
            .unwrap()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D, A>(deserializer: D) -> Result<SmallVec<A>, D::Error>
    where
        D: Deserializer<'de>,
        A: Array<Item = u8>,
    {
        Ok(SmallVec::from_slice(
            <&str as Deserialize<'de>>::deserialize(deserializer)?.as_bytes(),
        ))
    }
}

impl<'a, A> From<&'a str> for GenericString<A>
where
    A: Array<Item = u8>,
{
    fn from(s: &'a str) -> GenericString<A> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_ser_de() {
        let s = String::from("hoge");

        assert_tokens(&s, &[Token::BorrowedStr("hoge")]);
    }
}
