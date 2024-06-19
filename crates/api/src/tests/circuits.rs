use serde_json::json;

use super::common::Test;

#[tokio::test]
async fn test_get_circuit() {
    Test::new(
        r#"{
            circuit(circuitRef: "spa") {
                circuitRef
                name
                location
                country
                lat
                lng
                alt
                url
            }
        }"#,
        spa(),
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_circuits_by_driver_ref() {
    Test::new(
        r#"{
            circuits(options: { driverRef: "leclerc" }) {
                data {
                    circuitRef
                    name
                    location
                    country
                    lat
                    lng
                    alt
                    url
                }
            }
        }"#,
        leclerc_circuits(),
    )
    .specify_field("circuits")
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_circuits_by_driver_ref_and_win() {
    Test::new(
        r#"{
            circuits(options: { driverRef: "leclerc", result: 1 }) {
                data {
                    circuitRef
                    name
                    location
                    country
                    lat
                    lng
                    alt
                    url
                }
            }
        }"#,
        leclerc_circuits_win(),
    )
    .specify_field("circuits")
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_circuits_by_driver_ref_and_win_and_pole() {
    Test::new(
        r#"{
            circuits(options: { driverRef: "leclerc", result: 1, grid: 1 }) {
                data {
                    circuitRef
                    name
                    location
                    country
                    lat
                    lng
                    alt
                    url
                }
            }
        }"#,
        leclerc_circuits_win_and_pole(),
    )
    .specify_field("circuits")
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_circuits_by_constructor_ref() {
    Test::new(
        r#"{
            circuits(options: { constructorRef: "ferrari" }) {
                data {
                    circuitRef
                    name
                    location
                    country
                    lat
                    lng
                    alt
                    url
                }
            }
        }"#,
        ferrari_circuits(),
    )
    .specify_field("circuits")
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_circuits_by_constructor_ref_and_page() {
    Test::new(
        r#"{
            circuits(
                options: { constructorRef: "ferrari" }
                pagination: { page: 2, limit: 30 }
            ) {
                data {
                    circuitRef
                    name
                    location
                    country
                    lat
                    lng
                    alt
                    url
                }
            }
        }"#,
        ferrari_circuits_page_2(),
    )
    .specify_field("circuits")
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_circuits_by_year_and_round() {
    Test::new(
        r#"{
            circuits(options: { year: 2023, round: 22 }) {
                data {
                    circuitRef
                    name
                    location
                    country
                    lat
                    lng
                    alt
                    url
                }
            }
        }"#,
        yas_marina(),
    )
    .specify_field("circuits")
    .test_ok()
    .await
}

fn spa() -> serde_json::Value {
    json! ({
        "circuit": {
            "circuitRef": "spa",
            "name": "Circuit de Spa-Francorchamps",
            "location": "Spa",
            "country": "Belgium",
            "lat": 50.437198638916016,
            "lng": 5.9713897705078125,
            "alt": 401,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
        }
    })
}

