use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum BreachType {
	Unknown = 0,
	HackerUnauthorizedAccess = 1,
	StolenEquipment = 2,
	Ransomware = 3,
	LostInTransit = 4,
	ReleaseOrDisplayOfInformation = 5,
	TheftByEmployeeOrContractor = 6,
	Phishing = 7,
	Malicious3rdParty = 8,
}

impl From<crate::datamodels::BreachType> for BreachType {
	fn from(value: crate::datamodels::BreachType) -> Self {
		match value {
			crate::datamodels::BreachType::Unknown => BreachType::Unknown,
			crate::datamodels::BreachType::HackerUnauthorizedAccess => BreachType::HackerUnauthorizedAccess,
			crate::datamodels::BreachType::StolenEquipment => BreachType::StolenEquipment,
			crate::datamodels::BreachType::Ransomware => BreachType::Ransomware,
			crate::datamodels::BreachType::LostInTransit => BreachType::LostInTransit,
			crate::datamodels::BreachType::ReleaseOrDisplayOfInformation => BreachType::ReleaseOrDisplayOfInformation,
			crate::datamodels::BreachType::TheftByEmployeeOrContractor => BreachType::TheftByEmployeeOrContractor,
			crate::datamodels::BreachType::Phishing => BreachType::Phishing,
			crate::datamodels::BreachType::Malicious3rdParty => BreachType::Malicious3rdParty,
		}
	}
}

impl From<BreachType> for crate::datamodels::BreachType {
	fn from(value: BreachType) -> Self {
		match value {
			BreachType::Unknown => crate::datamodels::BreachType::Unknown,
			BreachType::HackerUnauthorizedAccess => crate::datamodels::BreachType::HackerUnauthorizedAccess,
			BreachType::StolenEquipment => crate::datamodels::BreachType::StolenEquipment,
			BreachType::Ransomware => crate::datamodels::BreachType::Ransomware,
			BreachType::LostInTransit => crate::datamodels::BreachType::LostInTransit,
			BreachType::ReleaseOrDisplayOfInformation => crate::datamodels::BreachType::ReleaseOrDisplayOfInformation,
			BreachType::TheftByEmployeeOrContractor => crate::datamodels::BreachType::TheftByEmployeeOrContractor,
			BreachType::Phishing => crate::datamodels::BreachType::Phishing,
			BreachType::Malicious3rdParty => crate::datamodels::BreachType::Malicious3rdParty,
		}
	}
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum State {
	WA = 1,
	OR = 2,
	CA = 3,
	MD = 4,
	HI = 5,
}

impl From<crate::datamodels::State> for State {
	fn from(value: crate::datamodels::State) -> Self {
		match value {
			crate::datamodels::State::WA => State::WA,
			crate::datamodels::State::OR => State::OR,
			crate::datamodels::State::CA => State::CA,
			crate::datamodels::State::MD => State::MD,
			crate::datamodels::State::HI => State::HI,
		}
	}
}

impl From<State> for crate::datamodels::State {
	fn from(value: State) -> Self {
		match value {
			State::WA => crate::datamodels::State::WA,
			State::OR => crate::datamodels::State::OR,
			State::CA => crate::datamodels::State::CA,
			State::MD => crate::datamodels::State::MD,
			State::HI => crate::datamodels::State::HI,
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
	BiometricData(Sensitivity),
	PhoneNumber(Sensitivity),
	Address(Sensitivity),
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
			crate::datamodels::ClassificationType::PhoneNumber => ClassificationType::PhoneNumber(Sensitivity::Medium),
			crate::datamodels::ClassificationType::Address => ClassificationType::Address(Sensitivity::Medium),
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
			ClassificationType::PhoneNumber(_) => crate::datamodels::ClassificationType::PhoneNumber,
			ClassificationType::Address(_) => crate::datamodels::ClassificationType::Address,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Breach {
	pub id: i32,
	pub date_reported: NaiveDateTime,
	pub organization_name: String,
	pub date_of_breach: Option<NaiveDateTime>,
	pub affected_count: Option<i32>,
	pub loc: State,
	pub breach_type: BreachType,
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
			breach_type: value.0.breach_type.into(),
			link: value.0.link.clone(),
			leaked_info: value.1.iter().map(|r|
				(&r.classification_type, r.content.as_str()).into()
				).collect()
		}
	}
}
