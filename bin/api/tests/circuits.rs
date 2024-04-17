use shared::prelude::*;

pub mod common;

#[test]
fn test_get_circuit() {
    common::Test::<'_, StaticCircuit, Circuit>::new(
        "/api/f1/circuits?circuit_ref=spa",
        Series::F1,
        SPA,
    )
    .test_ok();
}

#[test]
fn test_get_circuits_by_driver_ref() {
    common::Test::<'_, &[StaticCircuit], Vec<Circuit>>::new(
        "/api/f1/circuits?driver_ref=leclerc",
        Series::F1,
        &LECLERC_CIRCUITS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 2,
        total: 31,
    }))
    .test_ok();
}

#[test]
fn test_get_circuits_by_driver_ref_and_win() {
    common::Test::<'_, &[StaticCircuit], Vec<Circuit>>::new(
        "/api/f1/circuits?result=1&driver_ref=leclerc",
        Series::F1,
        &LECLERC_CIRCUITS_WINS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 1,
        total: 5,
    }))
    .test_ok();
}

#[test]
fn test_get_circuits_by_driver_ref_and_win_and_pole() {
    common::Test::<'_, &[StaticCircuit], Vec<Circuit>>::new(
        "/api/f1/circuits?grid=1&result=1&driver_ref=leclerc",
        Series::F1,
        &LECLERC_CIRCUITS_WINS_AND_POLE,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 1,
        total: 4,
    }))
    .test_ok();
}

#[test]
fn test_get_circuits_by_constructor_ref() {
    common::Test::<'_, &[StaticCircuit], Vec<Circuit>>::new(
        "/api/f1/circuits?constructor_ref=ferrari",
        Series::F1,
        &FERRARI_CIRCUITS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 3,
        total: 76,
    }))
    .test_ok();
}

#[test]
fn test_get_circuits_by_constructor_ref_and_page() {
    common::Test::<'_, &[StaticCircuit], Vec<Circuit>>::new(
        "/api/f1/circuits?constructor_ref=ferrari&page=2",
        Series::F1,
        &FERRARI_PAGE_2_CIRCUITS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 2,
        max_page: 3,
        total: 76,
    }))
    .test_ok();
}

#[test]
fn test_get_circuits_by_year_and_round() {
    common::Test::<'_, &[StaticCircuit], Vec<Circuit>>::new(
        "/api/f1/circuits?year=2023&round=22",
        Series::F1,
        &circuits_from_json![{
            "circuit_ref": "yas_marina",
            "name": "Yas Marina Circuit",
            "location": "Abu Dhabi",
            "country": "UAE",
            "lat": 24.4672,
            "lng": 54.6031,
            "alt": 3,
            "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit"
        }],
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 1,
        total: 1,
    }))
    .test_ok();
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

