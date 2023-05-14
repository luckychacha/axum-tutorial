use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/foo", get(get_foo).post(post_foo));
    let host = "127.0.0.1";
    let port = "3000";
    let builder = axum::Server::bind(&format!("{host}:{port}").parse().unwrap());
    println!("Server start at: {host}:{port}");

    builder.serve(app.into_make_service()).await.unwrap();
}

async fn get_foo() {}
async fn post_foo() {}
