use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;

mod error;
pub mod model;
mod types;

use crate::model::ModelController;

// If don't use `pub use self::error::{Error, Result};` here, then we need to use `error::Error` in `src/web/routes_login.rs`
// When we use `pub use self::error::{Error, Result};` here, then we can use `Error` in `src/web/routes_login.rs`
pub use self::error::{Error, Result};

mod web;

use types::HelloParams;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mc = ModelController::new().await?;
    let app = Router::new()
        .merge(route_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> Listening on {addr}\n");

    let builder = axum::Server::bind(&addr);

    builder.serve(app.into_make_service()).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}

fn route_hello() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(tower_http::services::ServeDir::new("./")))
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
