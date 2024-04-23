use std::net::SocketAddr;

use axum::Extension;
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use redis::Commands;
use time::{Duration, OffsetDateTime};

use infrastructure::ConnectionPool;
use shared::error;
use shared::prelude::*;

const RATE_LIMITER_KEY_PREFIX: &str = "RATE_LIMITER_";

#[derive(Clone)]
pub struct RateLimiter {
    kind: RateLimiterKind,
    request_num: usize,
    duration: Duration,
}

#[derive(Clone, Copy)]
pub enum RateLimiterKind {
    SlidingWindow,
}

pub async fn mw_rate_limiter(
    Extension(conn): Extension<ConnectionPool>,
    State(rate_limiter): State<RateLimiter>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    log::info!("hit rate_limiter");
    let ip_addr = req
        .extensions()
        .get::<axum::extract::ConnectInfo<SocketAddr>>()
        .map(|addr| addr.ip())
        .ok_or(error!(IpHeaderNotFound => "ip header not found"))?;
    let conn = &mut conn.cache.get()?;

    let key = format!("{}{}", RATE_LIMITER_KEY_PREFIX, ip_addr);
    let now = OffsetDateTime::now_utc();
    let timestamps: String = match conn.get(&key) {
        Ok(res) => res,
        Err(_) => {
            let timestamps = match serde_json::to_string(&[now.unix_timestamp()]) {
                Ok(s) => s,
                Err(e) => return Err(error!(InternalServer => e)),
            };

            match conn.set(key, timestamps) {
                Ok(()) => return Ok(next.run(req).await),
                Err(e) => return Err(error!(InternalServer => e)),
            }
        }
    };

    // SAFETY: values are stored this way in the cache
    let mut timestamps = rate_limiter.cleanup(serde_json::from_str(&timestamps).unwrap(), now);

    if !rate_limiter.is_allowed(&timestamps) {
        let time_to_wait = rate_limiter.time_to_wait(&timestamps, now).whole_seconds();
        return Err(
            error!(RateLimitReached => "you reached the rate limit, please wait `{}s` before your next request", time_to_wait),
        );
    }

    timestamps.push(now.unix_timestamp());
    let timestamps = match serde_json::to_string(&timestamps) {
        Ok(s) => s,
        Err(e) => return Err(error!(InternalServer => e)),
    };

    conn.set(key, timestamps)?;
    Ok(next.run(req).await)
}

impl RateLimiter {
    pub fn new<T: Into<RateLimiterKind>>(kind: T, request_num: usize, seconds: i64) -> Self {
        Self {
            kind: kind.into(),
            request_num,
            duration: Duration::seconds(seconds),
        }
    }

    fn cleanup(&self, timestamps: Vec<i64>, now: OffsetDateTime) -> Vec<i64> {
        cleanup(timestamps, |date| now - date > self.duration)
    }

    fn is_allowed(&self, timestamps: &[i64]) -> bool {
        timestamps.len() < self.request_num
    }

    fn time_to_wait(&self, timestamps: &[i64], now: OffsetDateTime) -> Duration {
        match self.kind {
            RateLimiterKind::SlidingWindow => {
                // SAFETY: timestamps is not empty and secs and nsecs
                // are from DateTime::timestamp and DateTime::timestamp_subsec_nanos
                let first = timestamps
                    .first()
                    .map(|&t| OffsetDateTime::from_unix_timestamp(t).unwrap())
                    .unwrap();
                self.duration - (now - first)
            }
        }
    }
}

fn cleanup<F>(timestamps: Vec<i64>, mut f: F) -> Vec<i64>
where
    F: FnMut(OffsetDateTime) -> bool,
{
    timestamps
        .into_iter()
        .skip_while(|&t| {
            // SAFETY: secs and nsecs are from DateTime::timestamp
            // and DateTime::timestamp_subsec_nanos
            let date = OffsetDateTime::from_unix_timestamp(t).unwrap();
            f(date)
        })
        .collect()
}

impl From<infrastructure::config::RateLimiterType> for RateLimiterKind {
    fn from(value: infrastructure::config::RateLimiterType) -> Self {
        match value {
            infrastructure::config::RateLimiterType::SlidingWindow => Self::SlidingWindow,
        }
    }
}

impl From<Option<infrastructure::config::RateLimiterType>> for RateLimiterKind {
    fn from(value: Option<infrastructure::config::RateLimiterType>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Self::SlidingWindow,
        }
    }
}
