
#[tokio::main]
async fn main() {
    let fut = blocking::unblock(||{
        loop{
            println!("a");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    let fut2 = tokio::task::spawn_blocking(||{
        loop{
            println!("b");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    let fut3 = async {
        loop {
            println!("c");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    };
    let fut4  = tokio::time::sleep(std::time::Duration::from_secs(3));
    tokio::select!{
        _ = fut => {
            println!("never");
        }
        _ = fut2 => {
            println!("never");
        }
        _ = fut3 => {
            println!("never");
        }
        _ = fut4 => {
            println!("done");
        }
    }
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("fin");
}
