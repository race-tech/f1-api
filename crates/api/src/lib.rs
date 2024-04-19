use axum::Router;

mod handlers;
mod middlewares;
#[cfg(test)]
mod tests;

pub struct PurpleSector {
    port: u16,
    router: Router,
}

impl PurpleSector {
    pub fn new(port: u16) -> PurpleSector {
        Self {
            port,
            router: router(),
        }
    }

    pub async fn serve(self) {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .await
            .unwrap();

        axum::serve(listener, self.router).await.unwrap();
    }
}

pub fn router() -> Router {
    use axum::routing::get;

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
        .route("/status", get(handlers::status::status))
        .with_state(infrastructure::ConnectionPool::try_new().unwrap());

    Router::new().nest("/api/:series", api_routes)
}
