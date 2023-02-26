use chrono::{NaiveDateTime};
use diesel::{prelude::*, AsExpression, sql_types::*, FromSqlRow, serialize::{self, Output, ToSql}, deserialize::{self, FromSql}, backend::{self, Backend}};

#[repr(i32)]
#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression, PartialEq)]
#[diesel(sql_type = Integer)]
pub enum State {
	WA = 1,
	OR = 2,
	CA = 3,
}

impl<DB> ToSql<Integer, DB> for State
where
	DB: Backend,
	i32: ToSql<Integer, DB>,
{
	fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
		match self {
			State::WA => 1.to_sql(out),
			State::OR => 2.to_sql(out),
			State::CA => 3.to_sql(out),
		}
	}
}

impl<DB> FromSql<Integer, DB> for State
where
	DB: Backend,
	i32: FromSql<Integer, DB>,
{
	fn from_sql(bytes: backend::RawValue<DB>) -> deserialize::Result<Self> {
		match i32::from_sql(bytes)? {
			1 => Ok(State::WA),
			2 => Ok(State::OR),
			3 => Ok(State::CA),
			x => Err(format!("Unrecognized variant {}", x).into()),
		}
	}
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression, PartialEq)]
#[diesel(sql_type = Integer)]
pub enum ClassificationType {
	Unknown = 0,
	Name = 1,
	Username = 2,
	Password = 3,
	SecurityQuestionOrAnswer = 4,
	Email = 5,
	StudentId = 6,
	DateOfBirth = 7,
	SocialSecurity = 8,
	DriversLicense = 9,
	StateId = 10,
	BankingInformation = 11,
	FinancialInformation = 12,
	PassportNumber = 13,
	HealthInsurancePolicy = 14,
	MedicalInformation = 15,
	BiometricData = 16,
}

impl<DB> ToSql<Integer, DB> for ClassificationType
where
	DB: Backend,
	i32: ToSql<Integer, DB>,
{
	fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
		match self {
			ClassificationType::Unknown => 0.to_sql(out),
			ClassificationType::Name => 1.to_sql(out),
			ClassificationType::Username => 2.to_sql(out),
			ClassificationType::Password => 3.to_sql(out),
			ClassificationType::SecurityQuestionOrAnswer => 4.to_sql(out),
			ClassificationType::Email => 5.to_sql(out),
			ClassificationType::StudentId => 6.to_sql(out),
			ClassificationType::DateOfBirth => 7.to_sql(out),
			ClassificationType::SocialSecurity => 8.to_sql(out),
			ClassificationType::DriversLicense => 9.to_sql(out),
			ClassificationType::StateId => 10.to_sql(out),
			ClassificationType::BankingInformation => 11.to_sql(out),
			ClassificationType::FinancialInformation => 12.to_sql(out),
			ClassificationType::PassportNumber => 13.to_sql(out),
			ClassificationType::HealthInsurancePolicy => 14.to_sql(out),
			ClassificationType::MedicalInformation => 15.to_sql(out),
			ClassificationType::BiometricData => 16.to_sql(out),
		}
	}
}

impl<DB> FromSql<Integer, DB> for ClassificationType
where
	DB: Backend,
	i32: FromSql<Integer, DB>,
{
	fn from_sql(bytes: backend::RawValue<DB>) -> deserialize::Result<Self> {
		match i32::from_sql(bytes)? {
			0 => Ok(ClassificationType::Unknown),
			1 => Ok(ClassificationType::Name),
			2 => Ok(ClassificationType::Username),
			3 => Ok(ClassificationType::Password),
			4 => Ok(ClassificationType::SecurityQuestionOrAnswer),
			5 => Ok(ClassificationType::Email),
			6 => Ok(ClassificationType::StudentId),
			7 => Ok(ClassificationType::DateOfBirth),
			8 => Ok(ClassificationType::SocialSecurity),
			9 => Ok(ClassificationType::DriversLicense),
			10 => Ok(ClassificationType::StateId),
			11 => Ok(ClassificationType::BankingInformation),
			12 => Ok(ClassificationType::FinancialInformation),
			13 => Ok(ClassificationType::PassportNumber),
			14 => Ok(ClassificationType::HealthInsurancePolicy),
			15 => Ok(ClassificationType::MedicalInformation),
			16 => Ok(ClassificationType::BiometricData),
			x => Err(format!("Unrecognized variant {}", x).into()),
		}
	}
}

#[derive(Queryable, Debug, PartialEq, Identifiable)]
#[diesel(table_name = crate::schema::classification)]
pub struct Classification {
	pub id: i32,
	pub breach_data_id: i32,
	pub content: String,
	pub classification_type: ClassificationType,
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::classification)]
pub struct NewClassification {
	pub breach_data_id: i32,
	pub content: String,
	pub classification_type: ClassificationType
}

#[derive(Queryable, Debug, PartialEq, Identifiable)]
#[diesel(table_name = crate::schema::breach_data)]
pub struct BreachData {
	pub id: i32,
	pub date_reported: NaiveDateTime,
	pub organization_name: String,
	pub date_of_breach: NaiveDateTime,
	pub affected_count: i32,
	pub loc: State,
	pub link: Option<String>,
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::breach_data)]
pub struct NewBreachData {
	pub date_reported: NaiveDateTime,
	pub organization_name: String,
	pub date_of_breach: NaiveDateTime,
	pub affected_count: i32,
	pub loc: State,
	pub link: Option<String>,
}

#[derive(Queryable, Debug, PartialEq, Identifiable, Copy, Clone)]
#[diesel(table_name = crate::schema::last_retrieved)]
pub struct LastRetrieved {
	pub id: i32,
	pub loc: State,
	pub retrieved_date: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::last_retrieved)]
pub struct NewLastRetrieved {
	pub loc: State,
	pub retrieved_date: NaiveDateTime,
}
