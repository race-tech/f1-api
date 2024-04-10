use rocket::serde::Serialize;

use super::prelude::*;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub data: T,

    #[serde(flatten)]
    pub pagination: Pagination,
    pub series: Series,
}
