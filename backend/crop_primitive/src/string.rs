use serde::de::{Deserialize, Deserializer, Error, Unexpected};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Deref)]
pub struct NonEmptyString(#[serde(deserialize_with = "error_when_empty_string")] String);

impl NonEmptyString {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn from_string(s: String) -> anyhow::Result<NonEmptyString> {
        if s.is_empty() {
            Err(anyhow::anyhow!("EmptyStringError"))
        } else {
            Ok(NonEmptyString(s))
        }
    }
}

fn error_when_empty_string<'de, D>(de: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    if s.is_empty() {
        Err(D::Error::invalid_value(
            Unexpected::Str(&s),
            &"non-empty string",
        ))
    } else {
        Ok(s)
    }
}
