use chrono::{NaiveDateTime};
use serde::{Serialize, Deserialize};
use crate::dto::{Breach, State, ClassificationType, BreachType, Sensitivity};
use super::Parser;

const FILE_BASE_URI: &str = "https://www.marylandattorneygeneral.gov/";

#[derive(Debug, Serialize, Deserialize)]
struct BreachData {
	#[serde(rename = "Case_x0020_Title")]
	pub case_x0020_title: String,
	#[serde(rename = "FileRef")]
	pub file_ref: String,
	#[serde(rename = "Date_x0020_Received")]
	pub date_x0020_received: String,
	#[serde(rename = "No_x0020_of_x0020_MD_x0020_Residents")]
	pub no_x0020_of_x0020_md_x0020_residents: String,
	#[serde(rename = "Information_x0020_Breached")]
	pub information_x0020_breached: String,
	#[serde(rename = "How_x0020_Breach_x0020_Occurred")]
	pub how_x0020_breach_x0020_occurred: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
	#[serde(rename = "Row")]
	rows: Vec<BreachData>,
	#[serde(rename = "NextHref")]
	next_url: Option<String>,
}

pub struct MdParser { }

impl MdParser {
	fn parse_body(text: &str) -> Result<(Vec<Breach>, Option<String>), Box<dyn std::error::Error>> {
		let des = serde_json::from_str::<Response>(text).unwrap();

		let mut breaches = vec!();

		for record in des.rows {
			if record.date_x0020_received == "" {
				breaches.push(Breach {
					id: 0,
					date_reported: NaiveDateTime::parse_from_str("01/01/0001 00:00:00 -00:00", "%m/%d/%Y %H:%M:%S %z").unwrap(),
					date_of_breach: None,
					organization_name: record.case_x0020_title.clone(),
					affected_count: match record.no_x0020_of_x0020_md_x0020_residents.trim().replace(",", "").parse::<i32>() {
						Ok(c) => Some(c),
						_ => None
					},
					loc: State::MD,
					breach_type: MdParser::parse_breach_type(&record.how_x0020_breach_x0020_occurred),
					link: Some(format!("{}{}", FILE_BASE_URI, record.file_ref.replace("\u{002f}", "/"))),
					leaked_info: MdParser::parse_leaked_info(&record.information_x0020_breached)
				});
				continue;
			}

			breaches.push(Breach {
				id: 0,
				date_reported: NaiveDateTime::parse_from_str(&format!("{} 00:00:00 -00:00", record.date_x0020_received.replace("\u{002f}", "/")), "%m/%d/%Y %H:%M:%S %z").unwrap(), // in mm/dd/yyyy format, but / are \u002f
				date_of_breach: None,
				organization_name: record.case_x0020_title.clone(),
				affected_count: match record.no_x0020_of_x0020_md_x0020_residents.trim().replace(",", "").parse::<i32>() {
					Ok(c) => Some(c),
					_ => None
				},
				loc: State::MD,
				breach_type: MdParser::parse_breach_type(&record.how_x0020_breach_x0020_occurred),
				link: Some(format!("{}{}", FILE_BASE_URI, record.file_ref.replace("\u{002f}", "/"))),
				leaked_info: MdParser::parse_leaked_info(&record.information_x0020_breached)
			})
		}

		if let Some(next_raw) = des.next_url {
			let mut next_url = "&".to_string();
			let mut url_it = next_raw.split("breachnotices.aspx?");
			_ = url_it.next();

			let to_add = url_it.next().unwrap();
			let mut to_add_it = to_add.split("View=");

			next_url.push_str(to_add_it.next().unwrap());

			return Ok((breaches, Some(next_url)))
		}

		return Ok((breaches, None))

	}

