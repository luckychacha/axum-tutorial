use std::net::SocketAddr;

use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/tickets", post(post_foo))
        .route("/foo/:id", get(get_foo));
    // let host = "127.0.0.1";
    // let port = "3000";
    // let builder = axum::Server::bind(&format!("{host}:{port}").parse().unwrap());
    // println!("Server start at: {host}:{port}");

    // builder.serve(app.into_make_service()).await.unwrap();

    // Better way to start server.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> Listening on {addr}\n");

    let builder = axum::Server::bind(&addr);

    builder.serve(app.into_make_service()).await.unwrap();
}

async fn get_foo(Path(id): Path<u64>) {
    println!("{id:?}");
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Foo {
    pub uid: u64,
    pub uname: String,
}
async fn post_foo(Json(foo): Json<Foo>) -> impl IntoResponse {
    println!("{foo:?}");
    println!("{:?}", foo.uname);
    // test(foo.clone()).await?;
    Json(foo)
}

async fn test(foo: Foo) {}
