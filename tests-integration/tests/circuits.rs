use rocket::http::{Header, Status};

use shared::prelude::*;

mod common;

#[test]
fn test_get_circuit() {
    let client = common::setup();

    let expected_circuit = Circuit {
        circuit_ref: "spa".to_string(),
        name: "Circuit de Spa-Francorchamps".to_string(),
        location: Some("Spa".to_string()),
        country: Some("Belgium".to_string()),
        lat: Some(50.4372),
        lng: Some(5.97139),
        alt: Some(401),
        url: "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps".to_string(),
    };

    let mut req = client.get("/api/f1/circuits?circuit_ref=spa");
    req.add_header(Header::new("x-real-ip", "127.0.0.1"));

    let resp = req.dispatch();
    assert_eq!(resp.status(), Status::Ok);
    let json = resp.into_json::<Response<Circuit>>().unwrap();

    assert_eq!(json.series, Series::F1);
    assert_eq!(json.pagination, None);
    assert_eq!(json.data, expected_circuit);
}
