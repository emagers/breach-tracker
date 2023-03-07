use chrono::NaiveDateTime;
use crate::dto::{Breach, State, BreachType};
use super::Parser;

pub struct HiParser { }

impl HiParser {
	fn parse_body(text: &str) -> Result<(Vec<Breach>, Option<String>), Box<dyn std::error::Error>> {
		let mut content_it = text.split("<tbody class=\"row-hover\">");
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
						let breach = HiParser::parse_breach(row)?;
						breaches.push(breach);
					}
				}
			}
		}

		Ok((breaches, None))
	}

	fn parse_breach(text: &str) -> Result<Breach, Box<dyn std::error::Error>> {
		let mut row_it = text.split("</td>");

		let mut date_reported = None;
		let mut organization_name = None;
		let mut link: Option<String> = None;
		let mut breach_type = BreachType::Unknown;
		let mut affected_count = None;

		/*
			Structure:
				date_reported (YYYY/mm.dd)
				case_number
				organization_name
				breach_type
				affected_count
				link
		*/

		if let Some(field) = row_it.next() {
			let mut field_it = field.split(">");
			_ = field_it.next(); // removes up to end of tr
			_ = field_it.next(); // removes up to start of td value

			let dr = field_it.next().unwrap(); // gets date_reported value include </td
			let date = dr.trim().chars().take(10).collect::<String>();

			date_reported = Some(NaiveDateTime::parse_from_str(&(date + " 00:00:00 +00:00"), "%Y/%m.%d %H:%M:%S %z").unwrap());
		}

		_ = row_it.next(); // skips case_number

		if let Some(field) = row_it.next() {
			let mut field_it = field.split(">");
			_ = field_it.next(); // removes up to start of td value

			if let Some(name) = field_it.next() {
				let mut name_it = name.split("<");
				let name = name_it.next().unwrap();

				organization_name = Some(name);
			}
		}

		if let Some(field) = row_it.next() {
			let mut field_it = field.split(">");
			_ = field_it.next();

			if let Some(bt) = field_it.next() {
				let mut bt_it = bt.split("<");
				let bt = bt_it.next().unwrap();

				breach_type = match bt {
					"Hackers/Unauthorized Access" => BreachType::HackerUnauthorizedAccess,
					"Hackers/ Unauthorized Access" => BreachType::HackerUnauthorizedAccess,
					"Stolen Laptops, Computers &amp; Equipment" => BreachType::StolenEquipment,
					"Lost in Transit" => BreachType::LostInTransit,
					"Release/Display of Information" => BreachType::ReleaseOrDisplayOfInformation,
					"Data Theft by Employee or Contractor" => BreachType::TheftByEmployeeOrContractor,
					"Phishing" => BreachType::Phishing,
					_ => BreachType::Unknown,
				}
			}
		}

		if let Some(field) = row_it.next() {
			let mut field_it = field.split(">");
			_ = field_it.next();

			if let Some(count) = field_it.next() {
				let mut count_it = count.split("<");
				let count = count_it.next().unwrap();

				affected_count = match count.trim().replace(",", "").parse::<i32>() {
					Ok(c) => Some(c),
					_ => None
				}
			}
		}

		if let Some(field) = row_it.next() {
			let mut field_it = field.split(">");
			_ = field_it.next();

			// link starts here
			let a = field_it.next().unwrap();
			let mut href_it = a.split("href=\"");
			let _ = href_it.next();

			link = match href_it.next() {
				Some(link_raw) => {
					let mut link_raw_it = link_raw.split("\"");
					let l = link_raw_it.next().unwrap().to_string();
					Some(l)
				},
				None => None
			};
		}

		if date_reported.is_none() || organization_name.is_none() {
			return Err(format!("HI parsing failure {}", text).into());
		}

		Ok(Breach {
			id: 0,
			date_reported: date_reported.unwrap(),
			date_of_breach: None,
			organization_name: organization_name.unwrap().to_string(),
			affected_count_local: affected_count,
			affected_count: None,
			loc: State::HI,
			link: link.clone(),
			breach_type,
			leaked_info: vec!()
		})
	}
}

impl Parser for HiParser {
	fn parse_page(&self, page: &str) -> Result<(Vec<Breach>, Option<String>), Box<dyn std::error::Error>> {
		HiParser::parse_body(page)
	}
}