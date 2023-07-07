use axum::{routing::post, Json, Router};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{types::login::LoginPayload, web, Error, Result};
pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "admin" || payload.pwd != "admin" {
        return Err(Error::LoginFail);
    }

    // FIXME: Implement real auth-token generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // Create the success body
    let body = Json(json!({
        "status": "success",
        "message": "Login success",
    }));

    Ok(body)
}