	fn parse_leaked_info(text: &str) -> Vec<ClassificationType> {
		let cleaned = text.replace("and/or", ",").replace("and", ",").replace(".", "").replace("&quot;", "");
		let mut classifications = vec!();
		let mut class_it = cleaned.split(",");

		while let Some(class) = class_it.next() {
			let trimmed = class.trim().to_lowercase();
			match trimmed.as_str() {
				"name" => classifications.push(ClassificationType::Name(Sensitivity::Low)),
				"full name" => classifications.push(ClassificationType::Name(Sensitivity::Low)),
				"last names" => classifications.push(ClassificationType::Name(Sensitivity::Low)),
				"first" => classifications.push(ClassificationType::Name(Sensitivity::Low)),
				"names" => classifications.push(ClassificationType::Name(Sensitivity::Low)),
				"password" => classifications.push(ClassificationType::Password(Sensitivity::High)),
				"username or email address" => classifications.push(ClassificationType::Username(Sensitivity::Medium)),
				"date of birth" => classifications.push(ClassificationType::DateOfBirth(Sensitivity::Medium)),
				"date of births" => classifications.push(ClassificationType::DateOfBirth(Sensitivity::Medium)),
				"dates of birth" => classifications.push(ClassificationType::DateOfBirth(Sensitivity::Medium)),
				"dob" => classifications.push(ClassificationType::DateOfBirth(Sensitivity::Medium)),
				"social security number" => classifications.push(ClassificationType::SocialSecurity(Sensitivity::High)),
				"social security numbers" => classifications.push(ClassificationType::SocialSecurity(Sensitivity::High)),
				"the last 4 digits of the resident&#39;s social security number" => classifications.push(ClassificationType::SocialSecurity(Sensitivity::High)),
				"ssn" => classifications.push(ClassificationType::SocialSecurity(Sensitivity::High)),
				"ssns" => classifications.push(ClassificationType::SocialSecurity(Sensitivity::High)),
				"ss #" => classifications.push(ClassificationType::SocialSecurity(Sensitivity::High)),
				"ssn&#39;s" => classifications.push(ClassificationType::SocialSecurity(Sensitivity::High)),
				"passport" => classifications.push(ClassificationType::PassportNumber(Sensitivity::High)),
				"passport number" => classifications.push(ClassificationType::PassportNumber(Sensitivity::High)),
				"passport numbers" => classifications.push(ClassificationType::PassportNumber(Sensitivity::High)),
				"driver&#39;s license" => classifications.push(ClassificationType::DriversLicense(Sensitivity::High)),
				"driver&#39;s license number" => classifications.push(ClassificationType::DriversLicense(Sensitivity::High)),
				"driver&#39;s license numbers" => classifications.push(ClassificationType::DriversLicense(Sensitivity::High)),
				"driver&#39;s license/state id number" => classifications.push(ClassificationType::DriversLicense(Sensitivity::High)),
				"driver&#39;s license or state identification card number" => classifications.push(ClassificationType::DriversLicense(Sensitivity::High)),
				"driver&#39;s license or state id number" => classifications.push(ClassificationType::DriversLicense(Sensitivity::High)),
				"credit card info" => classifications.push(ClassificationType::FinancialInformation(Sensitivity::High)),
				"payment card info" => classifications.push(ClassificationType::FinancialInformation(Sensitivity::High)),
				"financial account info" => classifications.push(ClassificationType::FinancialInformation(Sensitivity::High)),
				"financial account information" => classifications.push(ClassificationType::FinancialInformation(Sensitivity::High)),
				"financial account number" => classifications.push(ClassificationType::FinancialInformation(Sensitivity::High)),
				"financial account numbers" => classifications.push(ClassificationType::FinancialInformation(Sensitivity::High)),
				"financial account numbers with password or routing number" => classifications.push(ClassificationType::BankingInformation(Sensitivity::High)),
				"bank account numbers" => classifications.push(ClassificationType::BankingInformation(Sensitivity::High)),
				"direct deposit information" => classifications.push(ClassificationType::BankingInformation(Sensitivity::High)),
				"medical information" => classifications.push(ClassificationType::MedicalInformation(Sensitivity::High)),
				"medicare/medicade identification information" => classifications.push(ClassificationType::MedicalInformation(Sensitivity::High)),
				"medical record numbers" => classifications.push(ClassificationType::MedicalInformation(Sensitivity::High)),
				"health insurance information" => classifications.push(ClassificationType::HealthInsurancePolicy(Sensitivity::Medium)),
				"medical or health insurance information" => {
					classifications.push(ClassificationType::MedicalInformation(Sensitivity::High));
					classifications.push(ClassificationType::HealthInsurancePolicy(Sensitivity::Medium));
				},
				"name and driver's license number" => {
					classifications.push(ClassificationType::Name(Sensitivity::High));
					classifications.push(ClassificationType::DriversLicense(Sensitivity::High));
				},
				"address" => classifications.push(ClassificationType::Address(Sensitivity::Medium)),
				"addresses" => classifications.push(ClassificationType::Address(Sensitivity::Medium)),
				"mailing addresses" => classifications.push(ClassificationType::Address(Sensitivity::Medium)),
				"email" => classifications.push(ClassificationType::Email(Sensitivity::Low)),
				"email address" => classifications.push(ClassificationType::Email(Sensitivity::Low)),
				"emails addresses" => classifications.push(ClassificationType::Email(Sensitivity::Low)),
				"phone number" => classifications.push(ClassificationType::PhoneNumber(Sensitivity::Medium)),
				"phone numbers" => classifications.push(ClassificationType::PhoneNumber(Sensitivity::Medium)),
				"and payer information" => classifications.push(ClassificationType::FinancialInformation(Sensitivity::Medium)),
				"" => (),
				_ => {
					classifications.push(ClassificationType::Unknown(trimmed, Sensitivity::Unknown));
				}
			};
		}

		classifications
	}

	fn parse_breach_type(text: &str) -> BreachType {
		let ltext = text.to_lowercase();
		if ltext.contains("ransomware") || text == "server encryption" {
			return BreachType::Ransomware;
		}

		if ltext.contains("unauthorized party") || ltext.contains("unauthorized network access") || ltext.contains("unauthorized access") || ltext.contains("credential stuffing") || ltext.contains("cyber-attack") || ltext.contains("unauthorized actor") || ltext.contains("data security incident") || ltext.contains("data breach incident") || ltext.contains("data privacy event") || ltext.contains("impersonation of policy holder") {
			return BreachType::HackerUnauthorizedAccess;
		}

		if ltext.contains("phishing") {
			return BreachType::Phishing;
		}

		if ltext.contains("third-party") || ltext.contains("third party") || ltext.contains("malicious code") {
			return BreachType::Malicious3rdParty
		}

		if ltext.contains("inadvertently being disclosed") {
			return BreachType::ReleaseOrDisplayOfInformation;
		}

		if ltext.contains("a former employee") {
			return BreachType::TheftByEmployeeOrContractor;
		}

		BreachType::Unknown
	}
}

impl Parser for MdParser {
	fn parse_page(&self, page: &str) -> Result<(Vec<Breach>, Option<String>), Box<dyn std::error::Error>> {
		MdParser::parse_body(page)
	}
}