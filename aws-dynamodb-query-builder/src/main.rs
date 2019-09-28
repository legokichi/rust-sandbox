#![feature(async_await)]
#![allow(unused_imports)]

#[macro_use]
extern crate serde_dynamodb_derive;

use serde::{Serialize, Deserialize};
use serde_json::json;

use futures::compat::{Future01CompatExt as _, Stream01CompatExt as _};

use dynomite::DynamoDbExt;

use dynomite::{Item, FromAttributes, Attributes};

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};
use rusoto_dynamodb::{QueryInput, QueryOutput, AttributeValue};

use serde_dynamodb::ToQueryInput;

use std::error::Error;

use log::info;



#[derive(Deserialize, Debug, Clone)]
struct Config {
    pub permanent_table_name: String,
}

#[derive(Serialize, Deserialize, Item, ToQueryInput, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Person {
    pub hash_key: String,
    pub range_key: String,
}

fn list_device_query(o: ListDeviceRequest) -> QueryInput {

}
fn parse_device_query(q: QueryOutput) -> Vec<Device> {

}

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    dotenv::dotenv().ok();
    env_logger::try_init().ok();
    let config = envy::from_env::<Config>()?;

    let client = DynamoDbClient::new(Region::default());
    
    let QueryOutput {
        items,
        ..
    } = client.query(
        PersonQueryInput{
            hash_key: Some("a".to_string()),
            ..Default::default()
        }.to_query_input(config.permanent_table_name)
    ).compat().concat().await?;
    info!("{:?}", items);

    Ok(())
}
