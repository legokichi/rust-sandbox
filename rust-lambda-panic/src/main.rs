
#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(lambda_runtime::handler_fn(
            func
    )).await?;
    Ok(())
}

async fn func(event: serde_json::Value, _: lambda_runtime::Context) -> Result<serde_json::Value, lambda_runtime::Error> {
    panic!("hoge");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(serde_json::json!({}))
}

/*
.await してから panic
{
      "errorMessage": "RequestId: 0715c771-5477-443c-9e1f-c4d0834c1038 Error: Runtime exited with error: exit status 101",
        "errorType": "Runtime.ExitError"
}
*/
