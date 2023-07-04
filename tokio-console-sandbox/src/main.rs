#[tokio::main]
async fn main() {
    // choose subscribers
    //tracing_log::LogTracer::init().unwrap();
    //tracing_log::env_logger::init();
    tracing_subscriber::fmt::init();
    //console_subscriber::init();

    let app = axum::Router::new().route("/", axum::routing::get(root));
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[tracing::instrument]
async fn root() -> &'static str {
    log::debug!("log");
    tracing::debug!("tracing");
    "Hello, World!"
}
