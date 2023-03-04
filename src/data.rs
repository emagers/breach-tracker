use std::env;

use diesel::{SqliteConnection, Connection, RunQueryDsl, QueryDsl, dsl::sql, ExpressionMethods};
use dotenvy::dotenv;

use crate::{schema::{breach_data::{self}, classification, last_retrieved}, datamodels::{BreachData, NewBreachData, NewClassification, Classification, LastRetrieved, NewLastRetrieved, State}, dto::Breach};

pub fn establish_connection() -> SqliteConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	SqliteConnection::establish(&database_url)
			.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_breach_data(conn: &mut SqliteConnection, data: &crate::dto::Breach) -> Result<(usize, usize), String> {
	let mut breach_inserted = 0;
	// Creates the values to be stored
	let bd = NewBreachData {
		date_reported: data.date_reported,
		organization_name: data.organization_name.clone(),
		date_of_breach: data.date_of_breach,
		affected_count: data.affected_count,
		loc: data.loc.into(),
		breach_type: data.breach_type.into(),
		link: data.link.clone(),
	};

	let mut classes: Vec<NewClassification> = data.leaked_info.iter().map(|r| crate::datamodels::NewClassification {
		breach_data_id: 0,
		content: match r {
			crate::dto::ClassificationType::Unknown(c, _) => c.clone(),
			_ => "".to_string()
		},
		classification_type: r.into()
	}).collect();
	// ---

	let new_breach_id;

	// Checks if breach already exists, if so we do not need to use the existing record ID for all classifications
	let existing_breach = breach_data::dsl::breach_data
		.filter(breach_data::dsl::date_reported.eq(&bd.date_reported))
		.filter(breach_data::dsl::organization_name.eq(&bd.organization_name))
		.filter(breach_data::dsl::loc.eq(&bd.loc))
		.load::<BreachData>(conn).expect("Could not query breaches");

	if existing_breach.len() == 0 {
		_ = diesel::insert_into(breach_data::table)
			.values(bd)
			.execute(conn)
			.expect("Error inserting");

		new_breach_id = breach_data::table.find(sql("last_insert_rowid()")).get_result::<BreachData>(conn).expect("Error retrieving inserted breach data").id;
		breach_inserted = 1;
	}
	else {
		new_breach_id = existing_breach[0].id;
	}
	// ---

	// Sets breach_id on each classification
	classes.iter_mut().for_each(|classification| classification.breach_data_id = new_breach_id);
	// ---

	// Checks if classifications already exist, if not, insert
	if breach_inserted == 0 {
		classes = classes.iter().filter(|class| classification::dsl::classification
			.filter(classification::dsl::breach_data_id.eq(class.breach_data_id))
			.filter(classification::dsl::classification_type.eq(class.classification_type))
			.load::<Classification>(conn).unwrap().len() == 0).map(|class| class.clone()).collect::<Vec<NewClassification>>();
	}
	let inserted = diesel::insert_into(classification::table)
		.values(classes)
		.execute(conn)
		.expect("Error inserting classifications");
	// ---

	Ok((breach_inserted, inserted))
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

pub fn insert_last_retrieved(conn: &mut SqliteConnection, last_retrieved: NewLastRetrieved) -> Result<(), String> {
	let _ = diesel::insert_into(last_retrieved::table).values([last_retrieved]).execute(conn).expect("Failed inserting last retrieved");

	Ok(())
}

pub fn get_last_retrieved(conn: &mut SqliteConnection, location: State) -> Result<Option<LastRetrieved>, String> {
	let results = last_retrieved::dsl::last_retrieved
		.filter(last_retrieved::dsl::loc.eq(location))
		.order(last_retrieved::dsl::retrieved_date.desc())
		.limit(1)
		.load::<LastRetrieved>(conn)
		.expect(&format!("Could not retrieve last retrieved date for location {:?}", location));

	if results.len() == 1 {
		return Ok(Some(results[0]))
	}
	Ok(None)
}