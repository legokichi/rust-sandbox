#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DeadLetterRecord {
    request_context: RequestContext,
    response_context: ResponseContext,
    version: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "KinesisBatchInfo")]
    kinesis_batch_info: KinesisBatchInfo,
}
#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestContext {
    // uuid
    request_id: String,
    function_arn: String,
    // RetryAttemptsExhausted
    condition: String,
    approximate_invoke_count: i32,
}
#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ResponseContext {
    // 200
    status_code: i32,
    // $LATEST
    executed_version: String,
    // Unhandled
    function_error: String,
}
#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct KinesisBatchInfo {
    shard_id: String,
    // u128
    start_sequence_number: String,
    // u128
    end_sequence_number: String,
    approximate_arrival_of_first_record: chrono::DateTime<chrono::Utc>,
    approximate_arrival_of_last_record: chrono::DateTime<chrono::Utc>,
    batch_size: i32,
    stream_arn: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let queue_url = std::env::var("QUEUE_URL").unwrap();
    let stream_arn = std::env::var("STREAM_ARN").unwrap();
    let config = aws_config::load_from_env().await;

    let sqs = aws_sdk_sqs::Client::new(&config);
    let kds = aws_sdk_kinesis::Client::new(&config);
    //let lmd = aws_sdk_lambda::Client::new(&config);
    //let ddb = aws_sdk_dynamodb::Client::new(&config);

    let mut obj = std::collections::HashMap::<i64, i64>::new();
    loop {
        let aws_sdk_sqs::operation::receive_message::ReceiveMessageOutput { messages, .. } = sqs
            .receive_message()
            .queue_url(&queue_url)
            .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
            .max_number_of_messages(10)
            .visibility_timeout(60)
            .wait_time_seconds(3)
            .send()
            .await
            .unwrap();
        if messages.is_none() {
            break;
        }
        if let Some(ref msgs) = messages {
            if msgs.len() == 0 {
                break;
            }
        }
        for msg in messages.unwrap() {
            let aws_sdk_sqs::operation::delete_message::DeleteMessageOutput { .. } = sqs
                .delete_message()
                .queue_url(&queue_url)
                .receipt_handle(msg.receipt_handle.unwrap())
                .send()
                .await
                .unwrap();

            let msg_json_str = msg.body.unwrap();
            println!("{msg_json_str}");
            let msg_json = serde_json::from_str::<serde_json::Value>(&msg_json_str).unwrap();
            let dlr = serde_json::from_value::<DeadLetterRecord>(msg_json).unwrap();
            let KinesisBatchInfo {
                shard_id,
                start_sequence_number,
                end_sequence_number,
                approximate_arrival_of_first_record: _,
                approximate_arrival_of_last_record: _,
                batch_size,
                stream_arn: _,
            } = dlr.kinesis_batch_info;
            if batch_size > 1 {
                continue;
            }
            let ret = loop {
                let res = kds
                    .get_shard_iterator()
                    .stream_arn(&stream_arn)
                    .shard_id(&shard_id)
                    .shard_iterator_type(
                        aws_sdk_kinesis::types::ShardIteratorType::AtSequenceNumber,
                    )
                    .starting_sequence_number(start_sequence_number.clone())
                    .send()
                    .await;
                if let Ok(ret) = res {
                    break ret;
                }
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            };
            let aws_sdk_kinesis::operation::get_shard_iterator::GetShardIteratorOutput {
                shard_iterator,
                ..
            } = ret;
            let shard_iterator = shard_iterator.unwrap();
            use std::str::FromStr;
            let end_sequence_number = num_bigint::BigInt::from_str(&end_sequence_number).unwrap();
            let ret = loop {
                let res = kds
                    .get_records()
                    .shard_iterator(&shard_iterator)
                    .stream_arn(&stream_arn)
                    .send()
                    .await;
                if let Ok(ret) = res {
                    break ret;
                }
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            };
            let aws_sdk_kinesis::operation::get_records::GetRecordsOutput {
                records,
                next_shard_iterator: _,
                millis_behind_latest: _,
                child_shards: _,
                ..
            } = ret;
            for rec in records.unwrap() {
                let aws_sdk_kinesis::types::Record {
                    approximate_arrival_timestamp: _,
                    data,
                    encryption_type: _,
                    partition_key: _,
                    sequence_number,
                    ..
                } = rec;
                let sequence_number =
                    num_bigint::BigInt::from_str(&sequence_number.unwrap()).unwrap();
                if sequence_number > end_sequence_number {
                    // out of range
                    continue;
                }
                let json_blob = data.unwrap();
                let json = serde_json::from_slice::<serde_json::Value>(json_blob.as_ref()).unwrap();
                let json_str = serde_json::to_string(&json).unwrap();
                println!("{json_str}");
                let seq_num = json.pointer("/sequence_number").unwrap().as_i64().unwrap();
                obj.insert(seq_num, obj.get(&seq_num).unwrap_or(&0) + 1);
                //count += 1;
                //dbg!(obj.len());
                //dbg!(count);
                //dbg!(sqs_msg_count);
                //assert_eq!(data.ok, false);
            }
        }
        let map = obj
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    serde_json::Value::Number(serde_json::Number::from(*v)),
                )
            })
            .collect::<serde_json::map::Map<_, _>>();
        let _obj = serde_json::to_string_pretty(&serde_json::Value::Object(map)).unwrap();
        println!("{_obj}");
        println!("{}", obj.len());
    }
}

#[test]
fn test() {
    let json_str = r#"{"KinesisBatchInfo":{"approximateArrivalOfFirstRecord":"2023-10-03T07:39:15.910Z","approximateArrivalOfLastRecord":"2023-10-03T07:39:15.910Z","batchSize":1,"endSequenceNumber":"49644654839123957412784931542545307712589324552060796930","shardId":"shardId-000000000000","startSequenceNumber":"49644654839123957412784931542545307712589324552060796930","streamArn":"arn:aws:kinesis:ap-northeast-1:978700748103:stream/actcast-dev-kinesis-event"},"requestContext":{"approximateInvokeCount":6,"condition":"RetryAttemptsExhausted","functionArn":"arn:aws:lambda:ap-northeast-1:978700748103:function:actcast-dev-event-log-updater-kinesis:deploy","requestId":"6ff299e7-e54f-40b6-a2da-3ca04af011ed"},"responseContext":{"executedVersion":"$LATEST","functionError":"Unhandled","statusCode":200},"timestamp":"2023-10-03T08:19:36.258Z","version":"1.0"}"#;
    let dead_letter_record = serde_json::from_str::<DeadLetterRecord>(json_str).unwrap();
    println!("{:?}", dead_letter_record);
}
