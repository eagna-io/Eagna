use smallvec::{Array, SmallVec};

pub struct String<A: Array>(SmallVec<A>);

impl<A> String<A>
where
    A: Array<Item = u8>,
{
    pub fn from_str(s: &str) -> Self {
        let inner = SmallVec::from_slice(s.as_bytes());
        String(inner)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.0.as_slice()).unwrap()
    }
}

pub type SmallString = String<[u8; 8]>;
pub type MidString = String<[u8; 16]>;
pub type LargeString = String<[u8; 32]>;
