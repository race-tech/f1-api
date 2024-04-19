#[macro_use]
extern crate rocket;

use dotenvy::dotenv;

#[launch]
fn rocket() -> _ {
    logger::Logger::new()
        .init()
        .expect("cannot initialize the logger");

    match dotenv() {
        Ok(_) => log::info!("loadded `.env` file"),
        Err(e) => log::error!("cannot load `.env` file: {}", e),
    }

    api_lib::rocket_builder()
}
