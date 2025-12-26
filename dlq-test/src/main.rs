#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let stream_arn = std::env::var("STREAM_ARN").unwrap();

    let config = aws_config::load_from_env().await;
    let kds = aws_sdk_kinesis::Client::new(&config);

    let now = chrono::Utc::now();

    for j in 0..3 {
        let mut test_datas = vec![];
        for i in 0..500 {
            let data = if i % 2 == 0 {
                serde_json::json!({
                    "timestamp": now.timestamp_millis(),
                    "sequence_number": i+j*500,
                    "received_at": chrono::Utc::now().timestamp_millis(),
                    "thing_name": "thing_name",
                    "event": {"event_type": "act_unhealthy","act_id": 2,"uptime": 100},
                    "i": i,
                    "j": j,
                    "ok": false,
                })
            } else {
                serde_json::json!({
                    "timestamp": now.timestamp_millis(),
                    "sequence_number": i+j*500,
                    "received_at": chrono::Utc::now().timestamp_millis(),
                    "thing_name": "nyannnaynnnnaynn",
                    "event": {"event_type": "act_unhealthy","act_id": 2,"uptime": 100},
                    "i": i,
                    "j": j,
                    "ok": true,
                })
            };
            test_datas.push(data);
        }
        let recs = test_datas
            .iter()
            .map(|o| {
                let o = serde_json::to_vec(o).unwrap();
                let o = aws_sdk_kinesis::primitives::Blob::new(o);
                aws_sdk_kinesis::types::PutRecordsRequestEntry::builder()
                    .data(o)
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
            .stream_arn(&stream_arn)
            .set_records(Some(recs))
            .send()
            .await
            .unwrap();
    }
}
