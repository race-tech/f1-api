use dotenvy::dotenv;
use sea_orm::ConnectOptions;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let opt = database_options();

    rocket::build()
        .mount("/api", api_lib::handlers::handlers())
        .manage(sea_orm::Database::connect(opt).await.unwrap())
        .launch()
        .await?;

    Ok(())
}

fn database_options() -> ConnectOptions {
    let database_url = std::env::var("DATABASE_URL").unwrap();

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100).min_connections(10);
    opt
}
