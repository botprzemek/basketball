use axum::{
    body::Body,
    extract::State,
    http::{Method, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;
use redis::{AsyncCommands, aio::MultiplexedConnection};
use std::fmt::Display;
use std::sync::Arc;

fn internal_error<E: Display>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub struct ResponseCache {
    connection: MultiplexedConnection,
    prefix: String,
    ttl: u64,
}

impl ResponseCache {
    pub fn new(connection: MultiplexedConnection, ttl: u64) -> Self {
        Self {
            connection,
            ttl,
            prefix: "basketball:http:cache".to_string(),
        }
    }

    fn get_connection(&self) -> MultiplexedConnection {
        self.connection.clone()
    }

    fn get_key(&self, path: &str) -> String {
        format!("{}:{}", self.prefix, path)
    }

    pub async fn get_response(&self, path: &str) -> anyhow::Result<Option<Vec<u8>>> {
        let mut connection = self.get_connection();
        let key = self.get_key(path);

        let result = connection.get(key).await?;

        Ok(result)
    }

    pub async fn set_response(&self, path: &str, data: Vec<u8>) -> anyhow::Result<()> {
        let mut connection = self.get_connection();
        let key = self.get_key(path);

        let _: () = connection.set_ex(key, data, self.ttl).await?;

        Ok(())
    }

    pub async fn invalidate_response(&self, path: &str) -> anyhow::Result<()> {
        let mut connection = self.get_connection();
        let key = self.get_key(path);

        let _: () = connection.del(key).await?;

        Ok(())
    }
}

pub async fn layer(
    State(cache): State<Arc<ResponseCache>>,
    request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let path = request
        .uri()
        .path()
        .split('/')
        .take(4)
        .collect::<Vec<&str>>()
        .join("/");
    let method = request.method().clone();

    if method == Method::GET
        && let Ok(Some(cached_body)) = cache.get_response(&path).await
    {
        return Response::builder()
            .header("Content-Type", "application/json")
            .header("Cache-Control", format!("max-age={}", cache.ttl))
            .body(Body::from(cached_body))
            .map_err(internal_error);
    }

    let response = next.run(request).await;
    if !response.status().is_success() {
        return Ok(response);
    }

    let cache_clone = cache.clone();
    let path_clone = path.clone();

    if method != Method::GET {
        tokio::spawn(async move {
            let _ = cache_clone.invalidate_response(&path_clone).await;
            println!("INVALIDATED!");
        });

        return Ok(response);
    }

    let (parts, body) = response.into_parts();
    let collected = body.collect().await.map_err(internal_error)?;
    let bytes = collected.to_bytes();
    let bytes_cloned = bytes.clone();

    tokio::spawn(async move {
        let _ = cache_clone
            .set_response(&path_clone, bytes_cloned.to_vec())
            .await;
    });

    Ok(Response::from_parts(parts, Body::from(bytes)))
}