fn leclerc_circuits() -> serde_json::Value {
    json!({
        "data": [
                    {
                        "circuitRef": "albert_park",
                        "name": "Albert Park Grand Prix Circuit",
                        "location": "Melbourne",
                        "country": "Australia",
                        "lat": -37.849700927734375,
                        "lng": 144.96800231933594,
                        "alt": 10,
                        "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit"
                    },
                    {
                        "circuitRef": "americas",
                        "name": "Circuit of the Americas",
                        "location": "Austin",
                        "country": "USA",
                        "lat": 30.13279914855957,
                        "lng": -97.64109802246094,
                        "alt": 161,
                        "url": "http://en.wikipedia.org/wiki/Circuit_of_the_Americas"
                    },
                    {
                        "circuitRef": "bahrain",
                        "name": "Bahrain International Circuit",
                        "location": "Sakhir",
                        "country": "Bahrain",
                        "lat": 26.032499313354492,
                        "lng": 50.51060104370117,
                        "alt": 7,
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
                    },
                    {
                        "circuitRef": "baku",
                        "name": "Baku City Circuit",
                        "location": "Baku",
                        "country": "Azerbaijan",
                        "lat": 40.372501373291016,
                        "lng": 49.85329818725586,
                        "alt": -7,
                        "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit"
                    },
                    {
                        "circuitRef": "catalunya",
                        "name": "Circuit de Barcelona-Catalunya",
                        "location": "Montmeló",
                        "country": "Spain",
                        "lat": 41.56999969482422,
                        "lng": 2.2611100673675537,
                        "alt": 109,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Barcelona-Catalunya"
                    },
                    {
                        "circuitRef": "hockenheimring",
                        "name": "Hockenheimring",
                        "location": "Hockenheim",
                        "country": "Germany",
                        "lat": 49.32780075073242,
                        "lng": 8.56583023071289,
                        "alt": 103,
                        "url": "http://en.wikipedia.org/wiki/Hockenheimring"
                    },
                    {
                        "circuitRef": "hungaroring",
                        "name": "Hungaroring",
                        "location": "Budapest",
                        "country": "Hungary",
                        "lat": 47.57889938354492,
                        "lng": 19.248600006103516,
                        "alt": 264,
                        "url": "http://en.wikipedia.org/wiki/Hungaroring"
                    },
                    {
                        "circuitRef": "imola",
                        "name": "Autodromo Enzo e Dino Ferrari",
                        "location": "Imola",
                        "country": "Italy",
                        "lat": 44.34389877319336,
                        "lng": 11.716699600219727,
                        "alt": 37,
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Enzo_e_Dino_Ferrari"
                    },
                    {
                        "circuitRef": "interlagos",
                        "name": "Autódromo José Carlos Pace",
                        "location": "São Paulo",
                        "country": "Brazil",
                        "lat": -23.70359992980957,
                        "lng": -46.69969940185547,
                        "alt": 785,
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Jos%C3%A9_Carlos_Pace"
                    },
                    {
                        "circuitRef": "istanbul",
                        "name": "Istanbul Park",
                        "location": "Istanbul",
                        "country": "Turkey",
                        "lat": 40.951698303222656,
                        "lng": 29.405000686645508,
                        "alt": 130,
                        "url": "http://en.wikipedia.org/wiki/Istanbul_Park"
                    },
                    {
                        "circuitRef": "jeddah",
                        "name": "Jeddah Corniche Circuit",
                        "location": "Jeddah",
                        "country": "Saudi Arabia",
                        "lat": 21.631900787353516,
                        "lng": 39.104400634765625,
                        "alt": 15,
                        "url": "http://en.wikipedia.org/wiki/Jeddah_Street_Circuit"
                    },
                    {
                        "circuitRef": "losail",
                        "name": "Losail International Circuit",
                        "location": "Al Daayen",
                        "country": "Qatar",
                        "lat": 25.489999771118164,
                        "lng": 51.454200744628906,
                        "alt": 12,
                        "url": "http://en.wikipedia.org/wiki/Losail_International_Circuit"
                    },
                    {
                        "circuitRef": "marina_bay",
                        "name": "Marina Bay Street Circuit",
                        "location": "Marina Bay",
                        "country": "Singapore",
                        "lat": 1.2913999557495117,
                        "lng": 103.86399841308594,
                        "alt": 18,
                        "url": "http://en.wikipedia.org/wiki/Marina_Bay_Street_Circuit"
                    },
                    {
                        "circuitRef": "miami",
                        "name": "Miami International Autodrome",
                        "location": "Miami",
                        "country": "USA",
                        "lat": 25.958099365234375,
                        "lng": -80.23889923095703,
                        "alt": 0,
                        "url": "http://en.wikipedia.org/wiki/Miami_International_Autodrome"
                    },
                    {
                        "circuitRef": "monaco",
                        "name": "Circuit de Monaco",
                        "location": "Monte-Carlo",
                        "country": "Monaco",
                        "lat": 43.73469924926758,
                        "lng": 7.420559883117676,
                        "alt": 7,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco"
                    },
                    {
                        "circuitRef": "monza",
                        "name": "Autodromo Nazionale di Monza",
                        "location": "Monza",
                        "country": "Italy",
                        "lat": 45.6156005859375,
                        "lng": 9.281109809875488,
                        "alt": 162,
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
                    },
                    {
                        "circuitRef": "mugello",
                        "name": "Autodromo Internazionale del Mugello",
                        "location": "Mugello",
                        "country": "Italy",
                        "lat": 43.997501373291016,
                        "lng": 11.371899604797363,
                        "alt": 255,
                        "url": "http://en.wikipedia.org/wiki/Mugello_Circuit"
                    },
                    {
                        "circuitRef": "nurburgring",
                        "name": "Nürburgring",
                        "location": "Nürburg",
                        "country": "Germany",
                        "lat": 50.335601806640625,
                        "lng": 6.947500228881836,
                        "alt": 578,
                        "url": "http://en.wikipedia.org/wiki/N%C3%BCrburgring"
                    },
                    {
                        "circuitRef": "portimao",
                        "name": "Autódromo Internacional do Algarve",
                        "location": "Portimão",
                        "country": "Portugal",
                        "lat": 37.22700119018555,
                        "lng": -8.626700401306152,
                        "alt": 108,
                        "url": "http://en.wikipedia.org/wiki/Algarve_International_Circuit"
                    },
                    {
                        "circuitRef": "red_bull_ring",
                        "name": "Red Bull Ring",
                        "location": "Spielberg",
                        "country": "Austria",
                        "lat": 47.21969985961914,
                        "lng": 14.764699935913086,
                        "alt": 678,
                        "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring"
                    },
                    {
                        "circuitRef": "ricard",
                        "name": "Circuit Paul Ricard",
                        "location": "Le Castellet",
                        "country": "France",
                        "lat": 43.2505989074707,
                        "lng": 5.791669845581055,
                        "alt": 432,
                        "url": "http://en.wikipedia.org/wiki/Paul_Ricard_Circuit"
                    },
                    {
                        "circuitRef": "rodriguez",
                        "name": "Autódromo Hermanos Rodríguez",
                        "location": "Mexico City",
                        "country": "Mexico",
                        "lat": 19.404199600219727,
                        "lng": -99.0906982421875,
                        "alt": 2227,
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Hermanos_Rodr%C3%ADguez"
                    },
                    {
                        "circuitRef": "shanghai",
                        "name": "Shanghai International Circuit",
                        "location": "Shanghai",
                        "country": "China",
                        "lat": 31.338899612426758,
                        "lng": 121.22000122070312,
                        "alt": 5,
                        "url": "http://en.wikipedia.org/wiki/Shanghai_International_Circuit"
                    },
                    {
                        "circuitRef": "silverstone",
                        "name": "Silverstone Circuit",
                        "location": "Silverstone",
                        "country": "UK",
                        "lat": 52.0786018371582,
                        "lng": -1.0169399976730347,
                        "alt": 153,
                        "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit"
                    },
                    {
                        "circuitRef": "sochi",
                        "name": "Sochi Autodrom",
                        "location": "Sochi",
                        "country": "Russia",
                        "lat": 43.40570068359375,
                        "lng": 39.957801818847656,
                        "alt": 2,
                        "url": "http://en.wikipedia.org/wiki/Sochi_Autodrom"
                    },
                    {
                        "circuitRef": "spa",
                        "name": "Circuit de Spa-Francorchamps",
                        "location": "Spa",
                        "country": "Belgium",
                        "lat": 50.437198638916016,
                        "lng": 5.9713897705078125,
                        "alt": 401,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
                    },
                    {
                        "circuitRef": "suzuka",
                        "name": "Suzuka Circuit",
                        "location": "Suzuka",
                        "country": "Japan",
                        "lat": 34.843101501464844,
                        "lng": 136.54100036621094,
                        "alt": 45,
                        "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit"
                    },
                    {
                        "circuitRef": "vegas",
                        "name": "Las Vegas Strip Street Circuit",
                        "location": "Las Vegas",
                        "country": "United States",
                        "lat": 36.11470031738281,
                        "lng": -115.1729965209961,
                        "alt": 642,
                        "url": "https://en.wikipedia.org/wiki/Las_Vegas_Grand_Prix#Circuit"
                    },
                    {
                        "circuitRef": "villeneuve",
                        "name": "Circuit Gilles Villeneuve",
                        "location": "Montreal",
                        "country": "Canada",
                        "lat": 45.5,
                        "lng": -73.52279663085938,
                        "alt": 13,
                        "url": "http://en.wikipedia.org/wiki/Circuit_Gilles_Villeneuve"
                    },
                    {
                        "circuitRef": "yas_marina",
                        "name": "Yas Marina Circuit",
                        "location": "Abu Dhabi",
                        "country": "UAE",
                        "lat": 24.467199325561523,
                        "lng": 54.60309982299805,
                        "alt": 3,
                        "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit"
                    }
                ]
    })
}

