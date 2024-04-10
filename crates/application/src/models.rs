use mysql::prelude::*;

#[derive(FromRow, Debug)]
pub struct Circuits {
    #[mysql(rename = "circuitId")]
    circuit_id: i32,
    #[mysql(rename = "circuitRef")]
    circuit_ref: String,
    name: String,
    location: Option<String>,
    country: Option<String>,
    lat: Option<f32>,
    lng: Option<f32>,
    alt: Option<i32>,
    url: String,
}
