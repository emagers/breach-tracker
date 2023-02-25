use chrono::{NaiveDate, Days};

pub mod retrievers;
pub mod datamodels;
pub mod dto;
pub mod schema;
pub mod data;

use data::{establish_connection, create_breach_data, get_last_retrieved};
use datamodels::State;
use diesel::SqliteConnection;
use retrievers::{wa::WaRetriever, Retriever};
use serde_json::json;

use crate::{datamodels::NewLastRetrieved, data::insert_last_retrieved, retrievers::or::OrRetriever};

#[tokio::main]
async fn main() -> Result<(), ()> {
	let conn = &mut establish_connection();

	let result = process_oregon(conn).await;
	//let result = process_washington(conn).await;

	match result {
		Ok(_) => Ok(()),
		Err(err) => { println!("{:?}", err); return Err(()); }
	}
}

async fn process_washington(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
	let rec = WaRetriever{};

	let client = reqwest::Client::new();

	let last_recieved = get_last_retrieved(conn, State::WA)?;

	let options = retrievers::RetrieverOptions {
		collect_until: match last_recieved {
			Some(lr) => lr.retrieved_date.checked_sub_days(Days::new(1)).unwrap(),
			None => NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
		}
	};

	println!("Retrieving WA data breaches with options {:?}", options);

	let breaches = rec.retrieve(&client, &options).await?;

	if breaches.len() > 0 {
		let mut inserted_breaches_count = 0;
		for breach in &breaches {
			let res = create_breach_data(conn, &breach);

			if let Ok((i, c)) = res {
				inserted_breaches_count += i;
				if i == 0 && c > 0 {
					println!("Created {} new classification(s) for {:?}", c, breach);
				}
			}
			else {
				println!("Error storing {:?}: {:?}", breach, res);
			}
		}

		println!("Inserted total of {} breaches", inserted_breaches_count);

		if inserted_breaches_count > 0 {
			let last_retrieved = NewLastRetrieved {
				loc: State::WA,
				retrieved_date: breaches.first().unwrap().date_reported
			};

			let lr_result = insert_last_retrieved(conn, last_retrieved);

			println!("{}", json!(lr_result));
		}
		else {
			println!("No new breaches to insert");
		}
	}
	else {
		println!("No new breaches to insert");
	}

	Ok(())
}

async fn process_oregon(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
	let rec = OrRetriever{};

	let client = reqwest::Client::new();

	let last_recieved = get_last_retrieved(conn, State::WA)?;

	let options = retrievers::RetrieverOptions {
		collect_until: match last_recieved {
			Some(lr) => lr.retrieved_date.checked_sub_days(Days::new(1)).unwrap(),
			None => NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
		}
	};

	println!("Retrieving OR data breaches with options {:?}", options);

	let breaches = rec.retrieve(&client, &options).await?;

	if breaches.len() > 0 {
		let mut inserted_breaches_count = 0;
		for breach in &breaches {
			let res = create_breach_data(conn, &breach);

			if let Ok((i, c)) = res {
				inserted_breaches_count += i;
				if i == 0 && c > 0 {
					println!("Created {} new classification(s) for {:?}", c, breach);
				}
			}
			else {
				println!("Error storing {:?}: {:?}", breach, res);
			}
		}

		println!("Inserted total of {} breaches", inserted_breaches_count);

		if inserted_breaches_count > 0 {
			let last_retrieved = NewLastRetrieved {
				loc: State::OR,
				retrieved_date: breaches.first().unwrap().date_reported
			};

			let lr_result = insert_last_retrieved(conn, last_retrieved);

			println!("{}", json!(lr_result));
		}
		else {
			println!("No new breaches to insert");
		}
	}
	else {
		println!("No new breaches to insert");
	}

	Ok(())
}
