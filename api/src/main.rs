use axum::{
    routing::get,
    Json,
    Router,
};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Json<ResponseMessage> {
    Json(ResponseMessage {
        message: "Hello world!".to_string(),
    })
}