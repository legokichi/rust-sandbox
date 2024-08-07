mod api;
mod auth;
mod db;
mod model;
mod web;

#[derive(serde::Deserialize, Debug)]
struct Config {
    host_addr: String,
    database_url: String,
    github_client_id: oauth2::ClientId,
    github_client_secret: oauth2::ClientSecret,
    local_client_id: oauth2::ClientId,
    local_client_secret: oauth2::ClientSecret,
    facebook_client_id: oauth2::ClientId,
    facebook_client_secret: oauth2::ClientSecret,
    //instagram_client_id: oauth2::ClientId,
    //instagram_client_secret: oauth2::ClientSecret,
    redirect_url: oauth2::RedirectUrl,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    //env_logger::init();
    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .init();
    let Config {
        host_addr,
        database_url,
        github_client_id,
        github_client_secret,
        local_client_id,
        local_client_secret,
        facebook_client_id,
        facebook_client_secret,
        //instagram_client_id,
        //instagram_client_secret,
        redirect_url,
    } = envy::from_env::<Config>()?;
    use std::str::FromStr;
    let opt = sqlx::sqlite::SqliteConnectOptions::from_str(&database_url)?.foreign_keys(true);
    let pool = sqlx::sqlite::SqlitePool::connect_with(opt).await?;

    // query! マクロ使ってたらいらないはず
    sqlx::migrate!().run(&pool).await?;

    let session_store = tower_sessions_sqlx_store::SqliteStore::new(pool.clone());
    // セッションテーブルの作成
    session_store.migrate().await?;

    // セッションの定期削除タスク
    // tokio::task::spawn を rt=current_thread で使うと single thread で動く
    let deletion_task = tokio::task::spawn({
        use tower_sessions::ExpiredDeletion;
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(3600))
    });

    // cookie のセッションの設定
    let session_layer = tower_sessions::SessionManagerLayer::new(session_store)
        // 本番環境で有効にする
        //.with_secure(true)
        .with_secure(false)
        // oauth でリダイレクトするときにStrict だとエラーになる
        //.with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_same_site(tower_sessions::cookie::SameSite::Strict)
        .with_expiry(tower_sessions::Expiry::OnInactivity(
            std::time::Duration::from_secs(600).try_into()?,
        ));

    let backend = crate::auth::Backend::new(
        pool.clone(),
        crate::auth::ClientToken {
            client_id: local_client_id.clone(),
            client_secret: local_client_secret.clone(),
        },
        crate::auth::ClientToken {
            client_id: github_client_id.clone(),
            client_secret: github_client_secret.clone(),
        },
        crate::auth::ClientToken {
            client_id: facebook_client_id.clone(),
            client_secret: facebook_client_secret.clone(),
        },
        // crate::auth::instagram::ClientToken {
        //     client_id: instagram_client_id.clone(),
        //     client_secret: instagram_client_secret.clone(),
        // },
        redirect_url,
    );
    // 一般のリクエストで DB にアクセスするための State
    let st = crate::web::State::from_pool(pool)?;

    let app = axum::Router::new()
        // プライベートなページはないのでコメントアウト
        // .route_layer(axum_login::login_required!(
        //     crate::auth::Backend,
        //     login_url = "/"
        // ))
        .route("/", axum::routing::get(crate::web::index::index))
        .route("/login", axum::routing::post(crate::web::login::login))
        .route("/logout", axum::routing::post(crate::web::login::logout))
        .route(
            "/oauth/callback",
            axum::routing::get(crate::web::login::callback),
        )
        .route("/api", axum::routing::post(crate::web::api::api))
        .layer(axum_login::AuthManagerLayerBuilder::new(backend, session_layer).build())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_methods(tower_http::cors::Any)
                .allow_origin(tower_http::cors::Any),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::compression::CompressionLayer::new())
        .with_state(st);

    let listener = tokio::net::TcpListener::bind(host_addr).await?;

    axum::serve(listener, app)
        // これすると sqlite の中のセッションが永続化しない
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await?;

    match deletion_task.await {
        Ok(Ok(())) => {
            // nop
        }
        Ok(Err(e)) => {
            // session の削除タスクが異常終了
            Err(e)?;
        }
        Err(e) if e.is_cancelled() => {
            // nop
        }
        Err(e) => {
            // task が panic
            Err(e)?;
        }
    }
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
