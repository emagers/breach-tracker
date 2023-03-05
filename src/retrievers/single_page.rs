use reqwest::{Client};
use super::{Retriever, RetrieverOptions, invoke};
use crate::{dto::{Breach}, parsers::{Parser}};
use async_trait::async_trait;

pub struct SinglePage {}

#[async_trait]
impl Retriever for SinglePage {
	async fn retrieve(&self, client: &Client, parser: Box<dyn Parser + Send>, options: &RetrieverOptions, _: Box<dyn Fn(i32) -> i32 + Send>, url_generator: Box<dyn Fn(String, String) -> String + Send>) -> Result<Vec<Breach>, Box<dyn std::error::Error>> {
		let mut breaches = vec!();
		let next_url = url_generator(options.base_url.clone(), "".into());

		let text = invoke(client, &next_url, &options.headers, &options.request_type).await?;

		let (mut brs, _) = parser.parse_page(&text)?;

		breaches.append(&mut brs);

		Ok(breaches)
	}
}