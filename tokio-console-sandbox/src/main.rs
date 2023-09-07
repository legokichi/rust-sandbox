#[tokio::main]
async fn main() {
    //console_subscriber::init();

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        tokio::task::spawn_blocking(move || {
            //tokio::time::sleep(std::time::Duration::from_nanos(5)).await;
            std::thread::sleep(std::time::Duration::from_nanos(5));
            println!("hello");
        })
        .await.unwrap();
        //tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        interval.tick().await;
    }
}

