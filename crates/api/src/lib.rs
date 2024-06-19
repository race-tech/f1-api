use application::graphql::{Query, ServiceSchema};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    routing::{get, post},
    Extension, Router,
};

use infrastructure::config::{Config, MiddlewareConfig};
use shared::error::Result;

#[cfg(test)]
mod tests;

pub struct Api {
    port: u16,
    router: Router,
}

impl Api {
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

impl TryFrom<Config> for Api {
    type Error = shared::error::Error;

    fn try_from(config: Config) -> std::prelude::v1::Result<Self, Self::Error> {
        Ok(Self {
            port: config.port.unwrap_or(8000),
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
    let schema = schema(config)?;

    let api_routes = Router::new();

    let builder = ServiceBuilder {
        config,
        router: api_routes,
    };

    let modular_router = builder.middlewares()?;

    let router = Router::new()
        .route("/", post(graphql_handler))
        .layer(Extension(schema));

    Ok(modular_router.merge(router))
}

fn schema(config: &Config) -> Result<Schema<Query, EmptyMutation, EmptySubscription>> {
    let pool = infrastructure::ConnectionPool::try_from(config)?;
    Ok(Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish())
}

struct ServiceBuilder<'c> {
    config: &'c Config,
    router: Router,
}

impl<'c> ServiceBuilder<'c> {
    fn middlewares(self) -> Result<Router> {
        if let Some(middlewares) = &self.config.middlewares {
            let router = middlewares.iter().fold(self.router, |router, m| match m {
                MiddlewareConfig::Graphiql { enabled, route } if *enabled => {
                    let route = route.as_deref().unwrap_or("/");
                    router.route(route, get(graphiql))
                }
                _ => router,
            });
            Ok(router)
        } else {
            Ok(self.router)
        }
    }
}
