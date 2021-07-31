use axum::{
    async_trait,
    extract::{self, FromRequest, RequestParts},
    prelude::*,
    response::{self, IntoResponse},
};
use http::Response;
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use tower::BoxError;

#[tokio::main]
async fn main() {
    let app = route("/users/:user_id", post(user_message));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct Params {
    user_id: u64,
    name: String,
}

#[async_trait]
impl<B> FromRequest<B> for Params
where
    B: Send + http_body::Body,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Response<Body>;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let url_params = extract::UrlParamsMap::from_request(req)
            .await
            .map_err(IntoResponse::into_response)?;
        let user_id = url_params
            .get_typed("user_id")
            .unwrap()
            .unwrap();

        let json_params: extract::Json<serde_json::Value> = extract::Json::from_request(req)
            .await
            .map_err(IntoResponse::into_response)?;
        let name = json_params.0.get("name")
            .unwrap()
            .as_str()
            .unwrap();

        Ok(Params {
            user_id: user_id,
            name: name.to_string(),
        })
    }
}

#[derive(Serialize)]
struct Message {
    message: String,
}

async fn user_message(
    params: Params
) -> response::Json<Message> {
    let user_id = params.user_id;
    let name = params.name;
    response::Json(Message {
        message: format!("user_id {} is {}", user_id, name),
    })
}
