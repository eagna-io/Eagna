#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate failure;

pub mod infra;
pub mod app;
pub mod domain;
