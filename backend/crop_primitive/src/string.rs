use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use smallvec::{Array, SmallVec};
use std::{marker::PhantomData, string::String as StdString};

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct GenericString<A: Array<Item = u8>>(SmallVec<A>);

/// Cropサービス内で主に利用する文字列型
pub type String = GenericString<[u8; 16]>;

impl<A> GenericString<A>
where
    A: Array<Item = u8>,
{
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        let inner = SmallVec::from_slice(s.as_bytes());
        GenericString(inner)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.0.as_slice()).unwrap()
    }
}

impl<'a, A, T> From<T> for GenericString<A>
where
    A: Array<Item = u8>,
    &'a str: From<T>,
{
    fn from(t: T) -> Self {
        Self::from_str(t.into())
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

impl<A> JsonSchema for GenericString<A>
where
    A: Array<Item = u8>,
{
    fn schema_name() -> StdString {
        StdString::schema_name()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        StdString::json_schema(gen)
    }
}

impl<A> Serialize for GenericString<A>
where
    A: Array<Item = u8>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

struct StrViditor<A>(PhantomData<A>);

impl<'de, A> Visitor<'de> for StrViditor<A>
where
    A: Array<Item = u8>,
{
    type Value = GenericString<A>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("str")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GenericString::from_str(v))
    }
}

impl<'de, A> Deserialize<'de> for GenericString<A>
where
    A: Array<Item = u8>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(StrViditor::<A>(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_ser_de() {
        let s = String::from("hoge");

        assert_tokens(&s, &[Token::Str("hoge")]);
        assert_tokens(&s, &[Token::BorrowedStr("hoge")]);
        assert_tokens(&s, &[Token::String("hoge")]);
    }
}
