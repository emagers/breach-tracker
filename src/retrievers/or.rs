use chrono::{NaiveDate, NaiveDateTime};
use reqwest::{Client, header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT, HOST}};
use super::{Retriever, RetrieverOptions, invoke};
use crate::{dto::{Breach, State}};
use async_trait::async_trait;

const BASE_URL: &str = "https://justice.oregon.gov/consumer/databreach/";

pub struct OrRetriever {

}

impl OrRetriever {
	fn parse_page(&self, text: &str) -> Result<Vec<Breach>, Box<dyn std::error::Error>> {
		let mut content_it = text.split("<tbody>");
		_ = content_it.next();
		let content = content_it.next();

		let mut breaches = vec!();
		match content {
			None => return Ok(breaches),
			Some(rows_content) => {
				let mut rows_it = rows_content.split("</tbody>");
				if let Some(rows) = rows_it.next() {
					let mut row_it = rows.split("<tr>");
					_ = row_it.next();

					while let Some(row) = row_it.next() {
						let mut breach = self.parse_breach(row)?;
						breaches.append(&mut breach);
					}
				}
			}
		}

		Ok(breaches)
	}

	fn parse_breach(&self, text: &str) -> Result<Vec<Breach>, Box<dyn std::error::Error>> {
		let mut row_it = text.split("</td>");

		let mut date_of_breaches = vec!();
		let mut date_reported = None;
		let mut organization_name = None;

		if let Some(field) = row_it.next() {
			let mut field_it = field.split(">");
			_ = field_it.next();
			_ = field_it.next();

			let org = field_it.next().unwrap();
			let mut org_it = org.split("<");
			organization_name = Some(org_it.next().unwrap().trim());
		}

		if let Some(field) = row_it.next() {
			let mut field_it = field.split(">");
			_ = field_it.next();

			let date_str = field_it.next();
			if let Some(date_str) = date_str {
				let mut date_str_it = date_str.split("<");
				let b = date_str_it.next().unwrap().trim();

				if b == "" {
					date_of_breaches.push(NaiveDate::from_ymd_opt(-4, 2, 29).unwrap().and_hms_opt(0, 0, 0).unwrap());
				}
				else {
					if b.contains(",") {
						let mut b_it = b.split(",");

						while let Some(one_date) = b_it.next() {
							if one_date.len() >= 10 {
								let date = one_date.trim().chars().take(10).collect::<String>();
								date_of_breaches.push(NaiveDateTime::parse_from_str(&(date + " 00:00:00 +00:00"), "%m/%d/%Y %H:%M:%S %z").unwrap());
							}
							else if one_date.len() < 10 {
								date_of_breaches.push(NaiveDate::from_ymd_opt(-4, 2, 29).unwrap().and_hms_opt(0, 0, 0).unwrap());
							}
						}
					}
					else {
						let date = b.chars().take(10).collect::<String>();
						date_of_breaches.push(NaiveDateTime::parse_from_str(&(date + " 00:00:00 +00:00"), "%m/%d/%Y %H:%M:%S %z").unwrap());
					}
				}
			}
			else {
				date_of_breaches.push(NaiveDate::from_ymd_opt(-4, 2, 29).unwrap().and_hms_opt(0, 0, 0).unwrap());
			}
		}

		if let Some(field) = row_it.next() {
			let mut field_it = field.split(">");
			_ = field_it.next();

			let date_str = field_it.next().unwrap();
			let mut date_str_it = date_str.split("<");
			let b = date_str_it.next().unwrap().trim();
			date_reported = Some(NaiveDateTime::parse_from_str(&(b.to_string() + " 00:00:00 +00:00"), "%m/%d/%Y %H:%M:%S %z").unwrap());
		}

		if date_reported.is_none() || date_of_breaches.len() == 0 || organization_name.is_none() {
			return Err(format!("OR parsing failure {}", text).into());
		}

		Ok(date_of_breaches.iter().map(|dob| Breach {
			id: 0,
			date_reported: date_reported.unwrap(),
			date_of_breach: dob.clone(),
			organization_name: organization_name.unwrap().to_string(),
			affected_count: 0,
			loc: State::OR,
			leaked_info: vec!()
		}).collect())
	}

	fn create_headers() -> HeaderMap<HeaderValue> {
		let mut headers = HeaderMap::new();

		headers.insert(ACCEPT, "*/*".parse().unwrap());
		headers.insert(USER_AGENT, "breach_tracker".parse().unwrap());
		headers.insert(HOST, "justice.oregon.gov".parse().unwrap());

		headers
	}
}

#[async_trait]
impl Retriever for OrRetriever {
	async fn retrieve(&self, client: &Client, _options: &RetrieverOptions) -> Result<Vec<Breach>, Box<dyn std::error::Error>> {
		let mut breaches = vec!();

		let text = invoke(client, &format!("{}", BASE_URL), OrRetriever::create_headers()).await?;

		let mut brs = self.parse_page(&text)?;

		breaches.append(&mut brs);

		Ok(breaches)
	}
}