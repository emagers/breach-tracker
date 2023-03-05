use reqwest::Client;
use super::{Retriever, RetrieverOptions, invoke};
use crate::{dto::{Breach}, parsers::{Parser}};
use async_trait::async_trait;

pub struct MultiPage {}

#[async_trait]
impl Retriever for MultiPage {
	async fn retrieve(&self, client: &Client, parser: Box<dyn Parser + Send>, options: &RetrieverOptions, page_incrementer: Box<dyn Fn(i32) -> i32 + Send>, url_generator: Box<dyn Fn(String, String) -> String + Send>) -> Result<Vec<Breach>, Box<dyn std::error::Error>> {
		let mut page = 0;
		let mut next_url_part: Option<String> = None;
		let mut continue_processing = true;

		let mut breaches = vec!();

		while continue_processing {
			let next_url = match &next_url_part {
				Some(part) => {
					url_generator(options.base_url.clone(), part.clone())
				},
				None => url_generator(options.base_url.clone(), page.to_string())
			};

			let text = invoke(client, &next_url, &options.headers, &options.request_type).await?;

			let (mut brs, nup) = parser.parse_page(&text)?;

			continue_processing = brs.len() > 0 && brs.last().unwrap().date_reported > options.collect_until;

			if breaches.len() > 0 && brs.len() > 0 {
				let last_inserted: &Breach = breaches.last().unwrap();
				let last_parsed = brs.last().unwrap();
				if last_inserted.organization_name == last_parsed.organization_name && last_inserted.date_reported == last_parsed.date_reported && last_inserted.link == last_parsed.link {
					break;
				}
			}

			breaches.append(&mut brs);

			page = page_incrementer(page);

			if let Some(part) = &nup {
				next_url_part = Some(part.clone());
			}

			if nup.is_none() && next_url_part.is_some() {
				break;
			}
		}

		Ok(breaches)
	}
}