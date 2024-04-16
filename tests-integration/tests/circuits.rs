use rocket::http::Status;

use shared::prelude::*;

pub mod common;

struct TestCircuit {
    uri: &'static str,
    series: Series,
    pagination: Option<Pagination>,
    expected: &'static [StaticCircuit<'static>],
}

fn test_circuits_ok(test_circuit: TestCircuit) {
    let client = common::setup();

    let resp = common::get(&client, test_circuit.uri);
    assert_eq!(resp.status(), Status::Ok);
    let json = resp.into_json::<Response<Vec<Circuit>>>().unwrap();

    assert_eq!(json.series, test_circuit.series);
    assert_eq!(json.pagination, test_circuit.pagination);
    assert_eq!(json.data.len(), test_circuit.expected.len());

    json.data
        .iter()
        .take(test_circuit.expected.len())
        .zip(test_circuit.expected)
        .for_each(|(l, r)| assert_eq!(r, l));
}

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
fn test_get_circuits_by_driver_ref() {
    let test = TestCircuit {
        uri: "/api/f1/circuits?driver_ref=leclerc",
        series: Series::F1,
        pagination: Some(Pagination {
            limit: 30,
            page: 1,
            max_page: 2,
            total: 31,
        }),
        expected: &LECLERC_CIRCUITS,
    };

    test_circuits_ok(test);
}

#[test]
fn test_get_circuits_by_driver_ref_and_win() {
    let test = TestCircuit {
        uri: "/api/f1/circuits?result=1&driver_ref=leclerc",
        series: Series::F1,
        pagination: Some(Pagination {
            limit: 30,
            page: 1,
            max_page: 1,
            total: 5,
        }),
        expected: &LECLERC_CIRCUITS_WINS,
    };

    test_circuits_ok(test);
}

#[test]
fn test_get_circuits_by_driver_ref_and_win_and_pole() {
    let test = TestCircuit {
        uri: "/api/f1/circuits?grid=1&result=1&driver_ref=leclerc",
        series: Series::F1,
        pagination: Some(Pagination {
            limit: 30,
            page: 1,
            max_page: 1,
            total: 4,
        }),
        expected: &LECLERC_CIRCUITS_WINS_AND_POLE,
    };

    test_circuits_ok(test);
}

#[test]
fn test_get_circuits_by_constructor_ref() {
    let test = TestCircuit {
        uri: "/api/f1/circuits?constructor_ref=ferrari",
        series: Series::F1,
        pagination: Some(Pagination {
            limit: 30,
            page: 1,
            max_page: 3,
            total: 76,
        }),
        expected: &FERRARI_CIRCUITS,
    };

    test_circuits_ok(test);
}

#[derive(Debug)]
struct StaticCircuit<'a> {
    circuit_ref: &'a str,
    name: &'a str,
    location: Option<&'a str>,
    country: Option<&'a str>,
    lat: Option<f32>,
    lng: Option<f32>,
    alt: Option<i32>,
    url: &'a str,
}

impl PartialEq<Circuit> for StaticCircuit<'_> {
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

