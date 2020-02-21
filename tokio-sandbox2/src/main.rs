#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get("https://example.com/").await?;
    println!("{:?}", res);
    for (key, val) in res.headers() {
        println!("{:?}: {:?}", key, val)
    }
    let body = res.text().await?;
    println!("{}", body);
    Ok(())
}
