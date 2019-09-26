use serde::de::{Deserialize, Deserializer, Error, Unexpected};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonZeroU32(#[serde(deserialize_with = "error_when_zero")] u32);

impl NonZeroU32 {
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn from_u32(n: u32) -> Result<NonZeroU32, ZeroU32Error> {
        if n == 0 {
            Err(ZeroU32Error())
        } else {
            Ok(NonZeroU32(n))
        }
    }
}

fn error_when_zero<'de, D>(de: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let n = u32::deserialize(de)?;
    if n == 0 {
        Err(D::Error::invalid_value(
            Unexpected::Unsigned(n as u64),
            &"non-zero unsigned number",
        ))
    } else {
        Ok(n)
    }
}

pub struct ZeroU32Error();

impl std::fmt::Debug for ZeroU32Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "ZeroU32Error")
    }
}

impl std::fmt::Display for ZeroU32Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "ZeroU32Error")
    }
}

impl std::error::Error for ZeroU32Error {}