macro_rules! __circuits_impl {
    (@internal [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@internal [$($expr:expr),*]; $(,)?{
        "circuit_ref": $ref:literal,
        "name": $name:literal,
        "location": $location:literal,
        "country": $country:literal,
        "lat": $lat:expr,
        "lng": $lng:expr,
        "alt": $alt:expr,
        "url": $url:literal
    } $($tt:tt)*) => {
        __circuits_impl!(@internal [$($expr,)* StaticCircuit {
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

macro_rules! circuits_from_json {
    ($($tt:tt)*) => {
        __circuits_impl!(@internal []; $($tt)*)
    };
}

use __circuits_impl;
use circuits_from_json;

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

const FERRARI_PAGE_2_CIRCUITS: [StaticCircuit; 30] = circuits_from_json![
    {
        "circuit_ref": "jacarepagua",
        "name": "Autódromo Internacional Nelson Piquet",
        "location": "Rio de Janeiro",
        "country": "Brazil",
        "lat": -22.9756,
        "lng": -43.395,
        "alt": 1126,
        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Internacional_Nelson_Piquet"
    },
    {
        "circuit_ref": "jarama",
        "name": "Jarama",
        "location": "Madrid",
        "country": "Spain",
        "lat": 40.6171,
        "lng": -3.58558,
        "alt": 609,
        "url": "http://en.wikipedia.org/wiki/Circuito_Permanente_Del_Jarama"
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
        "circuit_ref": "jerez",
        "name": "Circuito de Jerez",
        "location": "Jerez de la Frontera",
        "country": "Spain",
        "lat": 36.7083,
        "lng": -6.03417,
        "alt": 37,
        "url": "http://en.wikipedia.org/wiki/Circuito_Permanente_de_Jerez"
    },
    {
        "circuit_ref": "kyalami",
        "name": "Kyalami",
        "location": "Midrand",
        "country": "South Africa",
        "lat": -25.9894,
        "lng": 28.0767,
        "alt": 1460,
        "url": "http://en.wikipedia.org/wiki/Kyalami"
    },
    {
        "circuit_ref": "las_vegas",
        "name": "Las Vegas Street Circuit",
        "location": "Nevada",
        "country": "USA",
        "lat": 36.1162,
        "lng": -115.174,
        "alt": 639,
        "url": "http://en.wikipedia.org/wiki/Las_Vegas_Street_Circuit"
    },
    {
        "circuit_ref": "lemans",
        "name": "Le Mans",
        "location": "Le Mans",
        "country": "France",
        "lat": 47.95,
        "lng": 0.224231,
        "alt": 67,
        "url": "http://en.wikipedia.org/wiki/Circuit_de_la_Sarthe#Bugatti_Circuit"
    },
    {
        "circuit_ref": "long_beach",
        "name": "Long Beach",
        "location": "California",
        "country": "USA",
        "lat": 33.7651,
        "lng": -118.189,
        "alt": 12,
        "url": "http://en.wikipedia.org/wiki/Long_Beach,_California"
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
        "circuit_ref": "magny_cours",
        "name": "Circuit de Nevers Magny-Cours",
        "location": "Magny Cours",
        "country": "France",
        "lat": 46.8642,
        "lng": 3.16361,
        "alt": 228,
        "url": "http://en.wikipedia.org/wiki/Circuit_de_Nevers_Magny-Cours"
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
        "circuit_ref": "monsanto",
        "name": "Monsanto Park Circuit",
        "location": "Lisbon",
        "country": "Portugal",
        "lat": 38.7197,
        "lng": -9.20306,
        "alt": 158,
        "url": "http://en.wikipedia.org/wiki/Monsanto_Park_Circuit"
    },
    {
        "circuit_ref": "montjuic",
        "name": "Montjuïc",
        "location": "Barcelona",
        "country": "Spain",
        "lat": 41.3664,
        "lng": 2.15167,
        "alt": 79,
        "url": "http://en.wikipedia.org/wiki/Montju%C3%AFc_circuit"
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
        "circuit_ref": "mosport",
        "name": "Mosport International Raceway",
        "location": "Ontario",
        "country": "Canada",
        "lat": 44.0481,
        "lng": -78.6756,
        "alt": 332,
        "url": "http://en.wikipedia.org/wiki/Mosport"
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
        "circuit_ref": "nivelles",
        "name": "Nivelles-Baulers",
        "location": "Brussels",
        "country": "Belgium",
        "lat": 50.6211,
        "lng": 4.32694,
        "alt": 139,
        "url": "http://en.wikipedia.org/wiki/Nivelles-Baulers"
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
        "circuit_ref": "okayama",
        "name": "Okayama International Circuit",
        "location": "Okayama",
        "country": "Japan",
        "lat": 34.915,
        "lng": 134.221,
        "alt": 266,
        "url": "http://en.wikipedia.org/wiki/TI_Circuit"
    },
    {
        "circuit_ref": "pedralbes",
        "name": "Circuit de Pedralbes",
        "location": "Barcelona",
        "country": "Spain",
        "lat": 41.3903,
        "lng": 2.11667,
        "alt": 85,
        "url": "http://en.wikipedia.org/wiki/Pedralbes_Circuit"
    },
    {
        "circuit_ref": "pescara",
        "name": "Pescara Circuit",
        "location": "Pescara",
        "country": "Italy",
        "lat": 42.475,
        "lng": 14.1508,
        "alt": 129,
        "url": "http://en.wikipedia.org/wiki/Pescara_Circuit"
    },
    {
        "circuit_ref": "phoenix",
        "name": "Phoenix street circuit",
        "location": "Phoenix",
        "country": "USA",
        "lat": 33.4479,
        "lng": -112.075,
        "alt": 345,
        "url": "http://en.wikipedia.org/wiki/Phoenix_street_circuit"
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
        "circuit_ref": "reims",
        "name": "Reims-Gueux",
        "location": "Reims",
        "country": "France",
        "lat": 49.2542,
        "lng": 3.93083,
        "alt": 88,
        "url": "http://en.wikipedia.org/wiki/Reims-Gueux"
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
        "circuit_ref": "sebring",
        "name": "Sebring International Raceway",
        "location": "Florida",
        "country": "USA",
        "lat": 27.4547,
        "lng": -81.3483,
        "alt": 18,
        "url": "http://en.wikipedia.org/wiki/Sebring_Raceway"
    }
];
