use chrono::{NaiveDate, Days};

pub mod retrievers;
pub mod datamodels;
pub mod dto;
pub mod schema;
pub mod data;
pub mod parsers;

use data::{establish_connection, create_breach_data, get_last_retrieved};
use datamodels::State;
use diesel::SqliteConnection;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use retrievers::{multi_page::MultiPage, single_page::SinglePage, Retriever};
use serde_json::json;

use crate::{datamodels::NewLastRetrieved, data::insert_last_retrieved, parsers::{ca_parser::CaParser, or_parser::OrParser, wa_parser::WaParser}};

#[tokio::main]
async fn main() -> Result<(), ()> {
	// CA does not support OpenSSL3 so we need to use a custom config that allows legacy redirects
	std::env::set_var("OPENSSL_CONF", "./openssl_temp.cnf");

	let conn = &mut establish_connection();

	let result = process_california(conn).await;
	//let result = process_washington(conn).await;

	match result {
		Ok(_) => Ok(()),
		Err(err) => { println!("{:?}", err); return Err(()); }
	}
}

fn create_headers() -> HeaderMap<HeaderValue> {
	let mut headers = HeaderMap::new();

	headers.insert(ACCEPT, "*/*".parse().unwrap());
	headers.insert(USER_AGENT, "breach_tracker".parse().unwrap());

	headers
}

async fn process_washington(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
	let rec = MultiPage{};

	let client = reqwest::Client::new();

	let last_recieved = get_last_retrieved(conn, State::WA)?;

	let options = retrievers::RetrieverOptions {
		collect_until: match last_recieved {
			Some(lr) => lr.retrieved_date.checked_sub_days(Days::new(1)).unwrap(),
			None => NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
		},
		base_url: "https://www.atg.wa.gov/data-breach-notifications?page=".to_string(),
		headers: create_headers()
	};

	println!("Retrieving WA data breaches with options {:?}", options);

	let breaches = rec.retrieve(&client, Box::new(WaParser{}), &options).await?;

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
	let rec = SinglePage{};

	let client = reqwest::Client::new();

	let last_recieved = get_last_retrieved(conn, State::OR)?;

	let options = retrievers::RetrieverOptions {
		collect_until: match last_recieved {
			Some(lr) => lr.retrieved_date.checked_sub_days(Days::new(1)).unwrap(),
			None => NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
		},
		base_url: "https://justice.oregon.gov/consumer/databreach/".to_string(),
		headers: create_headers()
	};

	println!("Retrieving OR data breaches with options {:?}", options);

	let breaches = rec.retrieve(&client, Box::new(OrParser{}), &options).await?;

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

async fn process_california(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
	let rec = SinglePage{};

	let client = reqwest::Client::new();

	let last_recieved = get_last_retrieved(conn, State::CA)?;

	let options = retrievers::RetrieverOptions {
		collect_until: match last_recieved {
			Some(lr) => lr.retrieved_date.checked_sub_days(Days::new(1)).unwrap(),
			None => NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
		},
		base_url: "https://oag.ca.gov/privacy/databreach/list".to_string(),
		headers: create_headers()
	};

	println!("Retrieving CA data breaches with options {:?}", options);

	let breaches = rec.retrieve(&client, Box::new(CaParser{}), &options).await?;

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
			println!("No new breaches to insert in CA");
		}
	}
	else {
		println!("No new breaches to insert in CA");
	}

	Ok(())
}
