use axum::{body::Body, handler::Handler, http::Request, Server};
use axum_macros::debug_handler;
use futures::TryStreamExt;
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
async fn handler(request: Request<Body>) -> String {
    let (parts, body) = request.into_parts();

    let mut output = parts.headers.iter().fold(
        format!("{:?} 200 OK\n", parts.version),
        |mut acc, (key, value)| {
            acc.push_str(&format!("\n{key}: {value:?}"));
            acc
        },
    );

    let body_string = body
        .map_ok(|bytes| String::from_utf8_lossy(bytes.as_ref()).to_string())
        .try_collect::<String>()
        .await
        .unwrap();

    output.push_str(&format!("\n\n{body_string}"));

    output
}
