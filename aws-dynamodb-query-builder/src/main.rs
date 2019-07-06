#![feature(async_await)]

use serde::Deserialize;
use serde_json::json;
use futures::compat::Future01CompatExt;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};
use std::error::Error;
use log::info;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    permanent_table_name: String,
}

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    dotenv::dotenv().ok();
    env_logger::try_init().ok();
    let config = envy::from_env::<Config>()?;

    #[derive(Serialize, Deserialize)]
    struct Todo {
        id: uuid::Uuid,
        title: &'static str,
        done: bool,
    }
    serde_dynamodb

    let client = DynamoDbClient::new(Region::default());
    use rusoto_dynamodb::{QueryInput, QueryOutput, AttributeValue};
    let QueryOutput {
        items,
        ..
    } = client.query(QueryInput{
        table_name: config.permanent_table_name,
        key_condition_expression: Some("#hash_key = :hash_key".to_string()),
        expression_attribute_names: Some(serde_json::from_value(json!({
            "#hash_key": "hash_key",
        }))?),
        expression_attribute_values: Some(serde_json::from_value(json!({
            ":hash_key": Into::<AttributeValue>::into("cast-counter#1084141216#1783188814".to_string()),
        }))?),
        ..Default::default()
    }).compat().await?;
    info!("{:?}", items);

    Ok(())
}
