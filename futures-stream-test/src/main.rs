#[tokio::main]
async fn main(){
    use futures::SinkExt;
    use futures::StreamExt;
    let (mut tx, rx) = futures::channel::mpsc::channel::<u8>(1);
    let (tx2, mut rx2) = futures::channel::mpsc::channel::<u8>(1);
    
    let fut1 = async move{
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        while let Some(o) = rx2.next().await {
            println!("{:?}", o);
        }
        println!("closed")
    };
    let fut2 = async move {
        for i in 0..10_u8 {
            tx.send(i).await.unwrap();
        }
        // tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        drop(tx);
    };
    let fut3 = rx.map(Ok).forward(tx2);
    let ((), (), o) = futures::future::join3(fut1,fut2,fut3).await;
    println!("{:?}", o);
}
