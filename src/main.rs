use axum::{
    handler::Handler,
    http::{Method, Uri},
    Json, Server,
};
use axum_macros::debug_handler;
use serde::Serialize;
use serde_json::Value;

#[tokio::main]
async fn main() {
    Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(handler.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Serialize)]
struct RequestInformation {
    method: String,
    uri: String,
    json: Value,
}

#[debug_handler]
async fn handler(method: Method, uri: Uri, json: Option<Json<Value>>) -> Json<RequestInformation> {
    dbg!(Json(RequestInformation {
        method: method.to_string(),
        uri: uri.to_string(),
        json: json.map(|json| json.0).unwrap_or_default(),
    }))
}