fn leclerc_circuits_win() -> serde_json::Value {
    json!({
        "data": [
                    {
                        "circuitRef": "albert_park",
                        "name": "Albert Park Grand Prix Circuit",
                        "location": "Melbourne",
                        "country": "Australia",
                        "lat": -37.849700927734375,
                        "lng": 144.96800231933594,
                        "alt": 10,
                        "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit"
                    },
                    {
                        "circuitRef": "bahrain",
                        "name": "Bahrain International Circuit",
                        "location": "Sakhir",
                        "country": "Bahrain",
                        "lat": 26.032499313354492,
                        "lng": 50.51060104370117,
                        "alt": 7,
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
                    },
                    {
                        "circuitRef": "monza",
                        "name": "Autodromo Nazionale di Monza",
                        "location": "Monza",
                        "country": "Italy",
                        "lat": 45.6156005859375,
                        "lng": 9.281109809875488,
                        "alt": 162,
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
                    },
                    {
                        "circuitRef": "red_bull_ring",
                        "name": "Red Bull Ring",
                        "location": "Spielberg",
                        "country": "Austria",
                        "lat": 47.21969985961914,
                        "lng": 14.764699935913086,
                        "alt": 678,
                        "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring"
                    },
                    {
                        "circuitRef": "spa",
                        "name": "Circuit de Spa-Francorchamps",
                        "location": "Spa",
                        "country": "Belgium",
                        "lat": 50.437198638916016,
                        "lng": 5.9713897705078125,
                        "alt": 401,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
                    }
                ]
    })
}

