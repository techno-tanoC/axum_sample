use axum::{
    extract,
    prelude::*,
    response,
};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::add_extension::AddExtensionLayer;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(State { counter: 0 }));
    let app = route("/", get(increment))
        .layer(AddExtensionLayer::new(state));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct State {
    counter: i64,
}

type SharedState = Arc<Mutex<State>>;

async fn increment(extract::Extension(state): extract::Extension<SharedState>) -> response::Json<State> {
    let mut s = state.lock().await;
    s.counter += 1;
    let current = s.counter;
    response::Json(State { counter: current })
}
