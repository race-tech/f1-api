use dotenvy::dotenv;

#[tokio::main]
async fn main() -> shared::error::Result<()> {
    logger::Logger::new()
        .init()
        .expect("cannot initialize the logger");

    match dotenv() {
        Ok(_) => log::info!("loadded `.env` file"),
        Err(e) => log::error!("cannot load `.env` file: {}", e),
    }

    api_lib::PurpleSector::new(8000).serve().await;
    Ok(())
}
