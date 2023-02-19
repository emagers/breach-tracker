use std::env;

use diesel::{SqliteConnection, Connection, RunQueryDsl, QueryDsl, dsl::sql};
use dotenvy::dotenv;

use crate::{schema::{breach_data, classification}, datamodels::{BreachData, NewBreachData, NewClassification, Classification}, dto::Breach};

pub fn establish_connection() -> SqliteConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	SqliteConnection::establish(&database_url)
			.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_breach_data(conn: &mut SqliteConnection, data: &crate::dto::Breach) -> Result<(usize, usize), String> {
	let bd = NewBreachData {
		date_reported: data.date_reported,
		organization_name: data.organization_name.clone(),
		date_of_breach: data.date_of_breach,
		affected_count: data.affected_count,
		loc: data.loc.into()
	};

	let mut classes: Vec<NewClassification> = data.leaked_info.iter().map(|r| crate::datamodels::NewClassification {
		breach_data_id: 0,
		content: match r {
			crate::dto::ClassificationType::Unknown(c, s) => c.clone(),
			_ => "".to_string()
		},
		classification_type: r.into()
	}).collect();

	_ = diesel::insert_into(breach_data::table)
		.values(bd)
		.execute(conn)
		.expect("Error inserting");

	let r = breach_data::table.find(sql("last_insert_rowid()")).get_result::<BreachData>(conn).expect("Error retrieving inserted breach data");

	classes.iter_mut().for_each(|classification| classification.breach_data_id = r.id);

	let inserted = diesel::insert_into(classification::table)
		.values(classes)
		.execute(conn)
		.expect("Error inserting classifications");

	Ok((1, inserted))
}

pub fn get_breaches(conn: &mut SqliteConnection) -> Result<Vec<Breach>, String> {
	let bdresults = breach_data::dsl::breach_data
		.load::<BreachData>(conn)
		.expect("Error loading breach data");

	let cresults = classification::dsl::classification
		.load::<Classification>(conn)
		.expect("Error retrieving classifications");

	let mut breaches: Vec<Breach> = vec!();
	for breach in bdresults.iter() {
		let bd = (breach, cresults.iter().filter(|r| r.breach_data_id == breach.id).collect::<Vec<&Classification>>());
		breaches.push(bd.into());
	}

	Ok(breaches)
}