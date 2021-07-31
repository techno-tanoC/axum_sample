use axum::prelude::*;
use http::{HeaderMap, StatusCode};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = route("/", get(response));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn response() -> (StatusCode, HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert("x-hello", "world".parse().unwrap());
    (StatusCode::CREATED, headers, "created")
}
