#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let stream_arn = std::env::var("STREAM_ARN").unwrap();
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_kinesis::Client::new(&config);
    let date = chrono::DateTime::parse_from_rfc3339("2024-06-14T02:00:00.00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let aws_sdk_kinesis::operation::list_shards::ListShardsOutput { shards, .. } =
        client.list_shards().stream_arn(stream_arn.clone()).send().await?;
    for shard in shards.unwrap() {
        let client = client.clone();
        let stream_arn = stream_arn.clone();
        let fut = async move {
            //println!("Shard: {:?}", shard.shard_id);
            let shard_id = shard.shard_id;
            use aws_smithy_types_convert::date_time::DateTimeExt;
            let aws_sdk_kinesis::operation::get_shard_iterator::GetShardIteratorOutput {
                shard_iterator,
                ..
            } = client
                .get_shard_iterator()
                .stream_arn(stream_arn.clone())
                .shard_iterator_type(aws_sdk_kinesis::types::ShardIteratorType::AtTimestamp)
                .shard_id(shard_id)
                .timestamp(aws_sdk_kinesis::primitives::DateTime::from_chrono_utc(date))
                .send()
                .await?;
            let mut next = shard_iterator;
            loop {
                let aws_sdk_kinesis::operation::get_records::GetRecordsOutput {
                    records,
                    next_shard_iterator,
                    ..
                } = client
                    .get_records()
                    .stream_arn(stream_arn.clone())
                    .shard_iterator(next.clone().unwrap())
                    .send()
                    .await?;
                next = next_shard_iterator;
                for record in records {
                    let aws_sdk_kinesis::types::Record { data, .. } = record;
                    let json = serde_json::from_slice::<serde_json::Value>(data.as_ref())?;
                    println!("Record: {}", serde_json::to_string_pretty(&json).unwrap());
                }
                if next.is_none() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            Ok(()) as Result<(), anyhow::Error>
        };
        tokio::spawn(fut);
    }
    tokio::time::sleep(std::time::Duration::from_secs(60*60)).await;
    Ok(())
}