fn leclerc_circuits_win_and_pole() -> serde_json::Value {
    json!({
        "data": [
                    {
                        "circuitRef": "albert_park",
                        "name": "Albert Park Grand Prix Circuit",
                        "location": "Melbourne",
                        "country": "Australia",
                        "lat": -37.849700927734375,
                        "lng": 144.96800231933594,
                        "alt": 10,
                        "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit"
                    },
                    {
                        "circuitRef": "bahrain",
                        "name": "Bahrain International Circuit",
                        "location": "Sakhir",
                        "country": "Bahrain",
                        "lat": 26.032499313354492,
                        "lng": 50.51060104370117,
                        "alt": 7,
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
                    },
                    {
                        "circuitRef": "monza",
                        "name": "Autodromo Nazionale di Monza",
                        "location": "Monza",
                        "country": "Italy",
                        "lat": 45.6156005859375,
                        "lng": 9.281109809875488,
                        "alt": 162,
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
                    },
                    {
                        "circuitRef": "spa",
                        "name": "Circuit de Spa-Francorchamps",
                        "location": "Spa",
                        "country": "Belgium",
                        "lat": 50.437198638916016,
                        "lng": 5.9713897705078125,
                        "alt": 401,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
                    }
                ]
    })
}

fn ferrari_circuits() -> serde_json::Value {
    json!({
        "data": [
                    {
                        "circuitRef": "adelaide",
                        "name": "Adelaide Street Circuit",
                        "location": "Adelaide",
                        "country": "Australia",
                        "lat": -34.92720031738281,
                        "lng": 138.61700439453125,
                        "alt": 58,
                        "url": "http://en.wikipedia.org/wiki/Adelaide_Street_Circuit"
                    },
                    {
                        "circuitRef": "ain-diab",
                        "name": "Ain Diab",
                        "location": "Casablanca",
                        "country": "Morocco",
                        "lat": 33.5786018371582,
                        "lng": -7.6875,
                        "alt": 19,
                        "url": "http://en.wikipedia.org/wiki/Ain-Diab_Circuit"
                    },
                    {
                        "circuitRef": "aintree",
                        "name": "Aintree",
                        "location": "Liverpool",
                        "country": "UK",
                        "lat": 53.476898193359375,
                        "lng": -2.9405601024627686,
                        "alt": 20,
                        "url": "http://en.wikipedia.org/wiki/Aintree_Motor_Racing_Circuit"
                    },
                    {
                        "circuitRef": "albert_park",
                        "name": "Albert Park Grand Prix Circuit",
                        "location": "Melbourne",
                        "country": "Australia",
                        "lat": -37.849700927734375,
                        "lng": 144.96800231933594,
                        "alt": 10,
                        "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit"
                    },
                    {
                        "circuitRef": "americas",
                        "name": "Circuit of the Americas",
                        "location": "Austin",
                        "country": "USA",
                        "lat": 30.13279914855957,
                        "lng": -97.64109802246094,
                        "alt": 161,
                        "url": "http://en.wikipedia.org/wiki/Circuit_of_the_Americas"
                    },
                    {
                        "circuitRef": "anderstorp",
                        "name": "Scandinavian Raceway",
                        "location": "Anderstorp",
                        "country": "Sweden",
                        "lat": 57.26530075073242,
                        "lng": 13.60420036315918,
                        "alt": 153,
                        "url": "http://en.wikipedia.org/wiki/Scandinavian_Raceway"
                    },
                    {
                        "circuitRef": "avus",
                        "name": "AVUS",
                        "location": "Berlin",
                        "country": "Germany",
                        "lat": 52.48059844970703,
                        "lng": 13.251399993896484,
                        "alt": 53,
                        "url": "http://en.wikipedia.org/wiki/AVUS"
                    },
                    {
                        "circuitRef": "bahrain",
                        "name": "Bahrain International Circuit",
                        "location": "Sakhir",
                        "country": "Bahrain",
                        "lat": 26.032499313354492,
                        "lng": 50.51060104370117,
                        "alt": 7,
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
                    },
                    {
                        "circuitRef": "baku",
                        "name": "Baku City Circuit",
                        "location": "Baku",
                        "country": "Azerbaijan",
                        "lat": 40.372501373291016,
                        "lng": 49.85329818725586,
                        "alt": -7,
                        "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit"
                    },
                    {
                        "circuitRef": "boavista",
                        "name": "Circuito da Boavista",
                        "location": "Oporto",
                        "country": "Portugal",
                        "lat": 41.170501708984375,
                        "lng": -8.673250198364258,
                        "alt": 28,
                        "url": "http://en.wikipedia.org/wiki/Circuito_da_Boavista"
                    },
                    {
                        "circuitRef": "brands_hatch",
                        "name": "Brands Hatch",
                        "location": "Kent",
                        "country": "UK",
                        "lat": 51.35689926147461,
                        "lng": 0.2630560100078583,
                        "alt": 145,
                        "url": "http://en.wikipedia.org/wiki/Brands_Hatch"
                    },
                    {
                        "circuitRef": "bremgarten",
                        "name": "Circuit Bremgarten",
                        "location": "Bern",
                        "country": "Switzerland",
                        "lat": 46.958900451660156,
                        "lng": 7.401939868927002,
                        "alt": 551,
                        "url": "http://en.wikipedia.org/wiki/Circuit_Bremgarten"
                    },
                    {
                        "circuitRef": "buddh",
                        "name": "Buddh International Circuit",
                        "location": "Uttar Pradesh",
                        "country": "India",
                        "lat": 28.34869956970215,
                        "lng": 77.53309631347656,
                        "alt": 194,
                        "url": "http://en.wikipedia.org/wiki/Buddh_International_Circuit"
                    },
                    {
                        "circuitRef": "catalunya",
                        "name": "Circuit de Barcelona-Catalunya",
                        "location": "Montmeló",
                        "country": "Spain",
                        "lat": 41.56999969482422,
                        "lng": 2.2611100673675537,
                        "alt": 109,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Barcelona-Catalunya"
                    },
                    {
                        "circuitRef": "charade",
                        "name": "Charade Circuit",
                        "location": "Clermont-Ferrand",
                        "country": "France",
                        "lat": 45.74720001220703,
                        "lng": 3.0388898849487305,
                        "alt": 790,
                        "url": "http://en.wikipedia.org/wiki/Charade_Circuit"
                    },
                    {
                        "circuitRef": "dallas",
                        "name": "Fair Park",
                        "location": "Dallas",
                        "country": "USA",
                        "lat": 32.777400970458984,
                        "lng": -96.75869750976562,
                        "alt": 139,
                        "url": "http://en.wikipedia.org/wiki/Fair_Park"
                    },
                    {
                        "circuitRef": "detroit",
                        "name": "Detroit Street Circuit",
                        "location": "Detroit",
                        "country": "USA",
                        "lat": 42.32979965209961,
                        "lng": -83.04010009765625,
                        "alt": 177,
                        "url": "http://en.wikipedia.org/wiki/Detroit_street_circuit"
                    },
                    {
                        "circuitRef": "dijon",
                        "name": "Dijon-Prenois",
                        "location": "Dijon",
                        "country": "France",
                        "lat": 47.36249923706055,
                        "lng": 4.899129867553711,
                        "alt": 484,
                        "url": "http://en.wikipedia.org/wiki/Dijon-Prenois"
                    },
                    {
                        "circuitRef": "donington",
                        "name": "Donington Park",
                        "location": "Castle Donington",
                        "country": "UK",
                        "lat": 52.83060073852539,
                        "lng": -1.3752800226211548,
                        "alt": 88,
                        "url": "http://en.wikipedia.org/wiki/Donington_Park"
                    },
                    {
                        "circuitRef": "essarts",
                        "name": "Rouen-Les-Essarts",
                        "location": "Rouen",
                        "country": "France",
                        "lat": 49.33060073852539,
                        "lng": 1.004580020904541,
                        "alt": 81,
                        "url": "http://en.wikipedia.org/wiki/Rouen-Les-Essarts"
                    },
                    {
                        "circuitRef": "estoril",
                        "name": "Autódromo do Estoril",
                        "location": "Estoril",
                        "country": "Portugal",
                        "lat": 38.7505989074707,
                        "lng": -9.394169807434082,
                        "alt": 130,
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_do_Estoril"
                    },
                    {
                        "circuitRef": "fuji",
                        "name": "Fuji Speedway",
                        "location": "Oyama",
                        "country": "Japan",
                        "lat": 35.371700286865234,
                        "lng": 138.927001953125,
                        "alt": 583,
                        "url": "http://en.wikipedia.org/wiki/Fuji_Speedway"
                    },
                    {
                        "circuitRef": "galvez",
                        "name": "Autódromo Juan y Oscar Gálvez",
                        "location": "Buenos Aires",
                        "country": "Argentina",
                        "lat": -34.69430160522461,
                        "lng": -58.45930099487305,
                        "alt": 8,
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Oscar_Alfredo_G%C3%A1lvez"
                    },
                    {
                        "circuitRef": "george",
                        "name": "Prince George Circuit",
                        "location": "Eastern Cape Province",
                        "country": "South Africa",
                        "lat": -33.04859924316406,
                        "lng": 27.873600006103516,
                        "alt": 15,
                        "url": "http://en.wikipedia.org/wiki/Prince_George_Circuit"
                    },
                    {
                        "circuitRef": "hockenheimring",
                        "name": "Hockenheimring",
                        "location": "Hockenheim",
                        "country": "Germany",
                        "lat": 49.32780075073242,
                        "lng": 8.56583023071289,
                        "alt": 103,
                        "url": "http://en.wikipedia.org/wiki/Hockenheimring"
                    },
                    {
                        "circuitRef": "hungaroring",
                        "name": "Hungaroring",
                        "location": "Budapest",
                        "country": "Hungary",
                        "lat": 47.57889938354492,
                        "lng": 19.248600006103516,
                        "alt": 264,
                        "url": "http://en.wikipedia.org/wiki/Hungaroring"
                    },
                    {
                        "circuitRef": "imola",
                        "name": "Autodromo Enzo e Dino Ferrari",
                        "location": "Imola",
                        "country": "Italy",
                        "lat": 44.34389877319336,
                        "lng": 11.716699600219727,
                        "alt": 37,
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Enzo_e_Dino_Ferrari"
                    },
                    {
                        "circuitRef": "indianapolis",
                        "name": "Indianapolis Motor Speedway",
                        "location": "Indianapolis",
                        "country": "USA",
                        "lat": 39.79499816894531,
                        "lng": -86.23470306396484,
                        "alt": 223,
                        "url": "http://en.wikipedia.org/wiki/Indianapolis_Motor_Speedway"
                    },
                    {
                        "circuitRef": "interlagos",
                        "name": "Autódromo José Carlos Pace",
                        "location": "São Paulo",
                        "country": "Brazil",
                        "lat": -23.70359992980957,
                        "lng": -46.69969940185547,
                        "alt": 785,
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Jos%C3%A9_Carlos_Pace"
                    },
                    {
                        "circuitRef": "istanbul",
                        "name": "Istanbul Park",
                        "location": "Istanbul",
                        "country": "Turkey",
                        "lat": 40.951698303222656,
                        "lng": 29.405000686645508,
                        "alt": 130,
                        "url": "http://en.wikipedia.org/wiki/Istanbul_Park"
                    }
                ]
    })
}

