use rocket::http::Status;

use shared::prelude::*;

pub mod common;

#[test]
fn test_get_circuit() {
    let client = common::setup();

    let resp = common::get(&client, "/api/f1/circuits?circuit_ref=spa");
    assert_eq!(resp.status(), Status::Ok);
    let json = resp.into_json::<Response<Circuit>>().unwrap();

    assert_eq!(json.series, Series::F1);
    assert_eq!(json.pagination, None);
    assert_eq!(SPA, json.data);
}

#[test]
fn test_get_circuits() {
    let client = common::setup();

    let expected_pagination = Pagination {
        limit: 30,
        page: 1,
        max_page: 2,
        total: 31,
    };

    let resp = common::get(&client, "/api/f1/circuits?driver_ref=leclerc");
    assert_eq!(resp.status(), Status::Ok);
    let json = resp.into_json::<Response<Vec<Circuit>>>().unwrap();

    assert_eq!(json.series, Series::F1);
    assert_eq!(json.pagination, Some(expected_pagination));
    assert_eq!(json.data.len(), 30);

    json.data
        .iter()
        .take(30)
        .zip(LECLERC_CIRCUITS)
        .for_each(|(l, r)| assert_eq!(r, l));
}

#[derive(Debug)]
struct RefCircuit<'a> {
    circuit_ref: &'a str,
    name: &'a str,
    location: Option<&'a str>,
    country: Option<&'a str>,
    lat: Option<f32>,
    lng: Option<f32>,
    alt: Option<i32>,
    url: &'a str,
}

impl PartialEq<Circuit> for RefCircuit<'_> {
    fn eq(&self, other: &Circuit) -> bool {
        self.circuit_ref.eq(&other.circuit_ref)
            && self.name.eq(&other.name)
            && self.location.eq(&other.location.as_deref())
            && self.country.eq(&other.country.as_deref())
            && self.alt.eq(&other.alt)
            && self.lat.eq(&other.lat)
            && self.lng.eq(&other.lng)
            && self.url.eq(&other.url)
    }
}

impl PartialEq<&Circuit> for RefCircuit<'_> {
    fn eq(&self, other: &&Circuit) -> bool {
        self.circuit_ref.eq(&other.circuit_ref)
            && self.name.eq(&other.name)
            && self.location.eq(&other.location.as_deref())
            && self.country.eq(&other.country.as_deref())
            && self.alt.eq(&other.alt)
            && self.lat.eq(&other.lat)
            && self.lng.eq(&other.lng)
            && self.url.eq(&other.url)
    }
}

macro_rules! circuits_from_json {
    (@internal [$($circuits:expr),*]; {
        "circuit_ref": $ref:literal,
        "name": $name:literal,
        "location": $location:literal,
        "country": $country:literal,
        "lat": $lat:expr,
        "lng": $lng:expr,
        "alt": $alt:expr,
        "url": $url:literal
    }) => {
        [$($circuits),*, RefCircuit {
            circuit_ref: $ref,
            name: $name,
            location: Some($location),
            country: Some($country),
            lat: Some($lat),
            lng: Some($lng),
            alt: Some($alt),
            url: $url,
        }]
    };
    (@internal [$($circuits:expr),*]; {
        "circuit_ref": $ref:literal,
        "name": $name:literal,
        "location": $location:literal,
        "country": $country:literal,
        "lat": $lat:expr,
        "lng": $lng:expr,
        "alt": $alt:expr,
        "url": $url:literal
    }, $($tt:tt)*) => {
        circuits_from_json!(@internal [$($circuits),*, RefCircuit {
            circuit_ref: $ref,
            name: $name,
            location: Some($location),
            country: Some($country),
            lat: Some($lat),
            lng: Some($lng),
            alt: Some($alt),
            url: $url,
        }]; $($tt)*)
    };
    ({
        "circuit_ref": $ref:literal,
        "name": $name:literal,
        "location": $location:literal,
        "country": $country:literal,
        "lat": $lat:expr,
        "lng": $lng:expr,
        "alt": $alt:expr,
        "url": $url:literal
    }, $($tt:tt)*) => {
        circuits_from_json!(@internal [RefCircuit {
            circuit_ref: $ref,
            name: $name,
            location: Some($location),
            country: Some($country),
            lat: Some($lat),
            lng: Some($lng),
            alt: Some($alt),
            url: $url,
        }]; $($tt)*)
    };
}

const SPA: RefCircuit = RefCircuit {
    circuit_ref: "spa",
    name: "Circuit de Spa-Francorchamps",
    location: Some("Spa"),
    country: Some("Belgium"),
    lat: Some(50.4372),
    lng: Some(5.97139),
    alt: Some(401),
    url: "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps",
};

