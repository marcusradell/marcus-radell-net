use axum::{Router, routing::get};
use marcus_radell_net::kits::status::Health;
use maud::{Markup, html};

async fn get_home_page() -> Markup {
    html! {
        h1 { "Marcus Rådell - Tech Leader | Senior Fullstack Software Developer"}
    }
}

async fn get_status() -> String {
    let health = Health::new();
    health.to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/status", get(get_status))
        .route("/", get(get_home_page));
    tracing::info!("Starting server!");
    Ok(router.into())
}
