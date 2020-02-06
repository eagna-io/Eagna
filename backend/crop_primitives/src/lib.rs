#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_more;

pub mod string;
pub mod vec;
pub use self::string::NonEmptyString;
pub use self::vec::NonEmptyVec;
