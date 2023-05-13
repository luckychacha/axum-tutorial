use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello World!" }));
    let host = "127.0.0.1";
    let port = "3000";
    let builder = axum::Server::bind(&format!("{host}:{port}").parse().unwrap());
    println!("Server start at: {host}:{port}");

    builder.serve(app.into_make_service()).await.unwrap();
}
