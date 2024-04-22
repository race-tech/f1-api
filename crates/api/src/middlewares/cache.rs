use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::Extension;
use axum::{http::Request, middleware::Next, response::Response};
use http_body_util::BodyExt;
use redis::Commands;

use infrastructure::ConnectionPool;
use shared::prelude::*;

const CACHE_KEY_PREFIX: &str = "cache";

#[derive(Clone, Copy)]
pub struct Cache {
    ttl: u64,
}

pub async fn mw_cache_layer(
    Extension(conn): Extension<ConnectionPool>,
    State(cache): State<Cache>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let conn = &mut conn.cache.get()?;
    let key = format!("{}:{}", CACHE_KEY_PREFIX, req.uri());

    let val: String = match conn.get(&key) {
        Ok(val) => val,
        Err(_) => {
            log::info!("missed cache, forwarding the request");
            let res = next.run(req).await;
            let (parts, body) = res.into_parts();
            let (val, bytes) = buffer_and_print(body).await?;
            let res = Response::from_parts(parts, Body::from(bytes));

            match conn.set_ex(key, val, cache.ttl) {
                Ok(()) => (),
                Err(e) => return Err(e.into()),
            }
            log::info!("added response to the cache with ttl: {}", cache.ttl);

            return Ok(res);
        }
    };
    log::info!("retrieved value from cache");
    let bytes = Bytes::from(val);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(bytes))?;

    Ok(response)
}

async fn buffer_and_print<B>(body: B) -> Result<(String, Bytes)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => todo!(),
    };

    let body = String::from_utf8(bytes.clone().into())?;
    Ok((body, bytes))
}

impl Cache {
    pub fn new(ttl: u64) -> Cache {
        Cache { ttl }
    }
}
