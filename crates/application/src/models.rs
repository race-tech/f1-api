use mysql::prelude::*;
use serde::Serialize;

#[derive(FromRow, Debug, Serialize)]
pub struct Circuits {
    #[mysql(rename = "circuitId")]
    circuit_id: i32,
    #[mysql(rename = "circuitRef")]
    circuit_ref: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lat: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lng: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alt: Option<i32>,
    url: String,
}
