use axum::{Router, routing::get};
use marcus_radell_net::kits::health::Health;
use maud::{Markup, html};
use tower_http::services::ServeFile;

async fn get_home_page() -> Markup {
    html! {
        h1 { "Marcus Rådell - Tech Leader | Senior Fullstack Software Developer"}
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let health = Health::new();
    let router = Router::new()
        .route("/", get(get_home_page))
        .nest("/health", health.get_router())
        .nest_service("/favicon.ico", ServeFile::new("public/favicon.ico"));
    tracing::info!("Starting server!");
    Ok(router.into())
}
