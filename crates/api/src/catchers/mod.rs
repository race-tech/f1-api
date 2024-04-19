use rocket::http::Status;
use rocket::{catch, catchers, Catcher, Request};

use shared::error;
use shared::error::Result;

#[catch(404)]
fn not_found(req: &Request) -> Result<()> {
    Err(error!(ResourceNotFound => "can't find the following resource: {}", req.uri()))
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request) -> Result<()> {
    log::error!(
        "an uncaught error occured : {} {} {}\n please open an issue",
        req.method(),
        req.uri(),
        status
    );
    Err(
        error!(InternalServer => "an uncaught error occured : {} {} {}\n please open an issue", req.method(), req.uri(), status),
    )
}

pub fn catchers() -> Vec<Catcher> {
    catchers![not_found, default_catcher]
}
