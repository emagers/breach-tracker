use reqwest::{Client};
use super::{Retriever, RetrieverOptions, invoke};
use crate::{dto::{Breach}, parsers::{Parser}};
use async_trait::async_trait;

pub struct SinglePage {}

#[async_trait]
impl Retriever for SinglePage {
	async fn retrieve(&self, client: &Client, parser: Box<dyn Parser + Send>, options: &RetrieverOptions) -> Result<Vec<Breach>, Box<dyn std::error::Error>> {
		let mut breaches = vec!();

		let text = invoke(client, &self.get_url(&options.base_url, 0), &options.headers).await?;

		let mut brs = parser.parse_page(&text)?;

		breaches.append(&mut brs);

		Ok(breaches)
	}

	fn get_url(&self, base_url: &str, _: i32) -> String {
		format!("{}", base_url)
	}
}