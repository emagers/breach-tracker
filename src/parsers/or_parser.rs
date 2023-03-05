use chrono::NaiveDateTime;
use crate::dto::{Breach, State, BreachType};
use super::Parser;

pub struct OrParser { }

impl OrParser {
	fn parse_body(text: &str) -> Result<(Vec<Breach>, Option<String>), Box<dyn std::error::Error>> {
		let mut content_it = text.split("<tbody>");
		_ = content_it.next();
		let content = content_it.next();

		let mut breaches = vec!();
		match content {
			None => return Ok((breaches, None)),
			Some(rows_content) => {
				let mut rows_it = rows_content.split("</tbody>");
				if let Some(rows) = rows_it.next() {
					let mut row_it = rows.split("<tr>");
					_ = row_it.next();

					while let Some(row) = row_it.next() {
						let mut breach = OrParser::parse_breach(row)?;
						breaches.append(&mut breach);
					}
				}
			}
		}

		Ok((breaches, None))
	}

	fn parse_breach(text: &str) -> Result<Vec<Breach>, Box<dyn std::error::Error>> {
		let mut row_it = text.split("</td>");

		let mut date_of_breaches: Vec<Option<NaiveDateTime>> = vec!();
		let mut date_reported = None;
		let mut organization_name = None;
		let mut link: Option<String> = None;

		if let Some(field) = row_it.next() {
			let mut field_it = field.split(">");
			_ = field_it.next();

			// link starts here
			{
				let a = field_it.next().unwrap();
				let mut href_it = a.split("href=\"");
				let _ = href_it.next();

				let link_raw = href_it.next().unwrap();
				let mut link_raw_it = link_raw.split("\"");
				let l = link_raw_it.next().unwrap().to_string();

				link = Some(format!("{}{}", "https://justice.oregon.gov", l));
			}

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
					date_of_breaches.push(None);
				}
				else {
					if b.contains(",") {
						let mut b_it = b.split(",");

						while let Some(one_date) = b_it.next() {
							if one_date.len() >= 10 {
								let date = one_date.trim().chars().take(10).collect::<String>();
								date_of_breaches.push(Some(NaiveDateTime::parse_from_str(&(date + " 00:00:00 +00:00"), "%m/%d/%Y %H:%M:%S %z").unwrap()));
							}
							else if one_date.len() < 10 {
								date_of_breaches.push(None);
							}
						}
					}
					else {
						let date = b.chars().take(10).collect::<String>();
						date_of_breaches.push(Some(NaiveDateTime::parse_from_str(&(date + " 00:00:00 +00:00"), "%m/%d/%Y %H:%M:%S %z").unwrap()));
					}
				}
			}
			else {
				date_of_breaches.push(None);
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
			affected_count: None,
			loc: State::OR,
			link: link.clone(),
			breach_type: BreachType::Unknown,
			leaked_info: vec!()
		}).collect())
	}
}

impl Parser for OrParser {
	fn parse_page(&self, page: &str) -> Result<(Vec<Breach>, Option<String>), Box<dyn std::error::Error>> {
		OrParser::parse_body(page)
	}
}