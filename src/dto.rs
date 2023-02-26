use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum State {
	WA = 1,
	OR = 2,
	CA = 3,
}

impl From<crate::datamodels::State> for State {
	fn from(value: crate::datamodels::State) -> Self {
		match value {
			crate::datamodels::State::WA => State::WA,
			crate::datamodels::State::OR => State::OR,
			crate::datamodels::State::CA => State::CA,
		}
	}
}

impl From<State> for crate::datamodels::State {
	fn from(value: State) -> Self {
		match value {
			State::WA => crate::datamodels::State::WA,
			State::OR => crate::datamodels::State::OR,
			State::CA => crate::datamodels::State::CA,
		}
	}
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Sensitivity {
	Unknown = 0,
	Low = 1,
	Medium = 2,
	High = 3
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassificationType {
	Unknown(String, Sensitivity),
	Name(Sensitivity),
	Username(Sensitivity),
	Password(Sensitivity),
	SecurityQuestionOrAnswer(Sensitivity),
	Email(Sensitivity),
	StudentId(Sensitivity),
	DateOfBirth(Sensitivity),
	SocialSecurity(Sensitivity),
	DriversLicense(Sensitivity),
	StateId(Sensitivity),
	BankingInformation(Sensitivity),
	FinancialInformation(Sensitivity),
	PassportNumber(Sensitivity),
	HealthInsurancePolicy(Sensitivity),
	MedicalInformation(Sensitivity),
	BiometricData(Sensitivity)
}

impl From<(&crate::datamodels::ClassificationType, &str)> for ClassificationType {
	fn from(value: (&crate::datamodels::ClassificationType, &str)) -> Self {
		match value.0 {
			crate::datamodels::ClassificationType::Unknown => ClassificationType::Unknown(value.1.to_string(), Sensitivity::Unknown),
			crate::datamodels::ClassificationType::Name => ClassificationType::Name(Sensitivity::Low),
			crate::datamodels::ClassificationType::Username => ClassificationType::Username(Sensitivity::Low),
			crate::datamodels::ClassificationType::Password => ClassificationType::Password(Sensitivity::High),
			crate::datamodels::ClassificationType::SecurityQuestionOrAnswer => ClassificationType::SecurityQuestionOrAnswer(Sensitivity::High),
			crate::datamodels::ClassificationType::Email => ClassificationType::Email(Sensitivity::Medium),
			crate::datamodels::ClassificationType::StudentId => ClassificationType::StudentId(Sensitivity::Medium),
			crate::datamodels::ClassificationType::DateOfBirth => ClassificationType::DateOfBirth(Sensitivity::Medium),
			crate::datamodels::ClassificationType::SocialSecurity => ClassificationType::SocialSecurity(Sensitivity::High),
			crate::datamodels::ClassificationType::DriversLicense => ClassificationType::DriversLicense(Sensitivity::Medium),
			crate::datamodels::ClassificationType::StateId => ClassificationType::StateId(Sensitivity::Medium),
			crate::datamodels::ClassificationType::BankingInformation => ClassificationType::BankingInformation(Sensitivity::High),
			crate::datamodels::ClassificationType::FinancialInformation => ClassificationType::FinancialInformation(Sensitivity::Medium),
			crate::datamodels::ClassificationType::PassportNumber => ClassificationType::PassportNumber(Sensitivity::Medium),
			crate::datamodels::ClassificationType::HealthInsurancePolicy => ClassificationType::HealthInsurancePolicy(Sensitivity::Medium),
			crate::datamodels::ClassificationType::MedicalInformation => ClassificationType::MedicalInformation(Sensitivity::Medium),
			crate::datamodels::ClassificationType::BiometricData => ClassificationType::BiometricData(Sensitivity::High),
		}
	}
}

impl From<&ClassificationType> for crate::datamodels::ClassificationType {
	fn from(value: &ClassificationType) -> Self {
		match value {
			ClassificationType::Unknown(_,_) => crate::datamodels::ClassificationType::Unknown,
			ClassificationType::Name(_) => crate::datamodels::ClassificationType::Name,
			ClassificationType::Username(_) => crate::datamodels::ClassificationType::Username,
			ClassificationType::Password(_) => crate::datamodels::ClassificationType::Password,
			ClassificationType::SecurityQuestionOrAnswer(_) => crate::datamodels::ClassificationType::SecurityQuestionOrAnswer,
			ClassificationType::Email(_) => crate::datamodels::ClassificationType::Email,
			ClassificationType::StudentId(_) => crate::datamodels::ClassificationType::StudentId,
			ClassificationType::DateOfBirth(_) => crate::datamodels::ClassificationType::DateOfBirth,
			ClassificationType::SocialSecurity(_) => crate::datamodels::ClassificationType::SocialSecurity,
			ClassificationType::DriversLicense(_) => crate::datamodels::ClassificationType::DriversLicense,
			ClassificationType::StateId(_) => crate::datamodels::ClassificationType::StateId,
			ClassificationType::BankingInformation(_) => crate::datamodels::ClassificationType::BankingInformation,
			ClassificationType::FinancialInformation(_) => crate::datamodels::ClassificationType::FinancialInformation,
			ClassificationType::PassportNumber(_) => crate::datamodels::ClassificationType::PassportNumber,
			ClassificationType::HealthInsurancePolicy(_) => crate::datamodels::ClassificationType::HealthInsurancePolicy,
			ClassificationType::MedicalInformation(_) => crate::datamodels::ClassificationType::MedicalInformation,
			ClassificationType::BiometricData(_) => crate::datamodels::ClassificationType::BiometricData,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Breach {
	pub id: i32,
	pub date_reported: NaiveDateTime,
	pub organization_name: String,
	pub date_of_breach: NaiveDateTime,
	pub affected_count: i32,
	pub loc: State,
	pub link: Option<String>,
	pub leaked_info: Vec<ClassificationType>
}

impl From<(&crate::datamodels::BreachData, Vec<&crate::datamodels::Classification>)> for Breach {
	fn from(value: (&crate::datamodels::BreachData, Vec<&crate::datamodels::Classification>)) -> Self {
		Breach {
			id: value.0.id,
			date_reported: value.0.date_reported,
			organization_name: value.0.organization_name.clone(),
			date_of_breach: value.0.date_of_breach,
			affected_count: value.0.affected_count,
			loc: value.0.loc.into(),
			link: value.0.link.clone(),
			leaked_info: value.1.iter().map(|r|
				(&r.classification_type, r.content.as_str()).into()
				).collect()
		}
	}
}
