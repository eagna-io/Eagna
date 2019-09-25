#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate failure;

pub mod app;
pub mod domain;
pub mod infra;
pub mod primitive;
