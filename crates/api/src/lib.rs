use axum::{middleware, Extension, Router};

use infrastructure::config::{Config, MiddlewareConfig};
use middlewares::cache::Cache;
use shared::error::Result;

use crate::middlewares::rate_limiter::RateLimiter;

mod handlers;
mod middlewares;
#[cfg(test)]
mod tests;

pub struct PurpleSector {
    port: u16,
    router: Router,
}

impl PurpleSector {
    pub async fn serve(self) {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .await
            .unwrap();

        axum::serve(
            listener,
            self.router
                .into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .await
        .unwrap();
    }
}

impl TryFrom<Config> for PurpleSector {
    type Error = shared::error::Error;

    fn try_from(config: Config) -> std::prelude::v1::Result<Self, Self::Error> {
        Ok(Self {
            port: 8000,
            router: router(&config)?,
        })
    }
}

fn router(config: &Config) -> Result<Router> {
    use axum::routing::get;

    let pool = infrastructure::ConnectionPool::try_from(config)?;

    let api_routes = Router::new()
        .route("/circuits", get(handlers::circuits::circuits))
        .route(
            "/constructors/standings",
            get(handlers::constructor_standings::constructor_standings),
        )
        .route("/constructors", get(handlers::constructors::constructors))
        .route(
            "/drivers/standings",
            get(handlers::driver_standings::driver_standings),
        )
        .route("/drivers", get(handlers::drivers::drivers))
        .route("/laps", get(handlers::laps::laps))
        .route("/races", get(handlers::races::races))
        .route("/pit-stops", get(handlers::pit_stops::pit_stops))
        .route("/seasons", get(handlers::seasons::seasons))
        .route("/status", get(handlers::status::status));

    let builder = ServiceBuilder {
        config,
        router: api_routes,
    };

    let api_routes = builder.middlewares()?.route_layer(Extension(pool));

    let router = Router::new().nest("/api/:series", api_routes);

    Ok(router)
}

struct ServiceBuilder<'c> {
    config: &'c Config,
    router: Router,
}

impl<'c> ServiceBuilder<'c> {
    fn middlewares(self) -> Result<Router> {
        if let Some(middlewares) = &self.config.middlewares {
            let router = middlewares.iter().fold(self.router, |router, m| match *m {
                MiddlewareConfig::RateLimiter {
                    enabled,
                    ty,
                    seconds,
                    requests,
                } if enabled => router.route_layer(middleware::from_fn_with_state(
                    RateLimiter::new(ty, requests, seconds),
                    middlewares::rate_limiter::mw_rate_limiter,
                )),
                MiddlewareConfig::Cache { enabled, ttl } if enabled => {
                    router.route_layer(middleware::from_fn_with_state(
                        Cache::new(ttl),
                        middlewares::cache::mw_cache_layer,
                    ))
                }
                _ => router,
            });
            Ok(router)
        } else {
            Ok(self.router)
        }
    }
}
