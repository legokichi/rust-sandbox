#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    log::debug!("Hello, world!");
    let mut futs = vec![];
    for i in 0..5 {
        futs.push(tokio::spawn(hoge(i)));
    }
    let rets = futures::future::try_join_all(futs).await?;
    let rets = rets.into_iter().collect::<Result<Vec<_>, _>>()?;
    log::debug!("{:?}", rets); 
    Ok(())
}

#[tracing::instrument]
async fn hoge(msg: u8) -> anyhow::Result<()> {
    if msg % 2 == 0 {
        return Err(anyhow::anyhow!("error"));
    }
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    let id = std::thread::current().id();
    log::debug!("{id:?}:{msg}");
    Ok(())
}

async fn _huga() -> anyhow::Result<()> {
    let span = tracing::span!(tracing::Level::INFO, "huga");
    let _enter = span.enter();
    log::debug!("Writing to stream");
    tracing::debug!("Writing to stream");
    tracing::event!(tracing::Level::INFO, "something happened");
    Ok(())
}
