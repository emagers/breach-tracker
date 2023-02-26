pub mod single_page;
pub mod multi_page;

use crate::{dto::Breach, parsers::Parser};
use std::error::Error;
use chrono::NaiveDateTime;
use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use async_trait::async_trait;

#[async_trait]
pub trait Retriever {
	async fn retrieve(&self, client: &Client, parser: Box<dyn Parser + Send>, options: &RetrieverOptions) -> Result<Vec<Breach>, Box<dyn std::error::Error>>;
	fn get_url(&self, base_url: &str, page: i32) -> String;
}

#[derive(Debug)]
pub struct RetrieverOptions {
	pub collect_until: NaiveDateTime,
	pub base_url: String,
	pub headers: HeaderMap<HeaderValue>,
}

async fn invoke(client: &Client, url: &str, headers: &HeaderMap<HeaderValue>) -> Result<String, Box<dyn Error>> {
	let body = client.get(url)
		.headers(headers.clone())
		.send()
		.await?
		.text()
		.await?;

	Ok(body)
}

