use chrono::{DateTime, Duration, Utc};
use redis::Commands;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{uri, Data, Request};

use infrastructure::ConnectionPool;

use crate::fallbacks::rocket_uri_macro_rate_limiter_fallback;

const RATE_LIMITER_KEY_PREFIX: &str = "RATE_LIMITER_";

pub struct RateLimiter;

#[rocket::async_trait]
impl Fairing for RateLimiter {
    fn info(&self) -> Info {
        Info {
            name: "RateLimiter",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
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
                req.set_uri(uri!("/fallback", rate_limiter_fallback(Some(ip_header), _)));
                return;
            }
        };

        let key = format!("{}{}", RATE_LIMITER_KEY_PREFIX, ip_addr);
        let now = Utc::now();
        let timestamps: String = match rate_limiter.get(&key) {
            Ok(res) => res,
            Err(_) => {
                rate_limiter
                    .set::<String, String, ()>(
                        key,
                        serde_json::to_string(&[(now.timestamp(), now.timestamp_subsec_nanos())])
                            .unwrap(),
                    )
                    .unwrap();
                return;
            }
        };

        let window_validation = |date| now - date > window.duration;
        let mut timestamps = cleanup(
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
            req.set_uri(uri!(
                "/fallback",
                rate_limiter_fallback(_, Some(time_to_wait))
            ));
            return;
        }

        timestamps.push((now.timestamp(), now.timestamp_subsec_nanos()));
        rate_limiter
            .set::<String, String, ()>(key, serde_json::to_string(&timestamps).unwrap())
            .unwrap();
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
