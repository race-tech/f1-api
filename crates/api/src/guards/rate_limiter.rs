use chrono::{DateTime, Duration, Utc};
use redis::Commands;
use rocket::request::{self, FromRequest, Outcome, Request};

use infrastructure::ConnectionPool;
use shared::error;
use shared::error::ErrorKind::*;

const RATE_LIMITER_KEY_PREFIX: &str = "RATE_LIMITER_";

pub struct RateLimiter;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RateLimiter {
    type Error = shared::error::Error;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // SAFETY: This values should always be defined
        let ip_header = req
            .rocket()
            .config()
            .ip_header
            .as_ref()
            .unwrap()
            .to_string();
        let pool = req.rocket().state::<ConnectionPool>().unwrap();
        let window = req.rocket().state::<SlidingWindow>().unwrap();
        let rate_limiter = &mut pool.cache.get().unwrap();

        let ip_addr = match req.real_ip() {
            Some(ip_addr) => ip_addr,
            None => {
                return Outcome::Error((
                    IpHeaderNotFound.into(),
                    error!(IpHeaderNotFound => "ip header not found, expected to find it under the `{}` header", ip_header),
                ))
            }
        };

        let key = format!("{}{}", RATE_LIMITER_KEY_PREFIX, ip_addr);
        let now = Utc::now();
        let timestamps: String = match rate_limiter.get(&key) {
            Ok(res) => res,
            Err(_) => {
                let timestamps =
                    match serde_json::to_string(&[(now.timestamp(), now.timestamp_subsec_nanos())])
                    {
                        Ok(s) => s,
                        Err(e) => {
                            return Outcome::Error((
                                InternalServer.into(),
                                error!(InternalServer => e),
                            ))
                        }
                    };

                match rate_limiter.set(key, timestamps) {
                    Ok(()) => return Outcome::Success(Self),
                    Err(e) => {
                        return Outcome::Error((InternalServer.into(), error!(InternalServer => e)))
                    }
                }
            }
        };

        let window_validation = |date| now - date > window.duration;
        let mut timestamps = cleanup(
            // SAFETY: values are stored this way in the cache
            serde_json::from_str(&timestamps).unwrap(),
            window_validation,
        );

        if timestamps.len() == window.request_num {
            // SAFETY: timestamps is not empty and secs and nsecs
            // are from DateTime::timestamp and DateTime::timestamp_subsec_nanos
            let first = timestamps
                .first()
                .map(|&(secs, nsecs)| DateTime::from_timestamp(secs, nsecs).unwrap())
                .unwrap();
            let time_to_wait = (window.duration - (now - first)).num_seconds();
            return Outcome::Error((
                RateLimitReached.into(),
                error!(RateLimitReached => "you reached the rate limit, please wait `{}s` before your next request", time_to_wait),
            ));
        }

        timestamps.push((now.timestamp(), now.timestamp_subsec_nanos()));
        let timestamps = match serde_json::to_string(&timestamps) {
            Ok(s) => s,
            Err(e) => return Outcome::Error((InternalServer.into(), error!(InternalServer => e))),
        };

        match rate_limiter.set(key, timestamps) {
            Ok(()) => Outcome::Success(Self),
            Err(e) => Outcome::Error((InternalServer.into(), error!(InternalServer => e))),
        }
    }
}

pub struct SlidingWindow {
    request_num: usize,
    duration: Duration,
}

impl SlidingWindow {
    /// Create a sliding window of `request_num` requests over a given `duration`.
    pub fn new(request_num: usize, duration: Duration) -> Self {
        Self {
            request_num,
            duration,
        }
    }
}

impl Default for SlidingWindow {
    fn default() -> Self {
        Self::new(120, Duration::seconds(10))
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