const LECLERC_CIRCUITS: [RefCircuit; 30] = circuits_from_json![
    {
        "circuit_ref": "albert_park",
        "name": "Albert Park Grand Prix Circuit",
        "location": "Melbourne",
        "country": "Australia",
        "lat": -37.8497,
        "lng": 144.968,
        "alt": 10,
        "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit"
    },
    {
        "circuit_ref": "americas",
        "name": "Circuit of the Americas",
        "location": "Austin",
        "country": "USA",
        "lat": 30.1328,
        "lng": -97.6411,
        "alt": 161,
        "url": "http://en.wikipedia.org/wiki/Circuit_of_the_Americas"
    },
    {
        "circuit_ref": "bahrain",
        "name": "Bahrain International Circuit",
        "location": "Sakhir",
        "country": "Bahrain",
        "lat": 26.0325,
        "lng": 50.5106,
        "alt": 7,
        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
    },
    {
        "circuit_ref": "baku",
        "name": "Baku City Circuit",
        "location": "Baku",
        "country": "Azerbaijan",
        "lat": 40.3725,
        "lng": 49.8533,
        "alt": -7,
        "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit"
    },
    {
        "circuit_ref": "catalunya",
        "name": "Circuit de Barcelona-Catalunya",
        "location": "Montmeló",
        "country": "Spain",
        "lat": 41.57,
        "lng": 2.26111,
        "alt": 109,
        "url": "http://en.wikipedia.org/wiki/Circuit_de_Barcelona-Catalunya"
    },
    {
        "circuit_ref": "hockenheimring",
        "name": "Hockenheimring",
        "location": "Hockenheim",
        "country": "Germany",
        "lat": 49.3278,
        "lng": 8.56583,
        "alt": 103,
        "url": "http://en.wikipedia.org/wiki/Hockenheimring"
    },
    {
        "circuit_ref": "hungaroring",
        "name": "Hungaroring",
        "location": "Budapest",
        "country": "Hungary",
        "lat": 47.5789,
        "lng": 19.2486,
        "alt": 264,
        "url": "http://en.wikipedia.org/wiki/Hungaroring"
    },
    {
        "circuit_ref": "imola",
        "name": "Autodromo Enzo e Dino Ferrari",
        "location": "Imola",
        "country": "Italy",
        "lat": 44.3439,
        "lng": 11.7167,
        "alt": 37,
        "url": "http://en.wikipedia.org/wiki/Autodromo_Enzo_e_Dino_Ferrari"
    },
    {
        "circuit_ref": "interlagos",
        "name": "Autódromo José Carlos Pace",
        "location": "São Paulo",
        "country": "Brazil",
        "lat": -23.7036,
        "lng": -46.6997,
        "alt": 785,
        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Jos%C3%A9_Carlos_Pace"
    },
    {
        "circuit_ref": "istanbul",
        "name": "Istanbul Park",
        "location": "Istanbul",
        "country": "Turkey",
        "lat": 40.9517,
        "lng": 29.405,
        "alt": 130,
        "url": "http://en.wikipedia.org/wiki/Istanbul_Park"
    },
    {
        "circuit_ref": "jeddah",
        "name": "Jeddah Corniche Circuit",
        "location": "Jeddah",
        "country": "Saudi Arabia",
        "lat": 21.6319,
        "lng": 39.1044,
        "alt": 15,
        "url": "http://en.wikipedia.org/wiki/Jeddah_Street_Circuit"
    },
    {
        "circuit_ref": "losail",
        "name": "Losail International Circuit",
        "location": "Al Daayen",
        "country": "Qatar",
        "lat": 25.49,
        "lng": 51.4542,
        "alt": 12,
        "url": "http://en.wikipedia.org/wiki/Losail_International_Circuit"
    },
    {
        "circuit_ref": "marina_bay",
        "name": "Marina Bay Street Circuit",
        "location": "Marina Bay",
        "country": "Singapore",
        "lat": 1.2914,
        "lng": 103.864,
        "alt": 18,
        "url": "http://en.wikipedia.org/wiki/Marina_Bay_Street_Circuit"
    },
    {
        "circuit_ref": "miami",
        "name": "Miami International Autodrome",
        "location": "Miami",
        "country": "USA",
        "lat": 25.9581,
        "lng": -80.2389,
        "alt": 0,
        "url": "http://en.wikipedia.org/wiki/Miami_International_Autodrome"
    },
    {
        "circuit_ref": "monaco",
        "name": "Circuit de Monaco",
        "location": "Monte-Carlo",
        "country": "Monaco",
        "lat": 43.7347,
        "lng": 7.42056,
        "alt": 7,
        "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco"
    },
    {
        "circuit_ref": "monza",
        "name": "Autodromo Nazionale di Monza",
        "location": "Monza",
        "country": "Italy",
        "lat": 45.6156,
        "lng": 9.28111,
        "alt": 162,
        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
    },
    {
        "circuit_ref": "mugello",
        "name": "Autodromo Internazionale del Mugello",
        "location": "Mugello",
        "country": "Italy",
        "lat": 43.9975,
        "lng": 11.3719,
        "alt": 255,
        "url": "http://en.wikipedia.org/wiki/Mugello_Circuit"
    },
    {
        "circuit_ref": "nurburgring",
        "name": "Nürburgring",
        "location": "Nürburg",
        "country": "Germany",
        "lat": 50.3356,
        "lng": 6.9475,
        "alt": 578,
        "url": "http://en.wikipedia.org/wiki/N%C3%BCrburgring"
    },
    {
        "circuit_ref": "portimao",
        "name": "Autódromo Internacional do Algarve",
        "location": "Portimão",
        "country": "Portugal",
        "lat": 37.227,
        "lng": -8.6267,
        "alt": 108,
        "url": "http://en.wikipedia.org/wiki/Algarve_International_Circuit"
    },
    {
        "circuit_ref": "red_bull_ring",
        "name": "Red Bull Ring",
        "location": "Spielberg",
        "country": "Austria",
        "lat": 47.2197,
        "lng": 14.7647,
        "alt": 678,
        "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring"
    },
    {
        "circuit_ref": "ricard",
        "name": "Circuit Paul Ricard",
        "location": "Le Castellet",
        "country": "France",
        "lat": 43.2506,
        "lng": 5.79167,
        "alt": 432,
        "url": "http://en.wikipedia.org/wiki/Paul_Ricard_Circuit"
    },
    {
        "circuit_ref": "rodriguez",
        "name": "Autódromo Hermanos Rodríguez",
        "location": "Mexico City",
        "country": "Mexico",
        "lat": 19.4042,
        "lng": -99.0907,
        "alt": 2227,
        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Hermanos_Rodr%C3%ADguez"
    },
    {
        "circuit_ref": "shanghai",
        "name": "Shanghai International Circuit",
        "location": "Shanghai",
        "country": "China",
        "lat": 31.3389,
        "lng": 121.22,
        "alt": 5,
        "url": "http://en.wikipedia.org/wiki/Shanghai_International_Circuit"
    },
    {
        "circuit_ref": "silverstone",
        "name": "Silverstone Circuit",
        "location": "Silverstone",
        "country": "UK",
        "lat": 52.0786,
        "lng": -1.01694,
        "alt": 153,
        "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit"
    },
    {
        "circuit_ref": "sochi",
        "name": "Sochi Autodrom",
        "location": "Sochi",
        "country": "Russia",
        "lat": 43.4057,
        "lng": 39.9578,
        "alt": 2,
        "url": "http://en.wikipedia.org/wiki/Sochi_Autodrom"
    },
    {
        "circuit_ref": "spa",
        "name": "Circuit de Spa-Francorchamps",
        "location": "Spa",
        "country": "Belgium",
        "lat": 50.4372,
        "lng": 5.97139,
        "alt": 401,
        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
    },
    {
        "circuit_ref": "suzuka",
        "name": "Suzuka Circuit",
        "location": "Suzuka",
        "country": "Japan",
        "lat": 34.8431,
        "lng": 136.541,
        "alt": 45,
        "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit"
    },
    {
        "circuit_ref": "vegas",
        "name": "Las Vegas Strip Street Circuit",
        "location": "Las Vegas",
        "country": "United States",
        "lat": 36.1147,
        "lng": -115.173,
        "alt": 642,
        "url": "https://en.wikipedia.org/wiki/Las_Vegas_Grand_Prix#Circuit"
    },
    {
        "circuit_ref": "villeneuve",
        "name": "Circuit Gilles Villeneuve",
        "location": "Montreal",
        "country": "Canada",
        "lat": 45.5,
        "lng": -73.5228,
        "alt": 13,
        "url": "http://en.wikipedia.org/wiki/Circuit_Gilles_Villeneuve"
    },
    {
        "circuit_ref": "yas_marina",
        "name": "Yas Marina Circuit",
        "location": "Abu Dhabi",
        "country": "UAE",
        "lat": 24.4672,
        "lng": 54.6031,
        "alt": 3,
        "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit"
    }
];
