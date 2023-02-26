use reqwest::{Client};
use super::{Retriever, RetrieverOptions, invoke};
use crate::{dto::{Breach}, parsers::{Parser}};
use async_trait::async_trait;

pub struct MultiPage {}

#[async_trait]
impl Retriever for MultiPage {
	async fn retrieve(&self, client: &Client, parser: Box<dyn Parser + Send>, options: &RetrieverOptions) -> Result<Vec<Breach>, Box<dyn std::error::Error>> {
		let mut page = 0;
		let mut continue_processing = true;

		let mut breaches = vec!();

		while continue_processing {
			let text = invoke(client, &self.get_url(&options.base_url, page), &options.headers).await?;

			let mut brs = parser.parse_page(&text)?;

			let earliest_date = brs.last().unwrap().date_reported;
			continue_processing = brs.len() > 0 && earliest_date > options.collect_until;

			breaches.append(&mut brs);

			page += 1;
		}

		Ok(breaches)
	}

	fn get_url(&self, base_url: &str, page: i32) -> String {
		format!("{}{}", base_url, page)
	}
}