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
					affected_count_local: match record.no_x0020_of_x0020_md_x0020_residents.trim().replace(",", "").parse::<i32>() {
						Ok(c) => Some(c),
						_ => None
					},
					affected_count: None,
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
				affected_count_local: match record.no_x0020_of_x0020_md_x0020_residents.trim().replace(",", "").parse::<i32>() {
					Ok(c) => Some(c),
					_ => None
				},
				affected_count: None,
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
		let cleaned = text.replace("and/or", ",").replace("and", ",").replace(".", "").replace("&quot;", "").replace("\"", "");
		let mut classifications = vec!();
		let mut class_it = cleaned.split(",");

		while let Some(class) = class_it.next() {
			let trimmed = class.trim().to_lowercase();
			let mut added = false;

			if trimmed.contains("name") {
				classifications.push(ClassificationType::Name(Sensitivity::Low));
				added = true;
			}
			else if trimmed.contains("password") {
				classifications.push(ClassificationType::Password(Sensitivity::High));
				added = true;
			}
			else if trimmed.contains("username") || trimmed.contains("user id") {
				classifications.push(ClassificationType::Username(Sensitivity::Medium));
				added = true;
			}
			else if (trimmed.contains("date") && trimmed.contains("birth")) ||
				trimmed.contains("dob")  {
				classifications.push(ClassificationType::DateOfBirth(Sensitivity::Medium));
				added = true;
			}
			else if trimmed.contains("social security") ||
				trimmed.contains("ssn") ||
				trimmed == "ss #" {
				classifications.push(ClassificationType::SocialSecurity(Sensitivity::High));
				added = true;
			}
			else if trimmed.contains("address") && !trimmed.contains("email") || trimmed.contains("zip code") {
				classifications.push(ClassificationType::Address(Sensitivity::Medium));
				added = true;
			}

			if trimmed.contains("student") && trimmed.contains("id") {
				classifications.push(ClassificationType::StudentId(Sensitivity::Low));
				added = true;
			}

			if trimmed.contains("personal information") || trimmed.contains("race") || trimmed.contains("age") || trimmed.contains("gender") || trimmed.contains("education") || trimmed.contains("marital") || trimmed.contains("demographic") || trimmed.contains("nationality") {
				classifications.push(ClassificationType::DemographicInformation(Sensitivity::Low));
				added = true;
			}

			if trimmed.contains("voter") {
				classifications.push(ClassificationType::VoterRegistrationNumber(Sensitivity::Low));
				added = true;
			}

			if trimmed.contains("employee") || trimmed.contains("employment") || trimmed.contains("employer") || trimmed.contains("work") || trimmed.contains("human resource") {
				classifications.push(ClassificationType::EmploymentInformation(Sensitivity::Low));
				added = true;
			}

			if trimmed.contains("security") && (trimmed.contains("question") || trimmed.contains("code") || trimmed.contains("word")) {
				classifications.push(ClassificationType::SecurityQuestionOrAnswer(Sensitivity::High));
				added = true;
			}

			if trimmed.contains("passport") {
				classifications.push(ClassificationType::PassportNumber(Sensitivity::High));
				added = true;
			}

			if trimmed.contains("driver") && trimmed.contains("license") {
				classifications.push(ClassificationType::DriversLicense(Sensitivity::High));
				added = true;
			}

			if (trimmed.contains("state") || trimmed.contains("government") || trimmed.contains("federal")) && trimmed.contains("id") {
				classifications.push(ClassificationType::StateId(Sensitivity::High));
				added = true;
			}

			if trimmed.contains("email") {
				classifications.push(ClassificationType::Email(Sensitivity::Low));
				added = true;
			}

			if trimmed.contains("phone number") || trimmed.contains("tele #") || trimmed.contains("tel num") {
				classifications.push(ClassificationType::PhoneNumber(Sensitivity::Low));
				added = true;
			}

			if trimmed.contains("w-9") || trimmed.contains("pay") || trimmed.contains("401k") || trimmed.contains("dependent") || trimmed.contains("beneficiar") || trimmed.contains("withholding") || trimmed.contains("1095") || trimmed.contains("1098") || trimmed.contains("tin") || trimmed.contains("w4") || trimmed.contains("billed") || trimmed.contains("billing") || trimmed.contains("income") || trimmed.contains("payroll") || trimmed.contains("w2") || trimmed.contains("w-2") || trimmed.contains("1099") || trimmed.contains("compensation") || trimmed.contains("charges") || trimmed.contains("donation") || trimmed.contains("purchase") || trimmed.contains("transaction") || trimmed.contains("financial") || trimmed.contains("payer information") || trimmed.contains("loan") || trimmed.contains("tax") {
				classifications.push(ClassificationType::FinancialInformation(Sensitivity::High));
				added = true;
			}

			if trimmed.contains("pin") || trimmed.contains("routing") || trimmed.contains("direct deposit") || trimmed.contains("bank") || trimmed.contains("debit") || trimmed.contains("payment card") || trimmed.contains("credit") || trimmed.contains("cvv") || trimmed.contains("benefit") || trimmed.contains("wage") || trimmed.contains("card number") {
				classifications.push(ClassificationType::BankingInformation(Sensitivity::High));
				added = true;
			}

			if trimmed.contains("immunization") || trimmed.contains("medication") || trimmed.contains("drugs") || trimmed.contains("provider") || trimmed.contains("procedure") || trimmed.contains("illness") || trimmed.contains("injury") || trimmed.contains("hipaa") || trimmed.contains("vaccination") || trimmed.contains("prescription") || trimmed.contains("dentist") || trimmed.contains("diagnosis") || trimmed.contains("physician") || trimmed.contains("medical") || trimmed.contains("treatment") || trimmed.contains("health info") || trimmed.contains("patient") || trimmed.contains("clinic") || trimmed.contains("visit") || trimmed.contains("dental") || trimmed.contains("vision") {
				classifications.push(ClassificationType::MedicalInformation(Sensitivity::High));
				added = true;
			}

			if trimmed.contains("deductible") || trimmed.contains("group plan") || trimmed.contains("health insurance") || trimmed.contains("medicare") || trimmed.contains("medicade") || trimmed.contains("medicaid") || trimmed.contains("insurance") || trimmed.contains("health plan") {
				classifications.push(ClassificationType::HealthInsurancePolicy(Sensitivity::High));
				added = true;
			}

			if !added {
				classifications.push(ClassificationType::Unknown(trimmed, Sensitivity::Unknown));
			}
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
