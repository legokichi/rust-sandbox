#![feature(async_await)]

use serde::{Serialize, Deserialize};
use serde_json::json;
use futures::compat::Future01CompatExt as _;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};
use std::error::Error;
use log::info;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    pub permanent_table_name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "version")]
enum DeviceTable {
    DeviceTable20190228(DeviceTable20190228),
    DeviceTable20190707(DeviceTable20190707),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct DeviceTable20190228 {
    pub hash_key: String,
    pub range_key: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct DeviceTable20190707 {
    pub hash_key: String,
    pub range_key: String,
}

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    dotenv::dotenv().ok();
    env_logger::try_init().ok();
    let config = envy::from_env::<Config>()?;


    let client = DynamoDbClient::new(Region::default());
    use rusoto_dynamodb::{QueryInput, QueryOutput, AttributeValue};
    let table = DeviceTable::DeviceTable20190228(DeviceTable20190228{hash_key: "a".to_string(), range_key: "b".to_string()});
    let QueryOutput {
        items,
        ..
    } = client.query(QueryInput{
        table_name: config.permanent_table_name,
        key_condition_expression: Some("#hash_key = :hash_key".to_string()),
        expression_attribute_names: Some(serde_json::from_value(json!({
            "#hash_key": "hash_key",
        }))?),
        expression_attribute_values: Some(serde_dynamodb::to_hashmap(&table)?),
        ..Default::default()
    }).compat().await?;
    info!("{:?}", items);

    Ok(())
}
