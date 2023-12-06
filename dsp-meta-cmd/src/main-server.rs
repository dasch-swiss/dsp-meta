use std::sync::Arc;
use std::time::Duration;

use config::Config;
use dsp_meta::app_state::AppState;
use dsp_meta::domain::service::project_metadata_service::ProjectMetadataService;
use dsp_meta::repo::service::project_metadata_repository::ProjectMetadataRepository;
use pid1::Pid1Settings;
use tracing::{trace, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    Pid1Settings::new()
        .enable_log(true)
        .timeout(Duration::from_secs(2))
        .launch()
        .expect("pid1 launch");

    // configure tracing library
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    trace!("Ivan was here!");

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    trace!("Version: {}", VERSION);

    let settings = Config::builder()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("DSP_META"))
        .build()
        .unwrap();

    let data_dir = settings
        .get::<String>("data_dir")
        .unwrap_or("/data".to_string());

    let frontend_dir = settings
        .get::<String>("frontend_dir")
        .unwrap_or("/public".to_string());

    let shared_state = Arc::new(AppState {
        project_metadata_service: ProjectMetadataService::new(ProjectMetadataRepository::new(
            &data_dir,
        )),
        frontend_dir,
        version: VERSION,
    });

    // build our application with a single route

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(dsp_meta::api::router::router(shared_state).into_make_service())
        .await
        .unwrap();
}
