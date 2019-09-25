use serde::de::{Deserialize, Deserializer, Error, Unexpected};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Deref)]
pub struct NonEmptyString(#[serde(deserialize_with = "error_when_empty_string")] String);

impl NonEmptyString {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn from_str(s: String) -> Result<NonEmptyString, EmptyStringError> {
        if s.is_empty() {
            Err(EmptyStringError())
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

pub struct EmptyStringError();

impl std::fmt::Debug for EmptyStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "EmptyStringError")
    }
}

impl std::fmt::Display for EmptyStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "EmptyStringError")
    }
}

impl std::error::Error for EmptyStringError {}
