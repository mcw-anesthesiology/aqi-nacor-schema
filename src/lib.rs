#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

pub mod schema;

#[derive(Debug)]
pub enum AQIError {
	RegexError
}
