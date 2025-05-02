use axum::{Router, routing::get};
use marcus_radell_net::Health;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn get_status() -> &'static str {
    let health = Health::new();

    match health.get_status() {
        marcus_radell_net::Status::Alive => "Alive",
        marcus_radell_net::Status::Ready => "Ready",
    }
}
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/status", get(get_status))
        .route("/", get(hello_world));
    println!("Starting server!");
    Ok(router.into())
}
