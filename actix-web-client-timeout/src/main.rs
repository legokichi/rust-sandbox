use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::error::Error;
use futures::prelude::*;
use actix_web::{web, App, HttpResponse, HttpServer};

// #[tokio::main]
#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let srvfut = server();

    let clifut = client();

    let ((), ()) = futures::future::try_join(srvfut, clifut).await?;
    
    Ok(())
}
async fn server() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = futures::channel::mpsc::channel::<()>(10);
    let tx = Arc::new(Mutex::new(tx));
    let srv = HttpServer::new({ let tx = tx.clone(); move|| {
        App::new()
            .data(tx.clone())
            .route("/", web::get().to(handler))
    }})
        .bind("127.0.0.1:8088")?
        .run();
    let srv2 = srv.clone();
    let fut = async move {
        rx.next().await;
        println!("stopping");
        srv2.stop(true).await;
        ()
    };
    actix_rt::spawn(fut);
    srv.await?;
    println!("stopped");
    Ok(())
}

async fn handler(tx: web::Data<Arc<Mutex<futures::channel::mpsc::Sender<()>>>>) -> Result<HttpResponse, actix_web::Error> {
    println!("1");
    tokio::time::delay_for(Duration::from_secs(1)).await;
    println!("2");
    tokio::time::delay_for(Duration::from_secs(1)).await;
    println!("3");
    tokio::time::delay_for(Duration::from_secs(1)).await;
    tx.lock().unwrap().try_send(()).unwrap();
    Ok(HttpResponse::Ok().finish())
}

async fn client() -> Result<(), Box<dyn Error>> {
    tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
    let cli = reqwest::ClientBuilder::new().build()?;
    let res: reqwest::Response = cli
        .get("http://localhost:8088/")
        .timeout(Duration::from_millis(1500))
        .send()
        .map_err(|err|{ println!("{:?}", err); err })
        .await?;
    println!("res: {:?}", res);
    let text = res.text().await?;
    println!("res: {}", text);
    Ok(())
}
