use axum::{
    Router,
    routing::{get, head},
};
use marcus_radell_net::kits::status::Health;

async fn get_status() -> String {
    let health = Health::new();
    health.to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/status", get(get_status))
        .route("/", head(|| async {}));
    tracing::info!("Starting server!");
    Ok(router.into())
}
