mod http;
mod db;
mod model;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenvy::dotenv().ok();
    env_logger::init();
    let pool = sqlx::sqlite::SqlitePool::connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
    let app = crate::http::app().with_state(pool);
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
