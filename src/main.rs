use axum::{Router, routing::get};
use marcus_radell_net::kits::health::Health;
use maud::{Markup, html};
use std::net::SocketAddr;
use tower_http::services::ServeFile;

async fn get_home_page() -> Markup {
    html! {
        h1 { "Marcus Rådell - Tech Leader | Senior Fullstack Software Developer"}
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let mut health = Health::new();
    let router = Router::new()
        .route("/", get(get_home_page))
        .nest("/health", health.get_router())
        .nest_service("/favicon.ico", ServeFile::new("public/favicon.ico"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Starting server on http://{}", addr);
    health.set_ready();

    tracing::info!("Health status: {}", health.get_status());
    axum::serve(listener, router).await.unwrap();
}
