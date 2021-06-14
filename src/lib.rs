#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod backend;
mod error;
pub mod routes;
mod schema;
mod tokenizer;
pub mod types;

pub use backend::Backend;
pub use error::Error;
pub use tokenizer::Tokenizer;

pub type Result<T> = std::result::Result<T, Error>;
