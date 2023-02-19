use crate::dto::Breach;
use std::error::Error;
use chrono::{NaiveDateTime};
use reqwest::{Client};
use async_trait::async_trait;

#[async_trait]
pub trait Retriever {
	async fn retrieve(&self, client: &Client, options: &RetrieverOptions) -> Result<Vec<Breach>, Box<dyn std::error::Error>>;
}

pub struct RetrieverOptions {
	pub collect_until: NaiveDateTime
}

async fn invoke(client: &Client, url: &str) -> Result<String, Box<dyn Error>> {
	let body = client.get(url)
		.header("user-agent", "breach_tracker")
		.send()
		.await?
		.text()
		.await?;

	Ok(body)
}

pub mod wa_retriever {
	use chrono::{NaiveDate, Utc, NaiveDateTime};
	use reqwest::Client;
	use super::{Retriever, RetrieverOptions, invoke};
	use crate::{dto::{Breach, ClassificationType, State, Sensitivity}};
	use async_trait::async_trait;

	const BASE_URL: &str = "https://www.atg.wa.gov/data-breach-notifications?page=";

	pub struct WaRetriever {

	}

	impl WaRetriever {
		fn parse_page(&self, text: &str) -> Result<(Vec<Breach>, NaiveDateTime), Box<dyn std::error::Error>> {
			let mut content_it = text.split("<tbody>");
			_ = content_it.next();
			_ = content_it.next();
			let content = content_it.next();

			let mut oldest_date = Utc::now().naive_local();

			let mut breaches = vec!();
			match content {
				None => return Ok((breaches, oldest_date)),
				Some(rows_content) => {
					let mut rows_it = rows_content.split("</tbody>");
					if let Some(rows) = rows_it.next() {
						let mut row_it = rows.split("<tr ");
						_ = row_it.next();

						while let Some(row) = row_it.next() {
							let breach = self.parse_breach(row)?;
							if breach.date_reported < oldest_date {
								oldest_date = breach.date_reported;
							}

							breaches.push(breach);
						}
					}
				}
			}

			Ok((breaches, oldest_date))
		}

		fn parse_breach(&self, text: &str) -> Result<Breach, Box<dyn std::error::Error>> {
			let mut row_it = text.split("</td>");

			let mut date_reported = None;
			let mut date_of_breach = None;
			let mut organization_name = None ;
			let mut affected_count = None;
			let mut leaked_info = vec!();

			if let Some(field) = row_it.next() {
				let mut field_it = field.split(">");
				_ = field_it.next();
				_ = field_it.next();
				_ = field_it.next();

				let date_str = field_it.next().unwrap();
				let mut date_str_it = date_str.split("<");
				let b = date_str_it.next().unwrap().trim();
				date_reported = Some(NaiveDateTime::parse_from_str(&(b.to_string() + " 00:00:00 +00:00"), "%m/%d/%Y %H:%M:%S %z").unwrap());
			}

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
				_ = field_it.next();

				let date_str = field_it.next();
				if let Some(date_str) = date_str {
					let mut date_str_it = date_str.split("<");
					let b = date_str_it.next().unwrap().trim();
					date_of_breach = Some(NaiveDateTime::parse_from_str(&(b.to_string() + " 00:00:00 +00:00"), "%m/%d/%Y %H:%M:%S %z").unwrap());
				}
				else {
					date_of_breach = NaiveDate::from_ymd_opt(-4, 2, 29).unwrap().and_hms_opt(0, 0, 0);
				}
			}

			if let Some(field) = row_it.next() {
				let mut field_it = field.split(">");
				_ = field_it.next();
				if let Some(count) = field_it.next() {
					let count = count.trim();
					if count == "Unknown" || count == "" {
						affected_count = Some(0);
					}
					else {
						let parsed = count.trim().parse::<i32>();
						if let Ok(i) = parsed {
							affected_count = Some(i);
						}
						else {
							println!("{}", count);
						}
					}
				}
			}

			if let Some(field) = row_it.next() {
				let mut field_it = field.split(">");
				_ = field_it.next();
				leaked_info = self.parse_classifications(field_it.next().unwrap().trim());
			}

			if date_reported.is_none() || date_of_breach.is_none() || organization_name.is_none() || affected_count.is_none() {
				return Err(format!("WA parsing failure {}", text).into());
			}

			Ok(Breach {
				id: 0,
				date_reported: date_reported.unwrap(),
				date_of_breach: date_of_breach.unwrap(),
				organization_name: organization_name.unwrap().to_string(),
				affected_count: affected_count.unwrap(),
				loc: State::WA,
				leaked_info
			})
		}

		fn parse_classifications(&self, text: &str) -> Vec<ClassificationType> {
			let mut classifications = vec!();
			let mut classification_it = text.split(";");

			while let Some(classification) = classification_it.next() {
				let classification = classification.trim();

				match classification {
					"Name" => classifications.push(ClassificationType::Name(Sensitivity::Low)),
					"Username and Password/Security Question Answers" => {
						classifications.push(ClassificationType::Password(Sensitivity::High));
						classifications.push(ClassificationType::SecurityQuestionOrAnswer(Sensitivity::High));
					},
					"Email Address and Password/Security Question Answers" => {
						classifications.push(ClassificationType::Email(Sensitivity::High));
						classifications.push(ClassificationType::SecurityQuestionOrAnswer(Sensitivity::High));
					},
					"Student ID Number" => classifications.push(ClassificationType::StudentId(Sensitivity::Low)),
					"Social Security Number" => classifications.push(ClassificationType::SocialSecurity(Sensitivity::High)),
					"Full Date of Birth" => classifications.push(ClassificationType::DateOfBirth(Sensitivity::Medium)),
					"Driver&#039" => classifications.push(ClassificationType::DriversLicense(Sensitivity::Medium)),
					"s License or Washington ID Card Number" => classifications.push(ClassificationType::StateId(Sensitivity::Medium)),
					"Financial &amp" => classifications.push(ClassificationType::FinancialInformation(Sensitivity::Medium)),
					"Banking Information" => classifications.push(ClassificationType::BankingInformation(Sensitivity::Medium)),
					"Passport Number" => classifications.push(ClassificationType::PassportNumber(Sensitivity::Medium)),
					"Health Insurance Policy or ID Number" => {
						classifications.push(ClassificationType::HealthInsurancePolicy(Sensitivity::Medium));
						classifications.push(ClassificationType::MedicalInformation(Sensitivity::Medium));
					},
					"Biometric Data" => classifications.push(ClassificationType::BiometricData(Sensitivity::High)),
					"Medical Information" => classifications.push(ClassificationType::MedicalInformation(Sensitivity::Medium)),
					_ => classifications.push(ClassificationType::Unknown(classification.to_string(), Sensitivity::Unknown))
				}
			}

			classifications
		}
	}

	#[async_trait]
	impl Retriever for WaRetriever {
		async fn retrieve(&self, client: &Client, options: &RetrieverOptions) -> Result<Vec<Breach>, Box<dyn std::error::Error>> {
			let mut page = 0;
			let mut continue_processing = true;

			let mut breaches = vec!();

			while continue_processing {
				let text = invoke(client, &format!("{}{}", BASE_URL, page)).await?;

				let (mut brs, earliest_date) = self.parse_page(&text)?;
				continue_processing = brs.len() > 0 && earliest_date > options.collect_until;

				breaches.append(&mut brs);

				page += 1;
			}

			Ok(breaches)
		}
	}
}