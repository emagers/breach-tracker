use chrono::{NaiveDate, Days};

pub mod retrievers;
pub mod datamodels;
pub mod dto;
pub mod schema;
pub mod data;
pub mod parsers;

use data::{establish_connection, create_breach_data, get_last_retrieved};
use diesel::SqliteConnection;
use parsers::Parser;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use retrievers::{multi_page::MultiPage, single_page::SinglePage, Retriever, RetrieverOptions};
use serde_json::json;

use crate::{datamodels::NewLastRetrieved, data::insert_last_retrieved, parsers::{ca_parser::CaParser, or_parser::OrParser, wa_parser::WaParser, hi_parser::HiParser}};

#[tokio::main]
async fn main() -> Result<(), ()> {
	// CA does not support OpenSSL3 so we need to use a custom config that allows legacy redirects
	std::env::set_var("OPENSSL_CONF", "./openssl_temp.cnf");

	let conn = &mut establish_connection();

	let processor = ProcessorBuilder::new()
		.process_option(get_wa_options(conn))
		.process_option(get_hi_options(conn))
		.process_option(get_ca_options(conn))
		.process_option(get_or_options(conn))
		.build().unwrap();

	if let Err(err) = processor.process(conn).await {
		println!("{:?}", err);
	}

	Ok(())
}

fn create_headers() -> HeaderMap<HeaderValue> {
	let mut headers = HeaderMap::new();

	headers.insert(ACCEPT, "*/*".parse().unwrap());
	headers.insert(USER_AGENT, "breach_tracker".parse().unwrap());

	headers
}



fn get_parser(state: dto::State) -> Box<dyn Parser + Send> {
	match state {
		dto::State::WA => Box::new(WaParser{}),
		dto::State::CA => Box::new(CaParser{}),
		dto::State::OR => Box::new(OrParser{}),
		dto::State::HI => Box::new(HiParser{}),
		dto::State::MD => Box::new(WaParser{}),
	}
}

fn get_retriever(state: dto::State) -> Box<dyn Retriever> {
	match state {
		dto::State::WA => Box::new(MultiPage{}),
		dto::State::CA => Box::new(SinglePage{}),
		dto::State::OR => Box::new(SinglePage{}),
		dto::State::HI => Box::new(SinglePage{}),
		dto::State::MD => Box::new(SinglePage{}),
	}
}

fn get_wa_options(conn: &mut SqliteConnection) -> RetrieverOptions {
	let last_recieved = get_last_retrieved(conn, dto::State::WA.into()).unwrap();

	retrievers::RetrieverOptions {
		collect_until: match last_recieved {
			Some(lr) => lr.retrieved_date.checked_sub_days(Days::new(1)).unwrap(),
			None => NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
		},
		base_url: "https://www.atg.wa.gov/data-breach-notifications?page=".to_string(),
		headers: create_headers(),
		state: dto::State::WA,
	}
}

fn get_ca_options(conn: &mut SqliteConnection) -> RetrieverOptions {
	let last_recieved = get_last_retrieved(conn, dto::State::CA.into()).unwrap();

	retrievers::RetrieverOptions {
		collect_until: match last_recieved {
			Some(lr) => lr.retrieved_date.checked_sub_days(Days::new(1)).unwrap(),
			None => NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
		},
		base_url: "https://oag.ca.gov/privacy/databreach/list".to_string(),
		headers: create_headers(),
		state: dto::State::CA
	}
}

fn get_or_options(conn: &mut SqliteConnection) -> RetrieverOptions {
	let last_recieved = get_last_retrieved(conn, dto::State::OR.into()).unwrap();

	retrievers::RetrieverOptions {
		collect_until: match last_recieved {
			Some(lr) => lr.retrieved_date.checked_sub_days(Days::new(1)).unwrap(),
			None => NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
		},
		base_url: "https://justice.oregon.gov/consumer/databreach/".to_string(),
		headers: create_headers(),
		state: dto::State::OR,
	}
}

fn get_hi_options(conn: &mut SqliteConnection) -> RetrieverOptions {
	let last_recieved = get_last_retrieved(conn, dto::State::HI.into()).unwrap();

	retrievers::RetrieverOptions {
		collect_until: match last_recieved {
			Some(lr) => lr.retrieved_date.checked_sub_days(Days::new(1)).unwrap(),
			None => NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
		},
		base_url: "https://cca.hawaii.gov/ocp/notices/security-breach/".to_string(),
		headers: create_headers(),
		state: dto::State::HI
	}
}

fn get_md_options(conn: &mut SqliteConnection) -> RetrieverOptions {
	//let last_recieved = get_last_retrieved(conn, options.state.into())?;

	todo!()
}

pub struct Processor {
	to_process: Vec<RetrieverOptions>,
}

impl Processor {
	pub async fn process(&self, conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
		for options in self.to_process.iter() {
			if let Err(err) = self.process_breaches(conn, options).await {
				println!("{:?}", err)
			}
		}

		Ok(())
	}

	async fn process_breaches(&self, conn: &mut SqliteConnection, options: &retrievers::RetrieverOptions) -> Result<(), Box<dyn std::error::Error>> {
		let rec = get_retriever(options.state);

		let client = reqwest::Client::new();

		println!("Retrieving data breaches with options {:?}", options);

		let breaches = rec.retrieve(&client, get_parser(options.state), &options).await?;

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
					loc: options.state.into(),
					retrieved_date: breaches.first().unwrap().date_reported
				};

				let lr_result = insert_last_retrieved(conn, last_retrieved);

				println!("{}", json!(lr_result));
			}
			else {
				println!("No new breaches to insert in {:?}", options.state);
			}
		}
		else {
			println!("No new breaches to insert in {:?}", options.state);
		}

		Ok(())
	}
}

pub struct ProcessorBuilder {
	to_process: Vec<RetrieverOptions>
}

impl ProcessorBuilder {
	pub fn new() -> Self {
		Self {
			to_process: vec!()
		}
	}

	pub fn process_option(mut self, option: RetrieverOptions) -> ProcessorBuilder {
		self.to_process.push(option);
		self
	}

	pub fn build(self) -> Result<Processor, String> {
		if self.to_process.len() == 0 {
			return Err("Cannot create processor without any options".to_string())
		}

		Ok(Processor {
			to_process: self.to_process
		})
	}
}