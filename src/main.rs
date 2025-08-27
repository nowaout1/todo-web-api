use axum::Router;
use axum::http::Method;
use todo_web_api::AppState;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use todo_web_api::config::Config;
use todo_web_api::todo::create_todo_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::parse();
    let listener = TcpListener::bind(config.api_addr()).await?;
    let state = AppState::new(config).await?;

    let cors_layer = CorsLayer::new()
        // TODO: .allow_origin(std::any::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let router = Router::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer)
        .merge(create_todo_router())
        .with_state(state);

    tracing::info!("Listening on {:?}", config.api_addr());

    axum::serve(listener, router).await?;

    Ok(())
}
