#[macro_use]
extern crate rocket;

use dotenvy::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/api", api_lib::handlers::handlers())
        .manage(mysql::Pool::new("mysql://user:password@localhost:3306/f1db").unwrap())
}