fn ferrari_circuits_page_2() -> serde_json::Value {
    json!({
        "data": [
                    {
                        "circuitRef": "jacarepagua",
                        "name": "Autódromo Internacional Nelson Piquet",
                        "location": "Rio de Janeiro",
                        "country": "Brazil",
                        "lat": -22.97559928894043,
                        "lng": -43.39500045776367,
                        "alt": 1126,
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Internacional_Nelson_Piquet"
                    },
                    {
                        "circuitRef": "jarama",
                        "name": "Jarama",
                        "location": "Madrid",
                        "country": "Spain",
                        "lat": 40.61709976196289,
                        "lng": -3.5855801105499268,
                        "alt": 609,
                        "url": "http://en.wikipedia.org/wiki/Circuito_Permanente_Del_Jarama"
                    },
                    {
                        "circuitRef": "jeddah",
                        "name": "Jeddah Corniche Circuit",
                        "location": "Jeddah",
                        "country": "Saudi Arabia",
                        "lat": 21.631900787353516,
                        "lng": 39.104400634765625,
                        "alt": 15,
                        "url": "http://en.wikipedia.org/wiki/Jeddah_Street_Circuit"
                    },
                    {
                        "circuitRef": "jerez",
                        "name": "Circuito de Jerez",
                        "location": "Jerez de la Frontera",
                        "country": "Spain",
                        "lat": 36.70830154418945,
                        "lng": -6.034170150756836,
                        "alt": 37,
                        "url": "http://en.wikipedia.org/wiki/Circuito_Permanente_de_Jerez"
                    },
                    {
                        "circuitRef": "kyalami",
                        "name": "Kyalami",
                        "location": "Midrand",
                        "country": "South Africa",
                        "lat": -25.98940086364746,
                        "lng": 28.07670021057129,
                        "alt": 1460,
                        "url": "http://en.wikipedia.org/wiki/Kyalami"
                    },
                    {
                        "circuitRef": "las_vegas",
                        "name": "Las Vegas Street Circuit",
                        "location": "Nevada",
                        "country": "USA",
                        "lat": 36.1161994934082,
                        "lng": -115.17400360107422,
                        "alt": 639,
                        "url": "http://en.wikipedia.org/wiki/Las_Vegas_Street_Circuit"
                    },
                    {
                        "circuitRef": "lemans",
                        "name": "Le Mans",
                        "location": "Le Mans",
                        "country": "France",
                        "lat": 47.95000076293945,
                        "lng": 0.22423100471496582,
                        "alt": 67,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_la_Sarthe#Bugatti_Circuit"
                    },
                    {
                        "circuitRef": "long_beach",
                        "name": "Long Beach",
                        "location": "California",
                        "country": "USA",
                        "lat": 33.765098571777344,
                        "lng": -118.18900299072266,
                        "alt": 12,
                        "url": "http://en.wikipedia.org/wiki/Long_Beach,_California"
                    },
                    {
                        "circuitRef": "losail",
                        "name": "Losail International Circuit",
                        "location": "Al Daayen",
                        "country": "Qatar",
                        "lat": 25.489999771118164,
                        "lng": 51.454200744628906,
                        "alt": 12,
                        "url": "http://en.wikipedia.org/wiki/Losail_International_Circuit"
                    },
                    {
                        "circuitRef": "magny_cours",
                        "name": "Circuit de Nevers Magny-Cours",
                        "location": "Magny Cours",
                        "country": "France",
                        "lat": 46.864200592041016,
                        "lng": 3.1636099815368652,
                        "alt": 228,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Nevers_Magny-Cours"
                    },
                    {
                        "circuitRef": "marina_bay",
                        "name": "Marina Bay Street Circuit",
                        "location": "Marina Bay",
                        "country": "Singapore",
                        "lat": 1.2913999557495117,
                        "lng": 103.86399841308594,
                        "alt": 18,
                        "url": "http://en.wikipedia.org/wiki/Marina_Bay_Street_Circuit"
                    },
                    {
                        "circuitRef": "miami",
                        "name": "Miami International Autodrome",
                        "location": "Miami",
                        "country": "USA",
                        "lat": 25.958099365234375,
                        "lng": -80.23889923095703,
                        "alt": 0,
                        "url": "http://en.wikipedia.org/wiki/Miami_International_Autodrome"
                    },
                    {
                        "circuitRef": "monaco",
                        "name": "Circuit de Monaco",
                        "location": "Monte-Carlo",
                        "country": "Monaco",
                        "lat": 43.73469924926758,
                        "lng": 7.420559883117676,
                        "alt": 7,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco"
                    },
                    {
                        "circuitRef": "monsanto",
                        "name": "Monsanto Park Circuit",
                        "location": "Lisbon",
                        "country": "Portugal",
                        "lat": 38.71969985961914,
                        "lng": -9.203060150146484,
                        "alt": 158,
                        "url": "http://en.wikipedia.org/wiki/Monsanto_Park_Circuit"
                    },
                    {
                        "circuitRef": "montjuic",
                        "name": "Montjuïc",
                        "location": "Barcelona",
                        "country": "Spain",
                        "lat": 41.36640167236328,
                        "lng": 2.151669979095459,
                        "alt": 79,
                        "url": "http://en.wikipedia.org/wiki/Montju%C3%AFc_circuit"
                    },
                    {
                        "circuitRef": "monza",
                        "name": "Autodromo Nazionale di Monza",
                        "location": "Monza",
                        "country": "Italy",
                        "lat": 45.6156005859375,
                        "lng": 9.281109809875488,
                        "alt": 162,
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
                    },
                    {
                        "circuitRef": "mosport",
                        "name": "Mosport International Raceway",
                        "location": "Ontario",
                        "country": "Canada",
                        "lat": 44.048099517822266,
                        "lng": -78.67559814453125,
                        "alt": 332,
                        "url": "http://en.wikipedia.org/wiki/Mosport"
                    },
                    {
                        "circuitRef": "mugello",
                        "name": "Autodromo Internazionale del Mugello",
                        "location": "Mugello",
                        "country": "Italy",
                        "lat": 43.997501373291016,
                        "lng": 11.371899604797363,
                        "alt": 255,
                        "url": "http://en.wikipedia.org/wiki/Mugello_Circuit"
                    },
                    {
                        "circuitRef": "nivelles",
                        "name": "Nivelles-Baulers",
                        "location": "Brussels",
                        "country": "Belgium",
                        "lat": 50.62110137939453,
                        "lng": 4.326940059661865,
                        "alt": 139,
                        "url": "http://en.wikipedia.org/wiki/Nivelles-Baulers"
                    },
                    {
                        "circuitRef": "nurburgring",
                        "name": "Nürburgring",
                        "location": "Nürburg",
                        "country": "Germany",
                        "lat": 50.335601806640625,
                        "lng": 6.947500228881836,
                        "alt": 578,
                        "url": "http://en.wikipedia.org/wiki/N%C3%BCrburgring"
                    },
                    {
                        "circuitRef": "okayama",
                        "name": "Okayama International Circuit",
                        "location": "Okayama",
                        "country": "Japan",
                        "lat": 34.915000915527344,
                        "lng": 134.2209930419922,
                        "alt": 266,
                        "url": "http://en.wikipedia.org/wiki/TI_Circuit"
                    },
                    {
                        "circuitRef": "pedralbes",
                        "name": "Circuit de Pedralbes",
                        "location": "Barcelona",
                        "country": "Spain",
                        "lat": 41.39030075073242,
                        "lng": 2.1166698932647705,
                        "alt": 85,
                        "url": "http://en.wikipedia.org/wiki/Pedralbes_Circuit"
                    },
                    {
                        "circuitRef": "pescara",
                        "name": "Pescara Circuit",
                        "location": "Pescara",
                        "country": "Italy",
                        "lat": 42.474998474121094,
                        "lng": 14.150799751281738,
                        "alt": 129,
                        "url": "http://en.wikipedia.org/wiki/Pescara_Circuit"
                    },
                    {
                        "circuitRef": "phoenix",
                        "name": "Phoenix street circuit",
                        "location": "Phoenix",
                        "country": "USA",
                        "lat": 33.447898864746094,
                        "lng": -112.07499694824219,
                        "alt": 345,
                        "url": "http://en.wikipedia.org/wiki/Phoenix_street_circuit"
                    },
                    {
                        "circuitRef": "portimao",
                        "name": "Autódromo Internacional do Algarve",
                        "location": "Portimão",
                        "country": "Portugal",
                        "lat": 37.22700119018555,
                        "lng": -8.626700401306152,
                        "alt": 108,
                        "url": "http://en.wikipedia.org/wiki/Algarve_International_Circuit"
                    },
                    {
                        "circuitRef": "red_bull_ring",
                        "name": "Red Bull Ring",
                        "location": "Spielberg",
                        "country": "Austria",
                        "lat": 47.21969985961914,
                        "lng": 14.764699935913086,
                        "alt": 678,
                        "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring"
                    },
                    {
                        "circuitRef": "reims",
                        "name": "Reims-Gueux",
                        "location": "Reims",
                        "country": "France",
                        "lat": 49.25419998168945,
                        "lng": 3.9308300018310547,
                        "alt": 88,
                        "url": "http://en.wikipedia.org/wiki/Reims-Gueux"
                    },
                    {
                        "circuitRef": "ricard",
                        "name": "Circuit Paul Ricard",
                        "location": "Le Castellet",
                        "country": "France",
                        "lat": 43.2505989074707,
                        "lng": 5.791669845581055,
                        "alt": 432,
                        "url": "http://en.wikipedia.org/wiki/Paul_Ricard_Circuit"
                    },
                    {
                        "circuitRef": "rodriguez",
                        "name": "Autódromo Hermanos Rodríguez",
                        "location": "Mexico City",
                        "country": "Mexico",
                        "lat": 19.404199600219727,
                        "lng": -99.0906982421875,
                        "alt": 2227,
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Hermanos_Rodr%C3%ADguez"
                    },
                    {
                        "circuitRef": "sebring",
                        "name": "Sebring International Raceway",
                        "location": "Florida",
                        "country": "USA",
                        "lat": 27.454700469970703,
                        "lng": -81.34829711914062,
                        "alt": 18,
                        "url": "http://en.wikipedia.org/wiki/Sebring_Raceway"
                    }
                ]
    })
}

fn yas_marina() -> serde_json::Value {
    json!({
        "data": [
                    {
                        "circuitRef": "yas_marina",
                        "name": "Yas Marina Circuit",
                        "location": "Abu Dhabi",
                        "country": "UAE",
                        "lat": 24.467199325561523,
                        "lng": 54.60309982299805,
                        "alt": 3,
                        "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit"
                    }
                ]
    })
}
