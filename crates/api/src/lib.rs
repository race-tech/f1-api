use application::graphql::{Query, ServiceSchema};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    middleware,
    response::{self, IntoResponse},
    Extension, Router,
};

use infrastructure::config::{Config, MiddlewareConfig};
use middlewares::cache::Cache;
use shared::error::Result;

use crate::middlewares::rate_limiter::RateLimiter;

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

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

async fn graphql_handler(
    Extension(schema): Extension<ServiceSchema>, // (2)
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into() // (3)
}

fn router(config: &Config) -> Result<Router> {
    use axum::routing::get;

    let pool = infrastructure::ConnectionPool::try_from(config)?;
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(pool.clone())
        .finish();

    let api_routes = Router::new();

    let builder = ServiceBuilder {
        config,
        router: api_routes,
    };

    let _api_routes = builder.middlewares()?;

    let router = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .layer(Extension(pool));

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
