
// #[tokio::main]
#[async_std::main]
async fn main() -> Result<(), surf::Exception> {
    let body = surf::get("https://httpbin.org/get").await?.body_string().await?;
    println!("{}", body);
    
    // let body = reqwest::get("http://example.com")
    // .await?
    // .text()
    // .await?;
    // println!("{}", body);
    Ok(())
}
