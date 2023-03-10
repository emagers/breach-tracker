use chrono::NaiveDateTime;
use crate::dto::{Breach, State, BreachType};
use super::Parser;

pub struct CaParser { }

impl CaParser {
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
					let mut row_it = rows.split("<tr ");
					_ = row_it.next();

					while let Some(row) = row_it.next() {
						let mut breach = CaParser::parse_breach(row)?;
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
			_ = field_it.next();

			// this is the point of the link
			{
				let a = field_it.next().unwrap();
				let mut link_href_it = a.split("href=\"");
				_ = link_href_it.next();

				let link_cont = link_href_it.next().unwrap();
				let mut link_cont_it = link_cont.split("\"");
				link = Some(link_cont_it.next().unwrap().to_string());
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
				if date_str.contains("n/a") {
					date_of_breaches.push(None);
				}
				else {
					let mut content_it = date_str.split("content=\"");
					let _ = content_it.next();

					while let Some(content) = content_it.next() {
						let mut date_it = content.split("\"");
						let date = date_it.next().unwrap();
						date_of_breaches.push(Some(NaiveDateTime::parse_from_str(&date, "%+").unwrap()));
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
			return Err(format!("CA parsing failure {}", text).into());
		}

		Ok(date_of_breaches.iter().map(|dob| Breach {
			id: 0,
			date_reported: date_reported.unwrap(),
			date_of_breach: dob.clone(),
			organization_name: organization_name.unwrap().to_string(),
			affected_count: None,
			affected_count_local: None,
			loc: State::CA,
			link: link.clone(),
			breach_type: BreachType::Unknown,
			leaked_info: vec!()
		}).collect())
	}
}

impl Parser for CaParser {
	fn parse_page(&self, page: &str) -> Result<(Vec<Breach>, Option<String>), Box<dyn std::error::Error>> {
		CaParser::parse_body(page)
	}
}