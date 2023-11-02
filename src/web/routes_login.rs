use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookies, Cookie};

use crate::{Error, Result, web};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies,payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} api_login", "HANDLER");

    // TODO: real db login implementation
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // FIXME: Implement real auth-token generation/signature 
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // Create success body
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}
