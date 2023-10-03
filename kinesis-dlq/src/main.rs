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
    //
    let retry = 2;
    let batch_size = 10;
    let bisect = true;
    let report = false;
    let sleep_sec = 0;
    let now = chrono::Utc::now();

    let mut datas = vec![];
    for i in 0..500 {
        if i % 2 == 0 {
            datas.push(Data { now, i, ok: false });
            continue;
        }
        datas.push(Data { now, i, ok: true });
    }
    assert_eq!(datas.len(), 500);
    test(
        now,
        retry,
        batch_size,
        bisect,
        report,
        &datas,
        sleep_sec,
        std::env::var("QUEUE_URL").unwrap().as_str(),
        std::env::var("QUEUE_ARN").unwrap().as_str(),
        std::env::var("STREAM_ARN").unwrap().as_str(),
        std::env::var("LAMBDA_ARN").unwrap().as_str(),
    )
    .await;
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Data {
    now: chrono::DateTime<chrono::Utc>,
    i: i32,
    ok: bool,
}
async fn test(
    now: chrono::DateTime<chrono::Utc>,
    retry: i32,
    batch_size: i32,
    bisect: bool,
    report: bool,
    testdatas: &[Data],
    sleep_sec: u64,
    queue_url: &str,
    queue_arn: &str,
    stream_arn: &str,
    lambda_arn: &str,
) {
    let config = aws_config::load_from_env().await;
    let sqs = aws_sdk_sqs::Client::new(&config);
    let kds = aws_sdk_kinesis::Client::new(&config);
    let lmd = aws_sdk_lambda::Client::new(&config);
    println!("DLQ を空にする");
    {
        loop {
            let aws_sdk_sqs::operation::receive_message::ReceiveMessageOutput { messages, .. } =
                sqs.receive_message()
                    .queue_url(queue_url)
                    .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
                    .max_number_of_messages(10)
                    .visibility_timeout(1)
                    .wait_time_seconds(10)
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
                dbg!(&msg.body);
                let aws_sdk_sqs::operation::delete_message::DeleteMessageOutput { .. } = sqs
                    .delete_message()
                    .queue_url(queue_url)
                    .receipt_handle(msg.receipt_handle.unwrap())
                    .send()
                    .await
                    .unwrap();
            }
        }
    }
    println!("ESM を無効化");
    {
        let aws_sdk_lambda::operation::list_event_source_mappings::ListEventSourceMappingsOutput {
            event_source_mappings,
            ..
        } = lmd
            .list_event_source_mappings()
            .function_name(lambda_arn)
            .event_source_arn(stream_arn)
            .send()
            .await
            .unwrap();
        for esm in event_source_mappings.unwrap() {
            dbg!(&esm.uuid);
            let aws_sdk_lambda::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput {
                ..
            } = lmd
                .delete_event_source_mapping()
                .uuid(esm.uuid.unwrap())
                .send()
                .await
                .unwrap();
        }
    }
    println!("テストデータを kds に溜める");
    {
        let recs = testdatas
            .iter()
            .map(|o| {
                aws_sdk_kinesis::types::PutRecordsRequestEntry::builder()
                    .data(aws_sdk_kinesis::primitives::Blob::new(
                        serde_json::to_vec(o).unwrap(),
                    ))
                    .partition_key(uuid::Uuid::new_v4().to_string())
                    .build()
            })
            .collect::<Vec<_>>();
        let aws_sdk_kinesis::operation::put_records::PutRecordsOutput {
            failed_record_count: _,
            records: _,
            encryption_type: _,
            ..
        } = kds
            .put_records()
            .stream_arn(stream_arn)
            .set_records(Some(recs))
            .send()
            .await
            .unwrap();
    }
    println!("ESM を有効化");
    {
        use aws_smithy_types_convert::date_time::DateTimeExt;
        let mut builder = lmd
            .create_event_source_mapping()
            .event_source_arn(stream_arn)
            .function_name(lambda_arn)
            .enabled(true)
            .starting_position(aws_sdk_lambda::types::EventSourcePosition::AtTimestamp)
            .starting_position_timestamp(aws_sdk_lambda::primitives::DateTime::from_chrono_utc(now))
            .batch_size(batch_size)
            .maximum_batching_window_in_seconds(0)
            .destination_config(
                aws_sdk_lambda::types::DestinationConfig::builder()
                    .on_failure(
                        aws_sdk_lambda::types::OnFailure::builder()
                            .destination(queue_arn.to_string())
                            .build(),
                    )
                    .build(),
            )
            .maximum_record_age_in_seconds(-1)
            .bisect_batch_on_function_error(bisect)
            .maximum_retry_attempts(retry)
            .parallelization_factor(1)
            .tumbling_window_in_seconds(0);
        if report {
            builder = builder.function_response_types(
                aws_sdk_lambda::types::FunctionResponseType::ReportBatchItemFailures,
            );
        }
        let aws_sdk_lambda::operation::create_event_source_mapping::CreateEventSourceMappingOutput {
            uuid,
            ..
        } = builder
            .send()
            .await
            .unwrap();
        dbg!(&uuid);
    }
    println!("DLQ に貯まるのを待つ");
    tokio::time::sleep(std::time::Duration::from_secs(sleep_sec)).await;
    println!("DLQ の中身を確認");
    let mut count = 0;
    let mut sqs_msg_count = 0;
    let mut obj = std::collections::HashMap::<i32, i32>::new();
    loop {
        let aws_sdk_sqs::operation::receive_message::ReceiveMessageOutput { messages, .. } = sqs
            .receive_message()
            .queue_url(queue_url)
            .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
            .max_number_of_messages(10)
            .visibility_timeout(1)
            .wait_time_seconds(10)
            .send()
            .await
            .unwrap();
        if messages.is_none() {
            // if count >= testdatas.len() {
            //     break;
            // }
            dbg!(obj.len());
            dbg!(count);
            dbg!(sqs_msg_count);
            continue;
        }
        if let Some(ref msgs) = messages {
            if msgs.len() == 0 {
                // if count >= testdatas.len() {
                //     break;
                // }
            }
        }
        for msg in messages.unwrap() {
            sqs_msg_count += 1;
            dbg!(&msg.body);
            let aws_sdk_sqs::types::Message { body, .. } = msg;
            let dlr = serde_json::from_str::<DeadLetterRecord>(&body.unwrap()).unwrap();
            let KinesisBatchInfo {
                shard_id,
                start_sequence_number,
                end_sequence_number,
                approximate_arrival_of_first_record: _,
                approximate_arrival_of_last_record: _,
                batch_size: _,
                stream_arn: _,
            } = dlr.kinesis_batch_info;
            dbg!(end_sequence_number.clone() == start_sequence_number.clone());
            let aws_sdk_kinesis::operation::get_shard_iterator::GetShardIteratorOutput {
                shard_iterator,
                ..
            } = kds
                .get_shard_iterator()
                .stream_arn(stream_arn)
                .shard_id(shard_id)
                .shard_iterator_type(aws_sdk_kinesis::types::ShardIteratorType::AtSequenceNumber)
                .starting_sequence_number(start_sequence_number.clone())
                .send()
                .await
                .unwrap();
            use std::str::FromStr;
            let start_sequence_number =
                num_bigint::BigInt::from_str(&start_sequence_number).unwrap();
            let end_sequence_number = num_bigint::BigInt::from_str(&end_sequence_number).unwrap();
            let aws_sdk_kinesis::operation::get_records::GetRecordsOutput {
                records,
                next_shard_iterator: _,
                millis_behind_latest: _,
                child_shards: _,
                ..
            } = kds
                .get_records()
                .shard_iterator(shard_iterator.unwrap())
                .stream_arn(stream_arn)
                //.limit(1)    // limit 1 でエラーデータのみ取得できる
                .send()
                .await
                .unwrap();
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
                //dbg!(end_sequence_number.clone() == sequence_number.clone());
                if sequence_number > end_sequence_number {
                    continue;
                }
                let data = serde_json::from_slice::<Data>(data.unwrap().as_ref()).unwrap();
                dbg!(&data);
                obj.insert(data.i, obj.get(&data.i).unwrap_or(&0) + 1);
                count += 1;
                dbg!(obj.len());
                dbg!(count);
                dbg!(sqs_msg_count);
                assert_eq!(data.ok, false);
            }
            let aws_sdk_sqs::operation::delete_message::DeleteMessageOutput { .. } = sqs
                .delete_message()
                .queue_url(queue_url)
                .receipt_handle(msg.receipt_handle.unwrap())
                .send()
                .await
                .unwrap();
        }
    }

    // for i in 0..testdatas.len() {
    //     let a = obj.get(&(i as i32)).unwrap_or(&-1);
    //     dbg!((i, a));
    // }
    dbg!(obj.len());
    dbg!(count);
    dbg!(sqs_msg_count);
}
