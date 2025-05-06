use axum::{Router, routing::get};
use marcus_radell_net::kits::status::{Health, Status};
use maud::{Markup, html};

async fn get_home_page() -> Markup {
    html! {
        h1 { "Marcus Rådell - Tech Leader | Senior Fullstack Software Developer"}
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let health = Health::new();
    let router = Router::new()
        .route(
            "/health",
            get({
                let health = health.clone();
                || async move { health.get_status_route() }
            }),
        )
        .route("/", get(get_home_page));
    tracing::info!("Starting server!");
    Ok(router.into())
}
