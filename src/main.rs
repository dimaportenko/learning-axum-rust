use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router, middleware,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(static_routes());

    // region: --- Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start server
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

fn static_routes() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// e.g. `/hello/?name=Dima`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} handler_hello {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello, <strong>{name}!</strong>"))
}

// e.g. `/hello2/Dima`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} handler_hello2 {name:?}", "HANDLER");

    Html(format!("Hello, <strong>{name}!</strong>"))
}
