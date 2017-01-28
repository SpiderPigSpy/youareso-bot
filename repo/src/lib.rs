#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
extern crate rand;

mod schema;
mod diesel_ext;

pub mod dao;
pub mod user;
pub mod subject;
pub mod adjective;
pub mod joke;
