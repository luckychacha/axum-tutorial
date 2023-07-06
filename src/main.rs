use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .merge(route_hello())
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

fn route_hello() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello2/:name", get(handler_hello2))
}

async fn get_foo(Path(id): Path<u64>) -> impl IntoResponse {
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

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// `/hello?name=Jen`
async fn hello_handler(Query(hello_params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello {hello_params:?}", "HANDLER");
    let name = hello_params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// `/hello2/Jen`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}
