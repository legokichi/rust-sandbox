mod auth;
mod db;
mod http;
mod model;

#[derive(serde::Deserialize)]
struct Config {
    host_addr: String,
    database_url: String,
    facebook_client_id: oauth2::ClientId,
    facebook_client_secret: oauth2::ClientSecret,
    github_client_id: oauth2::ClientId,
    github_client_secret: oauth2::ClientSecret,
    redirect_url: oauth2::RedirectUrl,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    env_logger::init();
    // use tracing_subscriber::layer::SubscriberExt;
    // use tracing_subscriber::util::SubscriberInitExt;
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
    //         |_| "axum_login=debug,tower_sessions=debug,sqlx=warn,tower_http=debug,oauth2=trace".into(),
    //     )))
    //     .with(tracing_subscriber::fmt::layer())
    //     .try_init()?;
    let Config {
        host_addr,
        database_url,
        facebook_client_id,
        facebook_client_secret,
        github_client_id,
        github_client_secret,
        redirect_url,
    } = envy::from_env::<Config>()?;

    let pool = sqlx::sqlite::SqlitePool::connect(&database_url).await?;

    sqlx::migrate!().run(&pool).await?;

    let session_store = tower_sessions_sqlx_store::SqliteStore::new(pool.clone());
    session_store.migrate().await?;

    let deletion_task = tokio::task::spawn({
        use tower_sessions::ExpiredDeletion;
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60))
    });

    let session_layer = tower_sessions::SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(tower_sessions::cookie::SameSite::Lax) // Ensure we send the cookie from the OAuth redirect.
        .with_expiry(tower_sessions::Expiry::OnInactivity(
            std::time::Duration::from_secs(600).try_into().unwrap(),
        ));

    let backend =
        crate::auth::Backend::new(pool.clone(), facebook_client_id, facebook_client_secret, github_client_id, github_client_secret, redirect_url);
    let auth_layer = axum_login::AuthManagerLayerBuilder::new(backend, session_layer).build();

    let st = crate::http::State::from_pool(pool).unwrap();

    let app = axum::Router::new()
        .route("/", axum::routing::get(crate::http::auth::index))
        .route_layer(axum_login::login_required!(
            crate::auth::Backend,
            login_url = "/login"
        ))
        .route("/login", axum::routing::get(crate::http::auth::login_page))
        .route("/login", axum::routing::post(crate::http::auth::login))
        .route("/logout", axum::routing::get(crate::http::auth::logout))
        .route(
            "/oauth/callback",
            axum::routing::get(crate::http::auth::callback),
        )
        .layer(auth_layer)
        .with_state(st);

    let listener = tokio::net::TcpListener::bind(host_addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await?;

    deletion_task.await??;
    Ok(())
}

async fn shutdown_signal(deletion_task_abort_handle: tokio::task::AbortHandle) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
