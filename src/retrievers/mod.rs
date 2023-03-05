pub mod single_page;
pub mod multi_page;

use crate::{dto::Breach, parsers::Parser};
use std::error::Error;
use chrono::NaiveDateTime;
use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use async_trait::async_trait;

#[async_trait]
pub trait Retriever {
	async fn retrieve(&self, client: &Client, parser: Box<dyn Parser + Send>, options: &RetrieverOptions, page_incrementer: Box<dyn Fn(i32) -> i32 + Send>, url_generator: Box<dyn Fn(String, String) -> String + Send>) -> Result<Vec<Breach>, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub enum WebRequestType {
	Post,
	Get,
}

#[derive(Debug)]
pub struct RetrieverOptions {
	pub collect_until: NaiveDateTime,
	pub base_url: String,
	pub headers: HeaderMap<HeaderValue>,
	pub state: crate::dto::State,
	pub request_type: WebRequestType,
}

async fn invoke(client: &Client, url: &str, headers: &HeaderMap<HeaderValue>, request_type: &WebRequestType) -> Result<String, Box<dyn Error>> {
	match request_type {
		WebRequestType::Post => invoke_post(client, url, headers).await,
		WebRequestType::Get => invoke_get(client, url, headers).await
	}
}

async fn invoke_get(client: &Client, url: &str, headers: &HeaderMap<HeaderValue>) -> Result<String, Box<dyn Error>> {
	let body = client.get(url)
		.headers(headers.clone())
		.send()
		.await?
		.text()
		.await?;

	Ok(body)
}

async fn invoke_post(client: &Client, url: &str, headers: &HeaderMap<HeaderValue>) -> Result<String, Box<dyn Error>> {
	let body = client.post(url)
		.headers(headers.clone())
		.send()
		.await?
		.text()
		.await?;

	Ok(body)
}

