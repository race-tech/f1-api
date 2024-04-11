use rocket::serde::Serialize;

use crate::parameters::Series;

#[derive(Debug, Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Pagination {
    pub limit: u64,
    pub page: u64,
    pub max_page: u64,
    pub total: u64,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub data: T,

    #[serde(flatten)]
    pub pagination: Pagination,
    pub series: Series,
}
