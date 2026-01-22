use infrastructure::config::Config;

mod api;
mod query_root;

#[tokio::main]
async fn main() -> shared::error::Result<()> {
    logger::Logger::new()
        .init()
        .expect("cannot initialize the logger");

    let config = match Config::try_new() {
        Ok(config) => {
            log::info!("config successfully loaded: {:?}", config);
            config
        }
        Err(e) => {
            log::error!("cannot load config : {}", e);
            log::error!("aborting API launch");
            return Err(e);
        }
    };

    api::Api::try_new(config).await?.serve().await;

    Ok(())
}
