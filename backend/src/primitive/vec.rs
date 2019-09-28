use serde::de::{Deserialize, Deserializer, Error, Unexpected};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Deref)]
pub struct NonEmptyVec<T>(
    #[serde(deserialize_with = "error_when_empty_vec")]
    #[serde(bound(deserialize = "T: for<'r> Deserialize<'r>"))]
    Vec<T>,
);

impl<T> NonEmptyVec<T> {
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.0.into_iter()
    }

    pub fn from_vec(vec: Vec<T>) -> Result<Self, EmptyVecError> {
        if vec.is_empty() {
            Err(EmptyVecError())
        } else {
            Ok(NonEmptyVec(vec))
        }
    }
}

fn error_when_empty_vec<'de, D, T>(de: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let vec = Vec::<T>::deserialize(de)?;
    if vec.is_empty() {
        Err(D::Error::invalid_value(Unexpected::Seq, &"non-empty vec"))
    } else {
        Ok(vec)
    }
}

pub struct EmptyVecError();

impl std::fmt::Debug for EmptyVecError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "EmptyVecError")
    }
}

impl std::fmt::Display for EmptyVecError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "EmptyVecError")
    }
}

impl std::error::Error for EmptyVecError {}
