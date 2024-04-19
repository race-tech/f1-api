use rocket::{get, routes, Route};

use shared::{error, prelude::*};

#[get("/rate_limiter?<header_not_found>&<too_many_requests>")]
fn rate_limiter_fallback(
    header_not_found: Option<String>,
    too_many_requests: Option<i64>,
) -> Result<()> {
    if let Some(ip_header) = header_not_found {
        return Err(
            error!(IpHeaderNotFound => "ip header not found, expected to find it under the `{}` header", ip_header),
        );
    }

    if let Some(time_to_wait) = too_many_requests {
        return Err(
            error!(RateLimitReached => "you reached the rate limit, please wait `{}s` before your next request", time_to_wait),
        );
    }

    log::error!(
        "rate_limiter_fallback should not be called without any parameters, please open an issue"
    );
    Err(
        error!(InternalServerError => "rate_limiter_fallback should not be called without any parameters, please open an issue"),
    )
}

#[get("/internal_ressource")]
fn internal_ressource() -> Result<()> {
    Err(
        error!(InternalRessource => "this ressource is intended for internal purposes you can't access it"),
    )
}

pub fn handlers() -> Vec<Route> {
    routes![rate_limiter_fallback, internal_ressource]
}
