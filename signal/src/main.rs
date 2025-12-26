#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    tokio::signal::ctrl_c().await?;
    println!("Hello, world!");
    Ok(())
}
