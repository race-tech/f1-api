use async_graphql::dynamic::Schema;
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{self, IntoResponse},
    routing::{get, post},
    Router,
};

use infrastructure::config::{Config, MiddlewareConfig};
use shared::error::Result;

pub struct Api {
    port: u16,
    router: Router,
}

impl Api {
    pub async fn try_new(config: Config) -> Result<Self> {
        Ok(Self {
            port: config.port.unwrap_or(8000),
            router: router(&config).await?,
        })
    }

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

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

async fn graphql_handler(schema: State<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn router(config: &Config) -> Result<Router> {
    let pool = infrastructure::create_database_conn_pool(config).await?;
    let schema = crate::query_root::schema(pool.into(), None, None).unwrap();

    let api_routes = Router::new();

    let builder = ServiceBuilder {
        config,
        router: api_routes,
    };

    let modular_router = builder.middlewares()?;

    let router = Router::new()
        .route("/", post(graphql_handler))
        .with_state(schema);

    Ok(modular_router.merge(router))
}

struct ServiceBuilder<'c> {
    config: &'c Config,
    router: Router,
}

impl ServiceBuilder<'_> {
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
