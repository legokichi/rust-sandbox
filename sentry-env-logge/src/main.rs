
fn main(){
    dotenv::dotenv().ok();
    let sentry_dsn = std::env::var("SETNRY_DSN").unwrap();
    let mut log_builder = env_logger::builder();
    log_builder.parse_filters("info");
    let logger = sentry::integrations::log::SentryLogger::with_dest(log_builder.build());
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Info);
    let integration = sentry::integrations::contexts::ContextIntegration::new()
        .add_os(true)
        .add_rust(true)
        .add_device(true);
    let mut opt = sentry::ClientOptions{
        dsn: Some(sentry_dsn.parse().unwrap()),
        ..Default::default()
    };
    let opt = opt.add_integration(integration);
    let _sentry = sentry::init(opt);    
    sentry::with_scope(
        |scope| {
            scope.set_extra("user", serde_json::json!({"hellow":"world"}));
        },
        || {
            log::error!("hello world");
            log::warn!("hello world");
            log::info!("hello world");
            log::debug!("hello world");
            log::trace!("hello world");
        }
    );

    std::thread::sleep(std::time::Duration::from_secs(1));
}