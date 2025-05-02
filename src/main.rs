use axum::{Router, routing::get};
use marcus_radell_net::Health;

async fn get_status() -> String {
    let health = Health::new();
    health.to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/status", get(get_status));
    println!("Starting server!");
    Ok(router.into())
}
