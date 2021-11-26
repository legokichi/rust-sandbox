#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(lambda_runtime::handler_fn(
        |request: serde_json::Value, _ctx| async {
            Err(anyhow::anyhow!("error")) as Result<(), anyhow::Error>
        },
    ))
    .await
}