impl PartialEq<&Circuit> for StaticCircuit<'_> {
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
        [$($circuits),*, StaticCircuit {
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
        circuits_from_json!(@internal [$($circuits),*, StaticCircuit {
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
        circuits_from_json!(@internal [StaticCircuit {
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

const SPA: StaticCircuit = StaticCircuit {
    circuit_ref: "spa",
    name: "Circuit de Spa-Francorchamps",
    location: Some("Spa"),
    country: Some("Belgium"),
    lat: Some(50.4372),
    lng: Some(5.97139),
    alt: Some(401),
    url: "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps",
};

const LECLERC_CIRCUITS: [StaticCircuit; 30] = circuits_from_json![
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

const LECLERC_CIRCUITS_WINS: [StaticCircuit; 5] = circuits_from_json![
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
        "circuit_ref": "spa",
        "name": "Circuit de Spa-Francorchamps",
        "location": "Spa",
        "country": "Belgium",
        "lat": 50.4372,
        "lng": 5.97139,
        "alt": 401,
        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
    }
];

const LECLERC_CIRCUITS_WINS_AND_POLE: [StaticCircuit; 4] = circuits_from_json![
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
        "circuit_ref": "spa",
        "name": "Circuit de Spa-Francorchamps",
        "location": "Spa",
        "country": "Belgium",
        "lat": 50.4372,
        "lng": 5.97139,
        "alt": 401,
        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
    }
];

const FERRARI_CIRCUITS: [StaticCircuit; 30] = circuits_from_json![
    {
        "circuit_ref": "adelaide",
        "name": "Adelaide Street Circuit",
        "location": "Adelaide",
        "country": "Australia",
        "lat": -34.9272,
        "lng": 138.617,
        "alt": 58,
        "url": "http://en.wikipedia.org/wiki/Adelaide_Street_Circuit"
    },
    {
        "circuit_ref": "ain-diab",
        "name": "Ain Diab",
        "location": "Casablanca",
        "country": "Morocco",
        "lat": 33.5786,
        "lng": -7.6875,
        "alt": 19,
        "url": "http://en.wikipedia.org/wiki/Ain-Diab_Circuit"
    },
    {
        "circuit_ref": "aintree",
        "name": "Aintree",
        "location": "Liverpool",
        "country": "UK",
        "lat": 53.4769,
        "lng": -2.94056,
        "alt": 20,
        "url": "http://en.wikipedia.org/wiki/Aintree_Motor_Racing_Circuit"
    },
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
        "circuit_ref": "anderstorp",
        "name": "Scandinavian Raceway",
        "location": "Anderstorp",
        "country": "Sweden",
        "lat": 57.2653,
        "lng": 13.6042,
        "alt": 153,
        "url": "http://en.wikipedia.org/wiki/Scandinavian_Raceway"
    },
    {
        "circuit_ref": "avus",
        "name": "AVUS",
        "location": "Berlin",
        "country": "Germany",
        "lat": 52.4806,
        "lng": 13.2514,
        "alt": 53,
        "url": "http://en.wikipedia.org/wiki/AVUS"
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
        "circuit_ref": "boavista",
        "name": "Circuito da Boavista",
        "location": "Oporto",
        "country": "Portugal",
        "lat": 41.1705,
        "lng": -8.67325,
        "alt": 28,
        "url": "http://en.wikipedia.org/wiki/Circuito_da_Boavista"
    },
    {
        "circuit_ref": "brands_hatch",
        "name": "Brands Hatch",
        "location": "Kent",
        "country": "UK",
        "lat": 51.3569,
        "lng": 0.263056,
        "alt": 145,
        "url": "http://en.wikipedia.org/wiki/Brands_Hatch"
    },
    {
        "circuit_ref": "bremgarten",
        "name": "Circuit Bremgarten",
        "location": "Bern",
        "country": "Switzerland",
        "lat": 46.9589,
        "lng": 7.40194,
        "alt": 551,
        "url": "http://en.wikipedia.org/wiki/Circuit_Bremgarten"
    },
    {
        "circuit_ref": "buddh",
        "name": "Buddh International Circuit",
        "location": "Uttar Pradesh",
        "country": "India",
        "lat": 28.3487,
        "lng": 77.5331,
        "alt": 194,
        "url": "http://en.wikipedia.org/wiki/Buddh_International_Circuit"
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
        "circuit_ref": "charade",
        "name": "Charade Circuit",
        "location": "Clermont-Ferrand",
        "country": "France",
        "lat": 45.7472,
        "lng": 3.03889,
        "alt": 790,
        "url": "http://en.wikipedia.org/wiki/Charade_Circuit"
    },
    {
        "circuit_ref": "dallas",
        "name": "Fair Park",
        "location": "Dallas",
        "country": "USA",
        "lat": 32.7774,
        "lng": -96.7587,
        "alt": 139,
        "url": "http://en.wikipedia.org/wiki/Fair_Park"
    },
    {
        "circuit_ref": "detroit",
        "name": "Detroit Street Circuit",
        "location": "Detroit",
        "country": "USA",
        "lat": 42.3298,
        "lng": -83.0401,
        "alt": 177,
        "url": "http://en.wikipedia.org/wiki/Detroit_street_circuit"
    },
    {
        "circuit_ref": "dijon",
        "name": "Dijon-Prenois",
        "location": "Dijon",
        "country": "France",
        "lat": 47.3625,
        "lng": 4.89913,
        "alt": 484,
        "url": "http://en.wikipedia.org/wiki/Dijon-Prenois"
    },
    {
        "circuit_ref": "donington",
        "name": "Donington Park",
        "location": "Castle Donington",
        "country": "UK",
        "lat": 52.8306,
        "lng": -1.37528,
        "alt": 88,
        "url": "http://en.wikipedia.org/wiki/Donington_Park"
    },
    {
        "circuit_ref": "essarts",
        "name": "Rouen-Les-Essarts",
        "location": "Rouen",
        "country": "France",
        "lat": 49.3306,
        "lng": 1.00458,
        "alt": 81,
        "url": "http://en.wikipedia.org/wiki/Rouen-Les-Essarts"
    },
    {
        "circuit_ref": "estoril",
        "name": "Autódromo do Estoril",
        "location": "Estoril",
        "country": "Portugal",
        "lat": 38.7506,
        "lng": -9.39417,
        "alt": 130,
        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_do_Estoril"
    },
    {
        "circuit_ref": "fuji",
        "name": "Fuji Speedway",
        "location": "Oyama",
        "country": "Japan",
        "lat": 35.3717,
        "lng": 138.927,
        "alt": 583,
        "url": "http://en.wikipedia.org/wiki/Fuji_Speedway"
    },
    {
        "circuit_ref": "galvez",
        "name": "Autódromo Juan y Oscar Gálvez",
        "location": "Buenos Aires",
        "country": "Argentina",
        "lat": -34.6943,
        "lng": -58.4593,
        "alt": 8,
        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Oscar_Alfredo_G%C3%A1lvez"
    },
    {
        "circuit_ref": "george",
        "name": "Prince George Circuit",
        "location": "Eastern Cape Province",
        "country": "South Africa",
        "lat": -33.0486,
        "lng": 27.8736,
        "alt": 15,
        "url": "http://en.wikipedia.org/wiki/Prince_George_Circuit"
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
        "circuit_ref": "indianapolis",
        "name": "Indianapolis Motor Speedway",
        "location": "Indianapolis",
        "country": "USA",
        "lat": 39.795,
        "lng": -86.2347,
        "alt": 223,
        "url": "http://en.wikipedia.org/wiki/Indianapolis_Motor_Speedway"
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
    }
];
