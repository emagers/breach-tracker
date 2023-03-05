use chrono::{NaiveDate, Days};

pub mod retrievers;
pub mod datamodels;
pub mod dto;
pub mod schema;
pub mod data;
pub mod parsers;

use data::{establish_connection, create_breach_data, get_last_retrieved};
use diesel::SqliteConnection;
use parsers::{Parser, md_parser::MdParser};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT, CONTENT_LENGTH};
use retrievers::{multi_page::MultiPage, single_page::SinglePage, Retriever, RetrieverOptions, WebRequestType};
use serde_json::json;

use crate::{datamodels::NewLastRetrieved, data::insert_last_retrieved, parsers::{ca_parser::CaParser, or_parser::OrParser, wa_parser::WaParser, hi_parser::HiParser}};

#[tokio::main]
async fn main() -> Result<(), ()> {
	// CA does not support OpenSSL3 so we need to use a custom config that allows legacy redirects
	std::env::set_var("OPENSSL_CONF", "./openssl_temp.cnf");

	let conn = &mut establish_connection();

	let processor = ProcessorBuilder::new()
		// .process_option(get_wa_options(conn))
		// .process_option(get_hi_options(conn))
		// .process_option(get_ca_options(conn))
		// .process_option(get_or_options(conn))
		.process_options(get_md_options(conn))
		.build().unwrap();

	if let Err(err) = processor.process(conn).await {
		println!("{:?}", err);
	}

	Ok(())
}

fn create_headers(state: dto::State) -> HeaderMap<HeaderValue> {
	let mut headers = HeaderMap::new();

	headers.insert(ACCEPT, "*/*".parse().unwrap());
	headers.insert(USER_AGENT, "breach_tracker".parse().unwrap());

	match state {
		dto::State::WA => {},
		dto::State::CA => {},
		dto::State::OR => {},
		dto::State::HI => {},
		dto::State::MD => { headers.insert(CONTENT_LENGTH, "0".parse().unwrap()); },
	};

	headers
}

fn get_url_generator(state: &dto::State) -> Box<dyn Fn(String, String) -> String + Send> {
	match state {
		dto::State::WA => Box::new(|base_url, page| format!("{}{}", base_url, page)),
		dto::State::CA => Box::new(|base_url, _| base_url.to_string()),
		dto::State::OR => Box::new(|base_url, _| base_url.to_string()),
		dto::State::HI => Box::new(|base_url, _| base_url.to_string()),
		dto::State::MD => Box::new(|base_url, page| {
			if page == "0" {
				return base_url;
			}

			format!("{}{}", base_url, page)
		}),
	}
}

fn get_page_incrementer(state: &dto::State) -> Box<dyn Fn(i32) -> i32 + Send> {
	match state {
		dto::State::WA => Box::new(|page| page + 1),
		dto::State::CA => Box::new(|_| 0),
		dto::State::OR => Box::new(|_| 0),
		dto::State::HI => Box::new(|_| 0),
		dto::State::MD => Box::new(|page| page + 30),
	}
}

fn get_parser(state: &dto::State) -> Box<dyn Parser + Send> {
	match state {
		dto::State::WA => Box::new(WaParser{}),
		dto::State::CA => Box::new(CaParser{}),
		dto::State::OR => Box::new(OrParser{}),
		dto::State::HI => Box::new(HiParser{}),
		dto::State::MD => Box::new(MdParser{}),
	}
}

fn get_retriever(state: &dto::State) -> Box<dyn Retriever> {
	match state {
		dto::State::WA => Box::new(MultiPage{}),
		dto::State::CA => Box::new(SinglePage{}),
		dto::State::OR => Box::new(SinglePage{}),
		dto::State::HI => Box::new(SinglePage{}),
		dto::State::MD => Box::new(MultiPage{}),
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
		headers: create_headers(dto::State::WA),
		state: dto::State::WA,
		request_type: WebRequestType::Get,
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
		headers: create_headers(dto::State::CA),
		state: dto::State::CA,
		request_type: WebRequestType::Get,
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
		headers: create_headers(dto::State::OR),
		state: dto::State::OR,
		request_type: WebRequestType::Get,
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
		headers: create_headers(dto::State::HI),
		state: dto::State::HI,
		request_type: WebRequestType::Get,
	}
}

