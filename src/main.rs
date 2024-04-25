use axum::{
    extract::Path,
    response::Redirect,
    routing::get,
    Router
};
use redis::{Client, Commands};
use std::env;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:key", get(redirect));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn redirect(Path(key): Path<String>) -> Redirect {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let client = Client::open(redis_url).expect("Failed to connect to Redis");
    let mut conn = client.get_connection().expect("Failed to get Redis connection");

    let redis_key = format!("/{}", key);

    match conn.get::<&str, String>(&redis_key) {
        Ok(url) => Redirect::permanent(&url),
        Err(_) => Redirect::permanent("/404"),
    }
}