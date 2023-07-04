
fn main() {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(async {
        loop{
            let timer = tokio::time::sleep(std::time::Duration::from_micros(1));
            let fut1 = async move {
                let client = reqwest::blocking::Client::builder()
                    .timeout(Some(std::time::Duration::from_secs(30)))
                    .build().unwrap();
                let res = client
                    .head("http://duxca.com/").send();
                dbg!(&res);
            };
            tokio::select!{
                b = timer => {
                    dbg!(b);
                },
                a = fut1 => {
                    dbg!(a);
                },
            };
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }
    });
}
