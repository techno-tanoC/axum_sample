use axum::{
    extract,
    prelude::*,
    response,
};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = route("/users/:user_id", post(user_message));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct Name {
    name: String,
}

#[derive(Serialize)]
struct Message {
    message: String,
}

async fn user_message(
    params: extract::Path<(u64,)>,
    name: extract::Json<Name>,
) -> response::Json<Message> {
    let user_id = (*params).0;
    let name = &name.name;
    response::Json(Message {
        message: format!("Hello {}, your id is {}", name, user_id),
    })
}
