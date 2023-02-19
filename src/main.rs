use chrono::{NaiveDate};

pub mod retrievers;
pub mod datamodels;
pub mod dto;
pub mod schema;
pub mod data;

use data::{establish_connection, create_breach_data, get_breaches};
use retrievers::{wa_retriever::WaRetriever, Retriever};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let conn = &mut establish_connection();
	// let rec = WaRetriever{};

	// let client = reqwest::Client::new();

	// let options = retrievers::RetrieverOptions {
	// 	collect_until: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
	// };

	// let breaches = rec.retrieve(&client, &options).await?;

	// for breach in breaches {
	// 	let _ = create_breach_data(conn, &breach);
	// }

	let b2 = get_breaches(conn);

	println!("{}", json!(b2?));

	Ok(())
}