fn get_md_options(conn: &mut SqliteConnection) -> Vec<RetrieverOptions> {
	let mut opts = vec!();
	let last_recieved = get_last_retrieved(conn, dto::State::MD.into()).unwrap();

	opts.push(retrievers::RetrieverOptions {
		collect_until: match last_recieved {
			Some(lr) => lr.retrieved_date.checked_sub_days(Days::new(1)).unwrap(),
			None => NaiveDate::from_ymd_opt(-1, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
		},
		base_url: "https://www.marylandattorneygeneral.gov/_layouts/15/inplview.aspx?List=%7B04EBF6F4-B351-492F-B96D-167E2DE39C85%7D&View=%7BAC628F51-0774-4B71-A77E-77D6B9909F7E%7D&ViewCount=23&IsXslView=TRUE&IsCSR=TRUE&ListViewPageUrl=https%3A%2F%2Fwww.marylandattorneygeneral.gov%2Fpages%2Fidentitytheft%2Fbreachnotices.aspx&GroupString=%3B%23%3B%23&IsGroupRender=TRUE&WebPartID={AC628F51-0774-4B71-A77E-77D6B9909F7E}".to_string(),
		headers: create_headers(dto::State::MD),
		state: dto::State::MD,
		request_type: WebRequestType::Post,
	});

	opts.push(retrievers::RetrieverOptions {
		collect_until: match last_recieved {
			Some(lr) => lr.retrieved_date.checked_sub_days(Days::new(1)).unwrap(),
			None => NaiveDate::from_ymd_opt(-1, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
		},
		base_url: "https://www.marylandattorneygeneral.gov/_layouts/15/inplview.aspx?List=%7B04EBF6F4-B351-492F-B96D-167E2DE39C85%7D&View=%7BAC628F51-0774-4B71-A77E-77D6B9909F7E%7D&ViewCount=23&IsXslView=TRUE&IsCSR=TRUE&ListViewPageUrl=https%3A%2F%2Fwww.marylandattorneygeneral.gov%2Fpages%2Fidentitytheft%2Fbreachnotices.aspx&GroupString=%3B%232020%3B%23&IsGroupRender=TRUE&WebPartID={AC628F51-0774-4B71-A77E-77D6B9909F7E}".to_string(),
		headers: create_headers(dto::State::MD),
		state: dto::State::MD,
		request_type: WebRequestType::Post,
	});

	opts.push(retrievers::RetrieverOptions {
		collect_until: match last_recieved {
			Some(lr) => lr.retrieved_date.checked_sub_days(Days::new(1)).unwrap(),
			None => NaiveDate::from_ymd_opt(-1, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
		},
		base_url: "https://www.marylandattorneygeneral.gov/_layouts/15/inplview.aspx?List=%7B04EBF6F4-B351-492F-B96D-167E2DE39C85%7D&View=%7BAC628F51-0774-4B71-A77E-77D6B9909F7E%7D&ViewCount=23&IsXslView=TRUE&IsCSR=TRUE&ListViewPageUrl=https%3A%2F%2Fwww.marylandattorneygeneral.gov%2Fpages%2Fidentitytheft%2Fbreachnotices.aspx&GroupString=%3B%232021%3B%23&IsGroupRender=TRUE&WebPartID={AC628F51-0774-4B71-A77E-77D6B9909F7E}".to_string(),
		headers: create_headers(dto::State::MD),
		state: dto::State::MD,
		request_type: WebRequestType::Post,
	});

	opts.push(retrievers::RetrieverOptions {
		collect_until: match last_recieved {
			Some(lr) => lr.retrieved_date.checked_sub_days(Days::new(1)).unwrap(),
			None => NaiveDate::from_ymd_opt(-1, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
		},
		base_url: "https://www.marylandattorneygeneral.gov/_layouts/15/inplview.aspx?List=%7B04EBF6F4-B351-492F-B96D-167E2DE39C85%7D&View=%7BAC628F51-0774-4B71-A77E-77D6B9909F7E%7D&ViewCount=23&IsXslView=TRUE&IsCSR=TRUE&ListViewPageUrl=https%3A%2F%2Fwww.marylandattorneygeneral.gov%2Fpages%2Fidentitytheft%2Fbreachnotices.aspx&GroupString=%3B%232022%3B%23&IsGroupRender=TRUE&WebPartID={AC628F51-0774-4B71-A77E-77D6B9909F7E}".to_string(),
		headers: create_headers(dto::State::MD),
		state: dto::State::MD,
		request_type: WebRequestType::Post,
	});

	opts
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
		let rec = get_retriever(&options.state);

		let client = reqwest::Client::new();

		let breaches = rec.retrieve(&client, get_parser(&options.state), &options, get_page_incrementer(&options.state), get_url_generator(&options.state)).await?;

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

	pub fn process_options(mut self, options: Vec<RetrieverOptions>) -> ProcessorBuilder {
		for opt in options {
			self.to_process.push(opt);
		}

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