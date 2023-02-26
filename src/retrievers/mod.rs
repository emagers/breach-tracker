pub mod wa;
pub mod or;
pub mod ca;

use crate::dto::Breach;
use std::{error::Error, collections::HashMap};
use chrono::{NaiveDateTime};
use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use async_trait::async_trait;

#[async_trait]
pub trait Retriever {
	async fn retrieve(&self, client: &Client, options: &RetrieverOptions) -> Result<Vec<Breach>, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub struct RetrieverOptions {
	pub collect_until: NaiveDateTime
}

async fn invoke(client: &Client, url: &str, headers: HeaderMap<HeaderValue>) -> Result<String, Box<dyn Error>> {
	let body = client.get(url)
		.headers(headers)
		.send()
		.await?
		.text()
		.await?;

	Ok(body)
}

