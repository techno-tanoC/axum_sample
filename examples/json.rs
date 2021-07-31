use axum::{
    extract,
    response,
    prelude::*,
};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = route("/ping", post(ping));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct Ping {
    count: i64,
}

#[derive(Serialize)]
struct Pong {
    count: i64,
}

async fn ping(extract::Json(ping): extract::Json<Ping>) -> response::Json<Pong> {
    response::Json(Pong {
        count: ping.count + 1,
    })
}
