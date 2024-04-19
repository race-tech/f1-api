use std::net::SocketAddr;

use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use chrono::{DateTime, Duration, Utc};
use redis::Commands;

use infrastructure::CachePool;
use shared::error;
use shared::prelude::*;

const RATE_LIMITER_KEY_PREFIX: &str = "RATE_LIMITER_";

#[derive(Clone)]
pub struct RateLimiter {
    pub kind: RateLimiterKind,
    pub cache: CachePool,
}

#[derive(Clone, Copy)]
pub enum RateLimiterKind {
    SlidingWindow {
        request_num: usize,
        duration: Duration,
    },
}

pub async fn mw_rate_limiter(
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
    let conn = &mut rate_limiter.cache.get()?;

    let key = format!("{}{}", RATE_LIMITER_KEY_PREFIX, ip_addr);
    let now = Utc::now();
    let timestamps: String = match conn.get(&key) {
        Ok(res) => res,
        Err(_) => {
            let timestamps =
                match serde_json::to_string(&[(now.timestamp(), now.timestamp_subsec_nanos())]) {
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
        let time_to_wait = rate_limiter.time_to_wait(&timestamps, now).num_seconds();
        return Err(
            error!(RateLimitReached => "you reached the rate limit, please wait `{}s` before your next request", time_to_wait),
        );
    }

    timestamps.push((now.timestamp(), now.timestamp_subsec_nanos()));
    let timestamps = match serde_json::to_string(&timestamps) {
        Ok(s) => s,
        Err(e) => return Err(error!(InternalServer => e)),
    };

    conn.set(key, timestamps)?;
    Ok(next.run(req).await)
}

impl RateLimiter {
    fn cleanup(&self, timestamps: Vec<(i64, u32)>, now: DateTime<Utc>) -> Vec<(i64, u32)> {
        self.kind.cleanup(timestamps, now)
    }

    fn is_allowed(&self, timestamps: &[(i64, u32)]) -> bool {
        self.kind.is_allowed(timestamps)
    }

    fn time_to_wait(&self, timestamps: &[(i64, u32)], now: DateTime<Utc>) -> Duration {
        self.kind.time_to_wait(timestamps, now)
    }
}

impl RateLimiterKind {
    fn cleanup(&self, timestamps: Vec<(i64, u32)>, now: DateTime<Utc>) -> Vec<(i64, u32)> {
        match self {
            RateLimiterKind::SlidingWindow { duration, .. } => {
                cleanup(timestamps, |date| now - date > *duration)
            }
        }
    }

    fn is_allowed(&self, timestamps: &[(i64, u32)]) -> bool {
        match self {
            RateLimiterKind::SlidingWindow { request_num, .. } => timestamps.len() < *request_num,
        }
    }

    fn time_to_wait(&self, timestamps: &[(i64, u32)], now: DateTime<Utc>) -> Duration {
        match self {
            RateLimiterKind::SlidingWindow { duration, .. } => {
                // SAFETY: timestamps is not empty and secs and nsecs
                // are from DateTime::timestamp and DateTime::timestamp_subsec_nanos
                let first = timestamps
                    .first()
                    .map(|&(secs, nsecs)| DateTime::from_timestamp(secs, nsecs).unwrap())
                    .unwrap();
                *duration - (now - first)
            }
        }
    }
}

fn cleanup<F>(timestamps: Vec<(i64, u32)>, mut f: F) -> Vec<(i64, u32)>
where
    F: FnMut(DateTime<Utc>) -> bool,
{
    timestamps
        .into_iter()
        .skip_while(|&(secs, nsecs)| {
            // SAFETY: secs and nsecs are from DateTime::timestamp
            // and DateTime::timestamp_subsec_nanos
            let date = DateTime::from_timestamp(secs, nsecs).unwrap();
            f(date)
        })
        .collect()
}

impl Default for RateLimiterKind {
    fn default() -> Self {
        Self::SlidingWindow {
            request_num: 120,
            duration: Duration::seconds(10),
        }
    }
}
