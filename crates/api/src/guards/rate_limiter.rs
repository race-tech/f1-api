use chrono::{DateTime, Duration, Utc};
use redis::Commands;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use shared::error;

use infrastructure::ConnectionPool;
use shared::error::Error;

const RATE_LIMITER_KEY_PREFIX: &str = "RATE_LIMITER_";

pub struct RateLimiter;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RateLimiter {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // SAFETY: This values should always be defined
        let ip_header = req.rocket().config().ip_header.as_ref().unwrap();
        let rate_limiter = req.rocket().state::<ConnectionPool>().unwrap();
        let window = req.rocket().state::<SlidingWindow>().unwrap();
        let rate_limiter = &mut rate_limiter.cache.get().unwrap();

        let ip_addr = match req.real_ip() {
            Some(ip_addr) => ip_addr,
            None => {
                return request::Outcome::Error((
                    Status::BadRequest,
                    error!(IpHeaderNotFound => "ip header not found, expected to find it under the `{}` header", ip_header),
                ))
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
                return request::Outcome::Success(Self);
            }
        };

        println!("cache retrieved: {:?}", timestamps);
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
            request::Outcome::Error((
                Status::TooManyRequests,
                error!(RateLimitReached => "you reached the rate limit, please wait `{}s` before your next request", (window.duration - (now - first)).num_seconds()),
            ))
        } else {
            timestamps.push((now.timestamp(), now.timestamp_subsec_nanos()));
            rate_limiter
                .set::<String, String, ()>(key, serde_json::to_string(&timestamps).unwrap())
                .unwrap();

            request::Outcome::Success(Self)
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

#[cfg(test)]
mod test {
    use chrono::{DateTime, TimeZone, Utc};

    #[test]
    fn test() {
        let dt: DateTime<Utc> = Utc.with_ymd_and_hms(2015, 5, 15, 0, 0, 0).unwrap();
        assert_eq!(
            DateTime::from_timestamp(dt.timestamp(), dt.timestamp_subsec_nanos()).unwrap(),
            dt
        );

        let now = Utc::now();
        assert!(DateTime::from_timestamp(now.timestamp(), now.timestamp_subsec_nanos()).is_some());
    }
}
