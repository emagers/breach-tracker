use std::error::Error;
use crate::dto::Breach;

pub mod wa_parser;
pub mod or_parser;
pub mod ca_parser;

pub trait Parser {
	fn parse_page(&self, page: &str) -> Result<Vec<Breach>, Box<dyn Error>>;
}
