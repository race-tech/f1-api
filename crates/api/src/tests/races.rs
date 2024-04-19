use shared::prelude::*;

use super::common::models::*;
use super::common::Test;
use crate::races_from_json;

#[tokio::test]
async fn test_get_races_by_year() {
    Test::<&[StaticRace], Vec<RaceResponse>>::new(
        "/api/f1/races?year=2024",
        Series::F1,
        &ALL_2024_RACES,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 1,
        total: 24,
    }))
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_races_by_year_and_round() {
    Test::<&[StaticRace], Vec<RaceResponse>>::new(
        "/api/f1/races?year=2024&round=1",
        Series::F1,
        &BAHRAIN_2024_ROUND_1,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 1,
        total: 1,
    }))
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_races_by_driver_ref() {
    Test::<&[StaticRace], Vec<RaceResponse>>::new(
        "/api/f1/races?driver_ref=leclerc",
        Series::F1,
        &LECLERC_RACES,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 5,
        total: 129,
    }))
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_races_by_constructor_ref() {
    Test::<&[StaticRace], Vec<RaceResponse>>::new(
        "/api/f1/races?constructor_ref=ferrari",
        Series::F1,
        &FERRARI_RACES,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 36,
        total: 1080,
    }))
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_races_by_driver_ref_and_page() {
    Test::<&[StaticRace], Vec<RaceResponse>>::new(
        "/api/f1/races?driver_ref=leclerc&page=4",
        Series::F1,
        &LECLERC_RACES_PAGE_4,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 4,
        max_page: 5,
        total: 129,
    }))
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_races_by_constructor_ref_and_page() {
    Test::<&[StaticRace], Vec<RaceResponse>>::new(
        "/api/f1/races?constructor_ref=ferrari&page=36",
        Series::F1,
        &FERRARI_RACES_PAGE_36,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 36,
        max_page: 36,
        total: 1080,
    }))
    .test_ok()
    .await
}

const ALL_2024_RACES: [StaticRace; 24] = races_from_json![
    {
        "season": 2024,
        "round": 1,
        "name": "Bahrain Grand Prix",
        "date": "2024-03-02",
        "time": "15:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Bahrain_Grand_Prix",
        "fp1": {
            "date": "2024-02-29",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2024-02-29",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2024-03-01",
            "time": "12:30:00"
        },
        "quali": {
            "date": "2024-03-01",
            "time": "16:00:00"
        },
        "circuit": {
            "circuit_ref": "bahrain",
            "name": "Bahrain International Circuit",
            "location": "Sakhir",
            "country": "Bahrain",
            "lat": 26.0325,
            "lng": 50.5106,
            "alt": 7,
            "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
        }
    },
    {
        "season": 2024,
        "round": 2,
        "name": "Saudi Arabian Grand Prix",
        "date": "2024-03-09",
        "time": "17:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Saudi_Arabian_Grand_Prix",
        "fp1": {
            "date": "2024-03-07",
            "time": "13:30:00"
        },
        "fp2": {
            "date": "2024-03-07",
            "time": "17:00:00"
        },
        "fp3": {
            "date": "2024-03-08",
            "time": "13:30:00"
        },
        "quali": {
            "date": "2024-03-08",
            "time": "17:00:00"
        },
        "circuit": {
            "circuit_ref": "jeddah",
            "name": "Jeddah Corniche Circuit",
            "location": "Jeddah",
            "country": "Saudi Arabia",
            "lat": 21.6319,
            "lng": 39.1044,
            "alt": 15,
            "url": "http://en.wikipedia.org/wiki/Jeddah_Street_Circuit"
        }
    },
    {
        "season": 2024,
        "round": 3,
        "name": "Australian Grand Prix",
        "date": "2024-03-24",
        "time": "04:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Australian_Grand_Prix",
        "fp1": {
            "date": "2024-03-22",
            "time": "01:30:00"
        },
        "fp2": {
            "date": "2024-03-22",
            "time": "05:00:00"
        },
        "fp3": {
            "date": "2024-03-23",
            "time": "01:30:00"
        },
        "quali": {
            "date": "2024-03-23",
            "time": "05:00:00"
        },
        "circuit": {
            "circuit_ref": "albert_park",
            "name": "Albert Park Grand Prix Circuit",
            "location": "Melbourne",
            "country": "Australia",
            "lat": -37.8497,
            "lng": 144.968,
            "alt": 10,
            "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit"
        }
    },
    {
        "season": 2024,
        "round": 4,
        "name": "Japanese Grand Prix",
        "date": "2024-04-07",
        "time": "05:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Japanese_Grand_Prix",
        "fp1": {
            "date": "2024-04-05",
            "time": "02:30:00"
        },
        "fp2": {
            "date": "2024-04-05",
            "time": "06:00:00"
        },
        "fp3": {
            "date": "2024-04-06",
            "time": "02:30:00"
        },
        "quali": {
            "date": "2024-04-06",
            "time": "06:00:00"
        },
        "circuit": {
            "circuit_ref": "suzuka",
            "name": "Suzuka Circuit",
            "location": "Suzuka",
            "country": "Japan",
            "lat": 34.8431,
            "lng": 136.541,
            "alt": 45,
            "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit"
        }
    },
    {
        "season": 2024,
        "round": 5,
        "name": "Chinese Grand Prix",
        "date": "2024-04-21",
        "time": "07:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Chinese_Grand_Prix",
        "fp1": {
            "date": "2024-04-19",
            "time": "03:30:00"
        },
        "fp2": {
            "date": "2024-04-19",
            "time": "07:30:00"
        },
        "quali": {
            "date": "2024-04-20",
            "time": "07:00:00"
        },
        "sprint": {
            "date": "2024-04-20",
            "time": "03:00:00"
        },
        "circuit": {
            "circuit_ref": "shanghai",
            "name": "Shanghai International Circuit",
            "location": "Shanghai",
            "country": "China",
            "lat": 31.3389,
            "lng": 121.22,
            "alt": 5,
            "url": "http://en.wikipedia.org/wiki/Shanghai_International_Circuit"
        }
    },
    {
        "season": 2024,
        "round": 6,
        "name": "Miami Grand Prix",
        "date": "2024-05-05",
        "time": "20:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Miami_Grand_Prix",
        "fp1": {
            "date": "2024-05-03",
            "time": "16:30:00"
        },
        "fp2": {
            "date": "2024-05-03",
            "time": "20:30:00"
        },
        "quali": {
            "date": "2024-05-04",
            "time": "20:00:00"
        },
        "sprint": {
            "date": "2024-05-04",
            "time": "16:00:00"
        },
        "circuit": {
            "circuit_ref": "miami",
            "name": "Miami International Autodrome",
            "location": "Miami",
            "country": "USA",
            "lat": 25.9581,
            "lng": -80.2389,
            "alt": 0,
            "url": "http://en.wikipedia.org/wiki/Miami_International_Autodrome"
        }
    },
    {
        "season": 2024,
        "round": 7,
        "name": "Emilia Romagna Grand Prix",
        "date": "2024-05-19",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Emilia_Romagna_Grand_Prix",
        "fp1": {
            "date": "2024-05-17",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2024-05-17",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2024-05-18",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2024-05-18",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "imola",
            "name": "Autodromo Enzo e Dino Ferrari",
            "location": "Imola",
            "country": "Italy",
            "lat": 44.3439,
            "lng": 11.7167,
            "alt": 37,
            "url": "http://en.wikipedia.org/wiki/Autodromo_Enzo_e_Dino_Ferrari"
        }
    },
    {
        "season": 2024,
        "round": 8,
        "name": "Monaco Grand Prix",
        "date": "2024-05-26",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Monaco_Grand_Prix",
        "fp1": {
            "date": "2024-05-24",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2024-05-24",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2024-05-25",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2024-05-25",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "monaco",
            "name": "Circuit de Monaco",
            "location": "Monte-Carlo",
            "country": "Monaco",
            "lat": 43.7347,
            "lng": 7.42056,
            "alt": 7,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco"
        }
    },
    {
        "season": 2024,
        "round": 9,
        "name": "Canadian Grand Prix",
        "date": "2024-06-09",
        "time": "18:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Canadian_Grand_Prix",
        "fp1": {
            "date": "2024-06-07",
            "time": "17:30:00"
        },
        "fp2": {
            "date": "2024-06-07",
            "time": "21:00:00"
        },
        "fp3": {
            "date": "2024-06-08",
            "time": "16:30:00"
        },
        "quali": {
            "date": "2024-06-08",
            "time": "20:00:00"
        },
        "circuit": {
            "circuit_ref": "villeneuve",
            "name": "Circuit Gilles Villeneuve",
            "location": "Montreal",
            "country": "Canada",
            "lat": 45.5,
            "lng": -73.5228,
            "alt": 13,
            "url": "http://en.wikipedia.org/wiki/Circuit_Gilles_Villeneuve"
        }
    },
    {
        "season": 2024,
        "round": 10,
        "name": "Spanish Grand Prix",
        "date": "2024-06-23",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Spanish_Grand_Prix",
        "fp1": {
            "date": "2024-06-21",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2024-06-21",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2024-06-22",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2024-06-22",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "catalunya",
            "name": "Circuit de Barcelona-Catalunya",
            "location": "Montmeló",
            "country": "Spain",
            "lat": 41.57,
            "lng": 2.26111,
            "alt": 109,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Barcelona-Catalunya"
        }
    },
    {
        "season": 2024,
        "round": 11,
        "name": "Austrian Grand Prix",
        "date": "2024-06-30",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Austrian_Grand_Prix",
        "fp1": {
            "date": "2024-06-28",
            "time": "10:30:00"
        },
        "fp2": {
            "date": "2024-06-28",
            "time": "14:30:00"
        },
        "quali": {
            "date": "2024-06-29",
            "time": "14:00:00"
        },
        "sprint": {
            "date": "2024-06-29",
            "time": "10:00:00"
        },
        "circuit": {
            "circuit_ref": "red_bull_ring",
            "name": "Red Bull Ring",
            "location": "Spielberg",
            "country": "Austria",
            "lat": 47.2197,
            "lng": 14.7647,
            "alt": 678,
            "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring"
        }
    },
    {
        "season": 2024,
        "round": 12,
        "name": "British Grand Prix",
        "date": "2024-07-07",
        "time": "14:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_British_Grand_Prix",
        "fp1": {
            "date": "2024-07-05",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2024-07-05",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2024-07-06",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2024-07-06",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "silverstone",
            "name": "Silverstone Circuit",
            "location": "Silverstone",
            "country": "UK",
            "lat": 52.0786,
            "lng": -1.01694,
            "alt": 153,
            "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit"
        }
    },
    {
        "season": 2024,
        "round": 13,
        "name": "Hungarian Grand Prix",
        "date": "2024-07-21",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Hungarian_Grand_Prix",
        "fp1": {
            "date": "2024-07-19",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2024-07-19",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2024-07-20",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2024-07-20",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "hungaroring",
            "name": "Hungaroring",
            "location": "Budapest",
            "country": "Hungary",
            "lat": 47.5789,
            "lng": 19.2486,
            "alt": 264,
            "url": "http://en.wikipedia.org/wiki/Hungaroring"
        }
    },
    {
        "season": 2024,
        "round": 14,
        "name": "Belgian Grand Prix",
        "date": "2024-07-28",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Belgian_Grand_Prix",
        "fp1": {
            "date": "2024-07-26",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2024-07-26",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2024-07-27",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2024-07-27",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "spa",
            "name": "Circuit de Spa-Francorchamps",
            "location": "Spa",
            "country": "Belgium",
            "lat": 50.4372,
            "lng": 5.97139,
            "alt": 401,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
        }
    },
    {
        "season": 2024,
        "round": 15,
        "name": "Dutch Grand Prix",
        "date": "2024-08-25",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Dutch_Grand_Prix",
        "fp1": {
            "date": "2024-08-23",
            "time": "10:30:00"
        },
        "fp2": {
            "date": "2024-08-23",
            "time": "14:00:00"
        },
        "fp3": {
            "date": "2024-08-24",
            "time": "09:30:00"
        },
        "quali": {
            "date": "2024-08-24",
            "time": "13:00:00"
        },
        "circuit": {
            "circuit_ref": "zandvoort",
            "name": "Circuit Park Zandvoort",
            "location": "Zandvoort",
            "country": "Netherlands",
            "lat": 52.3888,
            "lng": 4.54092,
            "alt": 6,
            "url": "http://en.wikipedia.org/wiki/Circuit_Zandvoort"
        }
    },
    {
        "season": 2024,
        "round": 16,
        "name": "Italian Grand Prix",
        "date": "2024-09-01",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Italian_Grand_Prix",
        "fp1": {
            "date": "2024-08-30",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2024-08-30",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2024-08-31",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2024-08-31",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "monza",
            "name": "Autodromo Nazionale di Monza",
            "location": "Monza",
            "country": "Italy",
            "lat": 45.6156,
            "lng": 9.28111,
            "alt": 162,
            "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
        }
    },
    {
        "season": 2024,
        "round": 17,
        "name": "Azerbaijan Grand Prix",
        "date": "2024-09-15",
        "time": "11:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Azerbaijan_Grand_Prix",
        "fp1": {
            "date": "2024-09-13",
            "time": "09:30:00"
        },
        "fp2": {
            "date": "2024-09-13",
            "time": "13:00:00"
        },
        "fp3": {
            "date": "2024-09-14",
            "time": "08:30:00"
        },
        "quali": {
            "date": "2024-09-14",
            "time": "12:00:00"
        },
        "circuit": {
            "circuit_ref": "baku",
            "name": "Baku City Circuit",
            "location": "Baku",
            "country": "Azerbaijan",
            "lat": 40.3725,
            "lng": 49.8533,
            "alt": -7,
            "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit"
        }
    },
    {
        "season": 2024,
        "round": 18,
        "name": "Singapore Grand Prix",
        "date": "2024-09-22",
        "time": "12:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Singapore_Grand_Prix",
        "fp1": {
            "date": "2024-09-20",
            "time": "09:30:00"
        },
        "fp2": {
            "date": "2024-09-20",
            "time": "13:00:00"
        },
        "fp3": {
            "date": "2024-09-21",
            "time": "09:30:00"
        },
        "quali": {
            "date": "2024-09-21",
            "time": "13:00:00"
        },
        "circuit": {
            "circuit_ref": "marina_bay",
            "name": "Marina Bay Street Circuit",
            "location": "Marina Bay",
            "country": "Singapore",
            "lat": 1.2914,
            "lng": 103.864,
            "alt": 18,
            "url": "http://en.wikipedia.org/wiki/Marina_Bay_Street_Circuit"
        }
    },
    {
        "season": 2024,
        "round": 19,
        "name": "United States Grand Prix",
        "date": "2024-10-20",
        "time": "19:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_United_States_Grand_Prix",
        "fp1": {
            "date": "2024-10-18",
            "time": "17:30:00"
        },
        "fp2": {
            "date": "2024-10-18",
            "time": "21:30:00"
        },
        "quali": {
            "date": "2024-10-19",
            "time": "22:00:00"
        },
        "sprint": {
            "date": "2024-10-19",
            "time": "18:00:00"
        },
        "circuit": {
            "circuit_ref": "americas",
            "name": "Circuit of the Americas",
            "location": "Austin",
            "country": "USA",
            "lat": 30.1328,
            "lng": -97.6411,
            "alt": 161,
            "url": "http://en.wikipedia.org/wiki/Circuit_of_the_Americas"
        }
    },
    {
        "season": 2024,
        "round": 20,
        "name": "Mexico City Grand Prix",
        "date": "2024-10-27",
        "time": "20:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Mexico_City_Grand_Prix",
        "fp1": {
            "date": "2024-10-25",
            "time": "18:30:00"
        },
        "fp2": {
            "date": "2024-10-25",
            "time": "22:00:00"
        },
        "fp3": {
            "date": "2024-10-26",
            "time": "17:30:00"
        },
        "quali": {
            "date": "2024-10-26",
            "time": "21:00:00"
        },
        "circuit": {
            "circuit_ref": "rodriguez",
            "name": "Autódromo Hermanos Rodríguez",
            "location": "Mexico City",
            "country": "Mexico",
            "lat": 19.4042,
            "lng": -99.0907,
            "alt": 2227,
            "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Hermanos_Rodr%C3%ADguez"
        }
    },
    {
        "season": 2024,
        "round": 21,
        "name": "São Paulo Grand Prix",
        "date": "2024-11-03",
        "time": "17:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_S%C3%A3o_Paulo_Grand_Prix",
        "fp1": {
            "date": "2024-11-01",
            "time": "14:30:00"
        },
        "fp2": {
            "date": "2024-11-01",
            "time": "18:30:00"
        },
        "quali": {
            "date": "2024-11-02",
            "time": "18:00:00"
        },
        "sprint": {
            "date": "2024-11-02",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "interlagos",
            "name": "Autódromo José Carlos Pace",
            "location": "São Paulo",
            "country": "Brazil",
            "lat": -23.7036,
            "lng": -46.6997,
            "alt": 785,
            "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Jos%C3%A9_Carlos_Pace"
        }
    },
    {
        "season": 2024,
        "round": 22,
        "name": "Las Vegas Grand Prix",
        "date": "2024-11-23",
        "time": "06:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Las_Vegas_Grand_Prix",
        "fp1": {
            "date": "2024-11-21",
            "time": "02:30:00"
        },
        "fp2": {
            "date": "2024-11-21",
            "time": "06:00:00"
        },
        "fp3": {
            "date": "2024-11-22",
            "time": "02:30:00"
        },
        "quali": {
            "date": "2024-11-22",
            "time": "06:00:00"
        },
        "circuit": {
            "circuit_ref": "vegas",
            "name": "Las Vegas Strip Street Circuit",
            "location": "Las Vegas",
            "country": "United States",
            "lat": 36.1147,
            "lng": -115.173,
            "alt": 642,
            "url": "https://en.wikipedia.org/wiki/Las_Vegas_Grand_Prix#Circuit"
        }
    },
    {
        "season": 2024,
        "round": 23,
        "name": "Qatar Grand Prix",
        "date": "2024-12-01",
        "time": "17:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Qatar_Grand_Prix",
        "fp1": {
            "date": "2024-11-29",
            "time": "13:30:00"
        },
        "fp2": {
            "date": "2024-11-29",
            "time": "17:30:00"
        },
        "quali": {
            "date": "2024-11-30",
            "time": "17:00:00"
        },
        "sprint": {
            "date": "2024-11-30",
            "time": "13:00:00"
        },
        "circuit": {
            "circuit_ref": "losail",
            "name": "Losail International Circuit",
            "location": "Al Daayen",
            "country": "Qatar",
            "lat": 25.49,
            "lng": 51.4542,
            "alt": 12,
            "url": "http://en.wikipedia.org/wiki/Losail_International_Circuit"
        }
    },
    {
        "season": 2024,
        "round": 24,
        "name": "Abu Dhabi Grand Prix",
        "date": "2024-12-08",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Abu_Dhabi_Grand_Prix",
        "fp1": {
            "date": "2024-12-06",
            "time": "09:30:00"
        },
        "fp2": {
            "date": "2024-12-06",
            "time": "13:00:00"
        },
        "fp3": {
            "date": "2024-12-07",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2024-12-07",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "yas_marina",
            "name": "Yas Marina Circuit",
            "location": "Abu Dhabi",
            "country": "UAE",
            "lat": 24.4672,
            "lng": 54.6031,
            "alt": 3,
            "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit"
        }
    }
];

const BAHRAIN_2024_ROUND_1: [StaticRace; 1] = races_from_json![
    {
        "season": 2024,
        "round": 1,
        "name": "Bahrain Grand Prix",
        "date": "2024-03-02",
        "time": "15:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Bahrain_Grand_Prix",
        "fp1": {
            "date": "2024-02-29",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2024-02-29",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2024-03-01",
            "time": "12:30:00"
        },
        "quali": {
            "date": "2024-03-01",
            "time": "16:00:00"
        },
        "circuit": {
            "circuit_ref": "bahrain",
            "name": "Bahrain International Circuit",
            "location": "Sakhir",
            "country": "Bahrain",
            "lat": 26.0325,
            "lng": 50.5106,
            "alt": 7,
            "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
        }
    }
];

const LECLERC_RACES_PAGE_4: [StaticRace; 30] = races_from_json![
    {
        "season": 2022,
        "round": 10,
        "name": "British Grand Prix",
        "date": "2022-07-03",
        "time": "14:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_British_Grand_Prix",
        "fp1": {
            "date": "2022-07-01",
            "time": "12:00:00"
        },
        "fp2": {
            "date": "2022-07-01",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2022-07-02",
            "time": "11:00:00"
        },
        "quali": {
            "date": "2022-07-02",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "silverstone",
            "name": "Silverstone Circuit",
            "location": "Silverstone",
            "country": "UK",
            "lat": 52.0786,
            "lng": -1.01694,
            "alt": 153,
            "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit"
        }
    },
    {
        "season": 2022,
        "round": 11,
        "name": "Austrian Grand Prix",
        "date": "2022-07-10",
        "time": "13:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_Austrian_Grand_Prix",
        "fp1": {
            "date": "2022-07-08",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2022-07-09",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2022-07-08",
            "time": "15:00:00"
        },
        "sprint": {
            "date": "2022-07-09",
            "time": "14:30:00"
        },
        "circuit": {
            "circuit_ref": "red_bull_ring",
            "name": "Red Bull Ring",
            "location": "Spielberg",
            "country": "Austria",
            "lat": 47.2197,
            "lng": 14.7647,
            "alt": 678,
            "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring"
        }
    },
    {
        "season": 2022,
        "round": 12,
        "name": "French Grand Prix",
        "date": "2022-07-24",
        "time": "13:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_French_Grand_Prix",
        "fp1": {
            "date": "2022-07-22",
            "time": "12:00:00"
        },
        "fp2": {
            "date": "2022-07-22",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2022-07-23",
            "time": "11:00:00"
        },
        "quali": {
            "date": "2022-07-23",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "ricard",
            "name": "Circuit Paul Ricard",
            "location": "Le Castellet",
            "country": "France",
            "lat": 43.2506,
            "lng": 5.79167,
            "alt": 432,
            "url": "http://en.wikipedia.org/wiki/Paul_Ricard_Circuit"
        }
    },
    {
        "season": 2022,
        "round": 13,
        "name": "Hungarian Grand Prix",
        "date": "2022-07-31",
        "time": "13:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_Hungarian_Grand_Prix",
        "fp1": {
            "date": "2022-07-29",
            "time": "12:00:00"
        },
        "fp2": {
            "date": "2022-07-29",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2022-07-30",
            "time": "11:00:00"
        },
        "quali": {
            "date": "2022-07-30",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "hungaroring",
            "name": "Hungaroring",
            "location": "Budapest",
            "country": "Hungary",
            "lat": 47.5789,
            "lng": 19.2486,
            "alt": 264,
            "url": "http://en.wikipedia.org/wiki/Hungaroring"
        }
    },
    {
        "season": 2022,
        "round": 14,
        "name": "Belgian Grand Prix",
        "date": "2022-08-28",
        "time": "13:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_Belgian_Grand_Prix",
        "fp1": {
            "date": "2022-08-26",
            "time": "12:00:00"
        },
        "fp2": {
            "date": "2022-08-26",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2022-08-27",
            "time": "11:00:00"
        },
        "quali": {
            "date": "2022-08-27",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "spa",
            "name": "Circuit de Spa-Francorchamps",
            "location": "Spa",
            "country": "Belgium",
            "lat": 50.4372,
            "lng": 5.97139,
            "alt": 401,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
        }
    },
    {
        "season": 2022,
        "round": 15,
        "name": "Dutch Grand Prix",
        "date": "2022-09-04",
        "time": "13:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_Dutch_Grand_Prix",
        "fp1": {
            "date": "2022-09-02",
            "time": "10:30:00"
        },
        "fp2": {
            "date": "2022-09-02",
            "time": "14:00:00"
        },
        "fp3": {
            "date": "2022-09-03",
            "time": "10:00:00"
        },
        "quali": {
            "date": "2022-09-03",
            "time": "13:00:00"
        },
        "circuit": {
            "circuit_ref": "zandvoort",
            "name": "Circuit Park Zandvoort",
            "location": "Zandvoort",
            "country": "Netherlands",
            "lat": 52.3888,
            "lng": 4.54092,
            "alt": 6,
            "url": "http://en.wikipedia.org/wiki/Circuit_Zandvoort"
        }
    },
    {
        "season": 2022,
        "round": 16,
        "name": "Italian Grand Prix",
        "date": "2022-09-11",
        "time": "13:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_Italian_Grand_Prix",
        "fp1": {
            "date": "2022-09-09",
            "time": "12:00:00"
        },
        "fp2": {
            "date": "2022-09-09",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2022-09-10",
            "time": "11:00:00"
        },
        "quali": {
            "date": "2022-09-10",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "monza",
            "name": "Autodromo Nazionale di Monza",
            "location": "Monza",
            "country": "Italy",
            "lat": 45.6156,
            "lng": 9.28111,
            "alt": 162,
            "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
        }
    },
    {
        "season": 2022,
        "round": 17,
        "name": "Singapore Grand Prix",
        "date": "2022-10-02",
        "time": "12:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_Singapore_Grand_Prix",
        "fp1": {
            "date": "2022-09-30",
            "time": "10:00:00"
        },
        "fp2": {
            "date": "2022-09-30",
            "time": "13:00:00"
        },
        "fp3": {
            "date": "2022-10-01",
            "time": "10:00:00"
        },
        "quali": {
            "date": "2022-10-01",
            "time": "13:00:00"
        },
        "circuit": {
            "circuit_ref": "marina_bay",
            "name": "Marina Bay Street Circuit",
            "location": "Marina Bay",
            "country": "Singapore",
            "lat": 1.2914,
            "lng": 103.864,
            "alt": 18,
            "url": "http://en.wikipedia.org/wiki/Marina_Bay_Street_Circuit"
        }
    },
    {
        "season": 2022,
        "round": 18,
        "name": "Japanese Grand Prix",
        "date": "2022-10-09",
        "time": "05:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_Japanese_Grand_Prix",
        "fp1": {
            "date": "2022-10-07",
            "time": "03:00:00"
        },
        "fp2": {
            "date": "2022-10-07",
            "time": "06:00:00"
        },
        "fp3": {
            "date": "2022-10-08",
            "time": "03:00:00"
        },
        "quali": {
            "date": "2022-10-08",
            "time": "06:00:00"
        },
        "circuit": {
            "circuit_ref": "suzuka",
            "name": "Suzuka Circuit",
            "location": "Suzuka",
            "country": "Japan",
            "lat": 34.8431,
            "lng": 136.541,
            "alt": 45,
            "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit"
        }
    },
    {
        "season": 2022,
        "round": 19,
        "name": "United States Grand Prix",
        "date": "2022-10-23",
        "time": "19:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_United_States_Grand_Prix",
        "fp1": {
            "date": "2022-10-21",
            "time": "19:00:00"
        },
        "fp2": {
            "date": "2022-10-21",
            "time": "22:00:00"
        },
        "fp3": {
            "date": "2022-10-22",
            "time": "19:00:00"
        },
        "quali": {
            "date": "2022-10-22",
            "time": "22:00:00"
        },
        "circuit": {
            "circuit_ref": "americas",
            "name": "Circuit of the Americas",
            "location": "Austin",
            "country": "USA",
            "lat": 30.1328,
            "lng": -97.6411,
            "alt": 161,
            "url": "http://en.wikipedia.org/wiki/Circuit_of_the_Americas"
        }
    },
    {
        "season": 2022,
        "round": 20,
        "name": "Mexico City Grand Prix",
        "date": "2022-10-30",
        "time": "20:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_Mexican_Grand_Prix",
        "fp1": {
            "date": "2022-10-28",
            "time": "18:00:00"
        },
        "fp2": {
            "date": "2022-10-28",
            "time": "21:00:00"
        },
        "fp3": {
            "date": "2022-10-29",
            "time": "17:00:00"
        },
        "quali": {
            "date": "2022-10-29",
            "time": "20:00:00"
        },
        "circuit": {
            "circuit_ref": "rodriguez",
            "name": "Autódromo Hermanos Rodríguez",
            "location": "Mexico City",
            "country": "Mexico",
            "lat": 19.4042,
            "lng": -99.0907,
            "alt": 2227,
            "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Hermanos_Rodr%C3%ADguez"
        }
    },
    {
        "season": 2022,
        "round": 21,
        "name": "São Paulo Grand Prix",
        "date": "2022-11-13",
        "time": "18:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_Brazilian_Grand_Prix",
        "fp1": {
            "date": "2022-11-11",
            "time": "15:30:00"
        },
        "fp2": {
            "date": "2022-11-12",
            "time": "15:30:00"
        },
        "quali": {
            "date": "2022-11-11",
            "time": "19:00:00"
        },
        "sprint": {
            "date": "2022-11-12",
            "time": "19:30:00"
        },
        "circuit": {
            "circuit_ref": "interlagos",
            "name": "Autódromo José Carlos Pace",
            "location": "São Paulo",
            "country": "Brazil",
            "lat": -23.7036,
            "lng": -46.6997,
            "alt": 785,
            "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Jos%C3%A9_Carlos_Pace"
        }
    },
    {
        "season": 2022,
        "round": 22,
        "name": "Abu Dhabi Grand Prix",
        "date": "2022-11-20",
        "time": "13:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_Abu_Dhabi_Grand_Prix",
        "fp1": {
            "date": "2022-11-18",
            "time": "10:00:00"
        },
        "fp2": {
            "date": "2022-11-18",
            "time": "13:00:00"
        },
        "fp3": {
            "date": "2022-11-19",
            "time": "11:00:00"
        },
        "quali": {
            "date": "2022-11-19",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "yas_marina",
            "name": "Yas Marina Circuit",
            "location": "Abu Dhabi",
            "country": "UAE",
            "lat": 24.4672,
            "lng": 54.6031,
            "alt": 3,
            "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 1,
        "name": "Bahrain Grand Prix",
        "date": "2023-03-05",
        "time": "15:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Bahrain_Grand_Prix",
        "fp1": {
            "date": "2023-03-03",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-03-03",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2023-03-04",
            "time": "11:30:00"
        },
        "quali": {
            "date": "2023-03-04",
            "time": "15:00:00"
        },
        "circuit": {
            "circuit_ref": "bahrain",
            "name": "Bahrain International Circuit",
            "location": "Sakhir",
            "country": "Bahrain",
            "lat": 26.0325,
            "lng": 50.5106,
            "alt": 7,
            "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 2,
        "name": "Saudi Arabian Grand Prix",
        "date": "2023-03-19",
        "time": "17:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Saudi_Arabian_Grand_Prix",
        "fp1": {
            "date": "2023-03-17",
            "time": "13:30:00"
        },
        "fp2": {
            "date": "2023-03-17",
            "time": "17:00:00"
        },
        "fp3": {
            "date": "2023-03-18",
            "time": "13:30:00"
        },
        "quali": {
            "date": "2023-03-18",
            "time": "17:00:00"
        },
        "circuit": {
            "circuit_ref": "jeddah",
            "name": "Jeddah Corniche Circuit",
            "location": "Jeddah",
            "country": "Saudi Arabia",
            "lat": 21.6319,
            "lng": 39.1044,
            "alt": 15,
            "url": "http://en.wikipedia.org/wiki/Jeddah_Street_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 3,
        "name": "Australian Grand Prix",
        "date": "2023-04-02",
        "time": "05:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Australian_Grand_Prix",
        "fp1": {
            "date": "2023-03-31",
            "time": "01:30:00"
        },
        "fp2": {
            "date": "2023-03-31",
            "time": "05:00:00"
        },
        "fp3": {
            "date": "2023-04-01",
            "time": "01:30:00"
        },
        "quali": {
            "date": "2023-04-01",
            "time": "05:00:00"
        },
        "circuit": {
            "circuit_ref": "albert_park",
            "name": "Albert Park Grand Prix Circuit",
            "location": "Melbourne",
            "country": "Australia",
            "lat": -37.8497,
            "lng": 144.968,
            "alt": 10,
            "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 4,
        "name": "Azerbaijan Grand Prix",
        "date": "2023-04-30",
        "time": "11:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Azerbaijan_Grand_Prix",
        "fp1": {
            "date": "2023-04-28",
            "time": "09:30:00"
        },
        "fp2": {
            "date": "2023-04-29",
            "time": "09:30:00"
        },
        "quali": {
            "date": "2023-04-28",
            "time": "13:00:00"
        },
        "sprint": {
            "date": "2023-04-29",
            "time": "13:30:00"
        },
        "circuit": {
            "circuit_ref": "baku",
            "name": "Baku City Circuit",
            "location": "Baku",
            "country": "Azerbaijan",
            "lat": 40.3725,
            "lng": 49.8533,
            "alt": -7,
            "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 5,
        "name": "Miami Grand Prix",
        "date": "2023-05-07",
        "time": "19:30:00",
        "url": "https://en.wikipedia.org/wiki/2023_Miami_Grand_Prix",
        "fp1": {
            "date": "2023-05-05",
            "time": "18:00:00"
        },
        "fp2": {
            "date": "2023-05-05",
            "time": "21:30:00"
        },
        "fp3": {
            "date": "2023-05-06",
            "time": "16:30:00"
        },
        "quali": {
            "date": "2023-05-06",
            "time": "20:00:00"
        },
        "circuit": {
            "circuit_ref": "miami",
            "name": "Miami International Autodrome",
            "location": "Miami",
            "country": "USA",
            "lat": 25.9581,
            "lng": -80.2389,
            "alt": 0,
            "url": "http://en.wikipedia.org/wiki/Miami_International_Autodrome"
        }
    },
    {
        "season": 2023,
        "round": 6,
        "name": "Monaco Grand Prix",
        "date": "2023-05-28",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Monaco_Grand_Prix",
        "fp1": {
            "date": "2023-05-26",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-05-26",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2023-05-27",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-05-27",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "monaco",
            "name": "Circuit de Monaco",
            "location": "Monte-Carlo",
            "country": "Monaco",
            "lat": 43.7347,
            "lng": 7.42056,
            "alt": 7,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco"
        }
    },
    {
        "season": 2023,
        "round": 7,
        "name": "Spanish Grand Prix",
        "date": "2023-06-04",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Spanish_Grand_Prix",
        "fp1": {
            "date": "2023-06-02",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-06-02",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2023-06-03",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-06-03",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "catalunya",
            "name": "Circuit de Barcelona-Catalunya",
            "location": "Montmeló",
            "country": "Spain",
            "lat": 41.57,
            "lng": 2.26111,
            "alt": 109,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Barcelona-Catalunya"
        }
    },
    {
        "season": 2023,
        "round": 8,
        "name": "Canadian Grand Prix",
        "date": "2023-06-18",
        "time": "18:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Canadian_Grand_Prix",
        "fp1": {
            "date": "2023-06-16",
            "time": "17:30:00"
        },
        "fp2": {
            "date": "2023-06-16",
            "time": "21:00:00"
        },
        "fp3": {
            "date": "2023-06-17",
            "time": "16:30:00"
        },
        "quali": {
            "date": "2023-06-17",
            "time": "20:00:00"
        },
        "circuit": {
            "circuit_ref": "villeneuve",
            "name": "Circuit Gilles Villeneuve",
            "location": "Montreal",
            "country": "Canada",
            "lat": 45.5,
            "lng": -73.5228,
            "alt": 13,
            "url": "http://en.wikipedia.org/wiki/Circuit_Gilles_Villeneuve"
        }
    },
    {
        "season": 2023,
        "round": 9,
        "name": "Austrian Grand Prix",
        "date": "2023-07-02",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Austrian_Grand_Prix",
        "fp1": {
            "date": "2023-06-30",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-07-01",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-06-30",
            "time": "15:00:00"
        },
        "sprint": {
            "date": "2023-07-01",
            "time": "14:30:00"
        },
        "circuit": {
            "circuit_ref": "red_bull_ring",
            "name": "Red Bull Ring",
            "location": "Spielberg",
            "country": "Austria",
            "lat": 47.2197,
            "lng": 14.7647,
            "alt": 678,
            "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring"
        }
    },
    {
        "season": 2023,
        "round": 10,
        "name": "British Grand Prix",
        "date": "2023-07-09",
        "time": "14:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_British_Grand_Prix",
        "fp1": {
            "date": "2023-07-07",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-07-07",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2023-07-08",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-07-08",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "silverstone",
            "name": "Silverstone Circuit",
            "location": "Silverstone",
            "country": "UK",
            "lat": 52.0786,
            "lng": -1.01694,
            "alt": 153,
            "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 11,
        "name": "Hungarian Grand Prix",
        "date": "2023-07-23",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Hungarian_Grand_Prix",
        "fp1": {
            "date": "2023-07-21",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-07-21",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2023-07-22",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-07-22",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "hungaroring",
            "name": "Hungaroring",
            "location": "Budapest",
            "country": "Hungary",
            "lat": 47.5789,
            "lng": 19.2486,
            "alt": 264,
            "url": "http://en.wikipedia.org/wiki/Hungaroring"
        }
    },
    {
        "season": 2023,
        "round": 12,
        "name": "Belgian Grand Prix",
        "date": "2023-07-30",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Belgian_Grand_Prix",
        "fp1": {
            "date": "2023-07-28",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-07-29",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-07-28",
            "time": "15:00:00"
        },
        "sprint": {
            "date": "2023-07-29",
            "time": "14:30:00"
        },
        "circuit": {
            "circuit_ref": "spa",
            "name": "Circuit de Spa-Francorchamps",
            "location": "Spa",
            "country": "Belgium",
            "lat": 50.4372,
            "lng": 5.97139,
            "alt": 401,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
        }
    },
    {
        "season": 2023,
        "round": 13,
        "name": "Dutch Grand Prix",
        "date": "2023-08-27",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Dutch_Grand_Prix",
        "fp1": {
            "date": "2023-08-25",
            "time": "10:30:00"
        },
        "fp2": {
            "date": "2023-08-25",
            "time": "14:00:00"
        },
        "fp3": {
            "date": "2023-08-26",
            "time": "09:30:00"
        },
        "quali": {
            "date": "2023-08-26",
            "time": "13:00:00"
        },
        "circuit": {
            "circuit_ref": "zandvoort",
            "name": "Circuit Park Zandvoort",
            "location": "Zandvoort",
            "country": "Netherlands",
            "lat": 52.3888,
            "lng": 4.54092,
            "alt": 6,
            "url": "http://en.wikipedia.org/wiki/Circuit_Zandvoort"
        }
    },
    {
        "season": 2023,
        "round": 14,
        "name": "Italian Grand Prix",
        "date": "2023-09-03",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Italian_Grand_Prix",
        "fp1": {
            "date": "2023-09-01",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-09-01",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2023-09-02",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-09-02",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "monza",
            "name": "Autodromo Nazionale di Monza",
            "location": "Monza",
            "country": "Italy",
            "lat": 45.6156,
            "lng": 9.28111,
            "alt": 162,
            "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
        }
    },
    {
        "season": 2023,
        "round": 15,
        "name": "Singapore Grand Prix",
        "date": "2023-09-17",
        "time": "12:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Singapore_Grand_Prix",
        "fp1": {
            "date": "2023-09-15",
            "time": "09:30:00"
        },
        "fp2": {
            "date": "2023-09-15",
            "time": "13:00:00"
        },
        "fp3": {
            "date": "2023-09-16",
            "time": "09:30:00"
        },
        "quali": {
            "date": "2023-09-16",
            "time": "13:00:00"
        },
        "circuit": {
            "circuit_ref": "marina_bay",
            "name": "Marina Bay Street Circuit",
            "location": "Marina Bay",
            "country": "Singapore",
            "lat": 1.2914,
            "lng": 103.864,
            "alt": 18,
            "url": "http://en.wikipedia.org/wiki/Marina_Bay_Street_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 16,
        "name": "Japanese Grand Prix",
        "date": "2023-09-24",
        "time": "05:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Japanese_Grand_Prix",
        "fp1": {
            "date": "2023-09-22",
            "time": "02:30:00"
        },
        "fp2": {
            "date": "2023-09-22",
            "time": "06:00:00"
        },
        "fp3": {
            "date": "2023-09-23",
            "time": "02:30:00"
        },
        "quali": {
            "date": "2023-09-23",
            "time": "06:00:00"
        },
        "circuit": {
            "circuit_ref": "suzuka",
            "name": "Suzuka Circuit",
            "location": "Suzuka",
            "country": "Japan",
            "lat": 34.8431,
            "lng": 136.541,
            "alt": 45,
            "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 17,
        "name": "Qatar Grand Prix",
        "date": "2023-10-08",
        "time": "17:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Qatar_Grand_Prix",
        "fp1": {
            "date": "2023-10-06",
            "time": "13:30:00"
        },
        "fp2": {
            "date": "2023-10-07",
            "time": "13:00:00"
        },
        "quali": {
            "date": "2023-10-06",
            "time": "17:00:00"
        },
        "sprint": {
            "date": "2023-10-07",
            "time": "17:30:00"
        },
        "circuit": {
            "circuit_ref": "losail",
            "name": "Losail International Circuit",
            "location": "Al Daayen",
            "country": "Qatar",
            "lat": 25.49,
            "lng": 51.4542,
            "alt": 12,
            "url": "http://en.wikipedia.org/wiki/Losail_International_Circuit"
        }
    }
];

const FERRARI_RACES_PAGE_36: [StaticRace; 30] = races_from_json![
    {
        "season": 2022,
        "round": 19,
        "name": "United States Grand Prix",
        "date": "2022-10-23",
        "time": "19:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_United_States_Grand_Prix",
        "fp1": {
            "date": "2022-10-21",
            "time": "19:00:00"
        },
        "fp2": {
            "date": "2022-10-21",
            "time": "22:00:00"
        },
        "fp3": {
            "date": "2022-10-22",
            "time": "19:00:00"
        },
        "quali": {
            "date": "2022-10-22",
            "time": "22:00:00"
        },
        "circuit": {
            "circuit_ref": "americas",
            "name": "Circuit of the Americas",
            "location": "Austin",
            "country": "USA",
            "lat": 30.1328,
            "lng": -97.6411,
            "alt": 161,
            "url": "http://en.wikipedia.org/wiki/Circuit_of_the_Americas"
        }
    },
    {
        "season": 2022,
        "round": 20,
        "name": "Mexico City Grand Prix",
        "date": "2022-10-30",
        "time": "20:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_Mexican_Grand_Prix",
        "fp1": {
            "date": "2022-10-28",
            "time": "18:00:00"
        },
        "fp2": {
            "date": "2022-10-28",
            "time": "21:00:00"
        },
        "fp3": {
            "date": "2022-10-29",
            "time": "17:00:00"
        },
        "quali": {
            "date": "2022-10-29",
            "time": "20:00:00"
        },
        "circuit": {
            "circuit_ref": "rodriguez",
            "name": "Autódromo Hermanos Rodríguez",
            "location": "Mexico City",
            "country": "Mexico",
            "lat": 19.4042,
            "lng": -99.0907,
            "alt": 2227,
            "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Hermanos_Rodr%C3%ADguez"
        }
    },
    {
        "season": 2022,
        "round": 21,
        "name": "São Paulo Grand Prix",
        "date": "2022-11-13",
        "time": "18:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_Brazilian_Grand_Prix",
        "fp1": {
            "date": "2022-11-11",
            "time": "15:30:00"
        },
        "fp2": {
            "date": "2022-11-12",
            "time": "15:30:00"
        },
        "quali": {
            "date": "2022-11-11",
            "time": "19:00:00"
        },
        "sprint": {
            "date": "2022-11-12",
            "time": "19:30:00"
        },
        "circuit": {
            "circuit_ref": "interlagos",
            "name": "Autódromo José Carlos Pace",
            "location": "São Paulo",
            "country": "Brazil",
            "lat": -23.7036,
            "lng": -46.6997,
            "alt": 785,
            "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Jos%C3%A9_Carlos_Pace"
        }
    },
    {
        "season": 2022,
        "round": 22,
        "name": "Abu Dhabi Grand Prix",
        "date": "2022-11-20",
        "time": "13:00:00",
        "url": "http://en.wikipedia.org/wiki/2022_Abu_Dhabi_Grand_Prix",
        "fp1": {
            "date": "2022-11-18",
            "time": "10:00:00"
        },
        "fp2": {
            "date": "2022-11-18",
            "time": "13:00:00"
        },
        "fp3": {
            "date": "2022-11-19",
            "time": "11:00:00"
        },
        "quali": {
            "date": "2022-11-19",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "yas_marina",
            "name": "Yas Marina Circuit",
            "location": "Abu Dhabi",
            "country": "UAE",
            "lat": 24.4672,
            "lng": 54.6031,
            "alt": 3,
            "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 1,
        "name": "Bahrain Grand Prix",
        "date": "2023-03-05",
        "time": "15:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Bahrain_Grand_Prix",
        "fp1": {
            "date": "2023-03-03",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-03-03",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2023-03-04",
            "time": "11:30:00"
        },
        "quali": {
            "date": "2023-03-04",
            "time": "15:00:00"
        },
        "circuit": {
            "circuit_ref": "bahrain",
            "name": "Bahrain International Circuit",
            "location": "Sakhir",
            "country": "Bahrain",
            "lat": 26.0325,
            "lng": 50.5106,
            "alt": 7,
            "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 2,
        "name": "Saudi Arabian Grand Prix",
        "date": "2023-03-19",
        "time": "17:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Saudi_Arabian_Grand_Prix",
        "fp1": {
            "date": "2023-03-17",
            "time": "13:30:00"
        },
        "fp2": {
            "date": "2023-03-17",
            "time": "17:00:00"
        },
        "fp3": {
            "date": "2023-03-18",
            "time": "13:30:00"
        },
        "quali": {
            "date": "2023-03-18",
            "time": "17:00:00"
        },
        "circuit": {
            "circuit_ref": "jeddah",
            "name": "Jeddah Corniche Circuit",
            "location": "Jeddah",
            "country": "Saudi Arabia",
            "lat": 21.6319,
            "lng": 39.1044,
            "alt": 15,
            "url": "http://en.wikipedia.org/wiki/Jeddah_Street_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 3,
        "name": "Australian Grand Prix",
        "date": "2023-04-02",
        "time": "05:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Australian_Grand_Prix",
        "fp1": {
            "date": "2023-03-31",
            "time": "01:30:00"
        },
        "fp2": {
            "date": "2023-03-31",
            "time": "05:00:00"
        },
        "fp3": {
            "date": "2023-04-01",
            "time": "01:30:00"
        },
        "quali": {
            "date": "2023-04-01",
            "time": "05:00:00"
        },
        "circuit": {
            "circuit_ref": "albert_park",
            "name": "Albert Park Grand Prix Circuit",
            "location": "Melbourne",
            "country": "Australia",
            "lat": -37.8497,
            "lng": 144.968,
            "alt": 10,
            "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 4,
        "name": "Azerbaijan Grand Prix",
        "date": "2023-04-30",
        "time": "11:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Azerbaijan_Grand_Prix",
        "fp1": {
            "date": "2023-04-28",
            "time": "09:30:00"
        },
        "fp2": {
            "date": "2023-04-29",
            "time": "09:30:00"
        },
        "quali": {
            "date": "2023-04-28",
            "time": "13:00:00"
        },
        "sprint": {
            "date": "2023-04-29",
            "time": "13:30:00"
        },
        "circuit": {
            "circuit_ref": "baku",
            "name": "Baku City Circuit",
            "location": "Baku",
            "country": "Azerbaijan",
            "lat": 40.3725,
            "lng": 49.8533,
            "alt": -7,
            "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 5,
        "name": "Miami Grand Prix",
        "date": "2023-05-07",
        "time": "19:30:00",
        "url": "https://en.wikipedia.org/wiki/2023_Miami_Grand_Prix",
        "fp1": {
            "date": "2023-05-05",
            "time": "18:00:00"
        },
        "fp2": {
            "date": "2023-05-05",
            "time": "21:30:00"
        },
        "fp3": {
            "date": "2023-05-06",
            "time": "16:30:00"
        },
        "quali": {
            "date": "2023-05-06",
            "time": "20:00:00"
        },
        "circuit": {
            "circuit_ref": "miami",
            "name": "Miami International Autodrome",
            "location": "Miami",
            "country": "USA",
            "lat": 25.9581,
            "lng": -80.2389,
            "alt": 0,
            "url": "http://en.wikipedia.org/wiki/Miami_International_Autodrome"
        }
    },
    {
        "season": 2023,
        "round": 6,
        "name": "Monaco Grand Prix",
        "date": "2023-05-28",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Monaco_Grand_Prix",
        "fp1": {
            "date": "2023-05-26",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-05-26",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2023-05-27",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-05-27",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "monaco",
            "name": "Circuit de Monaco",
            "location": "Monte-Carlo",
            "country": "Monaco",
            "lat": 43.7347,
            "lng": 7.42056,
            "alt": 7,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco"
        }
    },
    {
        "season": 2023,
        "round": 7,
        "name": "Spanish Grand Prix",
        "date": "2023-06-04",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Spanish_Grand_Prix",
        "fp1": {
            "date": "2023-06-02",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-06-02",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2023-06-03",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-06-03",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "catalunya",
            "name": "Circuit de Barcelona-Catalunya",
            "location": "Montmeló",
            "country": "Spain",
            "lat": 41.57,
            "lng": 2.26111,
            "alt": 109,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Barcelona-Catalunya"
        }
    },
    {
        "season": 2023,
        "round": 8,
        "name": "Canadian Grand Prix",
        "date": "2023-06-18",
        "time": "18:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Canadian_Grand_Prix",
        "fp1": {
            "date": "2023-06-16",
            "time": "17:30:00"
        },
        "fp2": {
            "date": "2023-06-16",
            "time": "21:00:00"
        },
        "fp3": {
            "date": "2023-06-17",
            "time": "16:30:00"
        },
        "quali": {
            "date": "2023-06-17",
            "time": "20:00:00"
        },
        "circuit": {
            "circuit_ref": "villeneuve",
            "name": "Circuit Gilles Villeneuve",
            "location": "Montreal",
            "country": "Canada",
            "lat": 45.5,
            "lng": -73.5228,
            "alt": 13,
            "url": "http://en.wikipedia.org/wiki/Circuit_Gilles_Villeneuve"
        }
    },
    {
        "season": 2023,
        "round": 9,
        "name": "Austrian Grand Prix",
        "date": "2023-07-02",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Austrian_Grand_Prix",
        "fp1": {
            "date": "2023-06-30",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-07-01",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-06-30",
            "time": "15:00:00"
        },
        "sprint": {
            "date": "2023-07-01",
            "time": "14:30:00"
        },
        "circuit": {
            "circuit_ref": "red_bull_ring",
            "name": "Red Bull Ring",
            "location": "Spielberg",
            "country": "Austria",
            "lat": 47.2197,
            "lng": 14.7647,
            "alt": 678,
            "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring"
        }
    },
    {
        "season": 2023,
        "round": 10,
        "name": "British Grand Prix",
        "date": "2023-07-09",
        "time": "14:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_British_Grand_Prix",
        "fp1": {
            "date": "2023-07-07",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-07-07",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2023-07-08",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-07-08",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "silverstone",
            "name": "Silverstone Circuit",
            "location": "Silverstone",
            "country": "UK",
            "lat": 52.0786,
            "lng": -1.01694,
            "alt": 153,
            "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 11,
        "name": "Hungarian Grand Prix",
        "date": "2023-07-23",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Hungarian_Grand_Prix",
        "fp1": {
            "date": "2023-07-21",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-07-21",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2023-07-22",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-07-22",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "hungaroring",
            "name": "Hungaroring",
            "location": "Budapest",
            "country": "Hungary",
            "lat": 47.5789,
            "lng": 19.2486,
            "alt": 264,
            "url": "http://en.wikipedia.org/wiki/Hungaroring"
        }
    },
    {
        "season": 2023,
        "round": 12,
        "name": "Belgian Grand Prix",
        "date": "2023-07-30",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Belgian_Grand_Prix",
        "fp1": {
            "date": "2023-07-28",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-07-29",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-07-28",
            "time": "15:00:00"
        },
        "sprint": {
            "date": "2023-07-29",
            "time": "14:30:00"
        },
        "circuit": {
            "circuit_ref": "spa",
            "name": "Circuit de Spa-Francorchamps",
            "location": "Spa",
            "country": "Belgium",
            "lat": 50.4372,
            "lng": 5.97139,
            "alt": 401,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
        }
    },
    {
        "season": 2023,
        "round": 13,
        "name": "Dutch Grand Prix",
        "date": "2023-08-27",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Dutch_Grand_Prix",
        "fp1": {
            "date": "2023-08-25",
            "time": "10:30:00"
        },
        "fp2": {
            "date": "2023-08-25",
            "time": "14:00:00"
        },
        "fp3": {
            "date": "2023-08-26",
            "time": "09:30:00"
        },
        "quali": {
            "date": "2023-08-26",
            "time": "13:00:00"
        },
        "circuit": {
            "circuit_ref": "zandvoort",
            "name": "Circuit Park Zandvoort",
            "location": "Zandvoort",
            "country": "Netherlands",
            "lat": 52.3888,
            "lng": 4.54092,
            "alt": 6,
            "url": "http://en.wikipedia.org/wiki/Circuit_Zandvoort"
        }
    },
    {
        "season": 2023,
        "round": 14,
        "name": "Italian Grand Prix",
        "date": "2023-09-03",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Italian_Grand_Prix",
        "fp1": {
            "date": "2023-09-01",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2023-09-01",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2023-09-02",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-09-02",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "monza",
            "name": "Autodromo Nazionale di Monza",
            "location": "Monza",
            "country": "Italy",
            "lat": 45.6156,
            "lng": 9.28111,
            "alt": 162,
            "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
        }
    },
    {
        "season": 2023,
        "round": 15,
        "name": "Singapore Grand Prix",
        "date": "2023-09-17",
        "time": "12:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Singapore_Grand_Prix",
        "fp1": {
            "date": "2023-09-15",
            "time": "09:30:00"
        },
        "fp2": {
            "date": "2023-09-15",
            "time": "13:00:00"
        },
        "fp3": {
            "date": "2023-09-16",
            "time": "09:30:00"
        },
        "quali": {
            "date": "2023-09-16",
            "time": "13:00:00"
        },
        "circuit": {
            "circuit_ref": "marina_bay",
            "name": "Marina Bay Street Circuit",
            "location": "Marina Bay",
            "country": "Singapore",
            "lat": 1.2914,
            "lng": 103.864,
            "alt": 18,
            "url": "http://en.wikipedia.org/wiki/Marina_Bay_Street_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 16,
        "name": "Japanese Grand Prix",
        "date": "2023-09-24",
        "time": "05:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Japanese_Grand_Prix",
        "fp1": {
            "date": "2023-09-22",
            "time": "02:30:00"
        },
        "fp2": {
            "date": "2023-09-22",
            "time": "06:00:00"
        },
        "fp3": {
            "date": "2023-09-23",
            "time": "02:30:00"
        },
        "quali": {
            "date": "2023-09-23",
            "time": "06:00:00"
        },
        "circuit": {
            "circuit_ref": "suzuka",
            "name": "Suzuka Circuit",
            "location": "Suzuka",
            "country": "Japan",
            "lat": 34.8431,
            "lng": 136.541,
            "alt": 45,
            "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 17,
        "name": "Qatar Grand Prix",
        "date": "2023-10-08",
        "time": "17:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Qatar_Grand_Prix",
        "fp1": {
            "date": "2023-10-06",
            "time": "13:30:00"
        },
        "fp2": {
            "date": "2023-10-07",
            "time": "13:00:00"
        },
        "quali": {
            "date": "2023-10-06",
            "time": "17:00:00"
        },
        "sprint": {
            "date": "2023-10-07",
            "time": "17:30:00"
        },
        "circuit": {
            "circuit_ref": "losail",
            "name": "Losail International Circuit",
            "location": "Al Daayen",
            "country": "Qatar",
            "lat": 25.49,
            "lng": 51.4542,
            "alt": 12,
            "url": "http://en.wikipedia.org/wiki/Losail_International_Circuit"
        }
    },
    {
        "season": 2023,
        "round": 18,
        "name": "United States Grand Prix",
        "date": "2023-10-22",
        "time": "19:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_United_States_Grand_Prix",
        "fp1": {
            "date": "2023-10-20",
            "time": "17:30:00"
        },
        "fp2": {
            "date": "2023-10-21",
            "time": "18:00:00"
        },
        "quali": {
            "date": "2023-10-20",
            "time": "21:00:00"
        },
        "sprint": {
            "date": "2023-10-21",
            "time": "22:00:00"
        },
        "circuit": {
            "circuit_ref": "americas",
            "name": "Circuit of the Americas",
            "location": "Austin",
            "country": "USA",
            "lat": 30.1328,
            "lng": -97.6411,
            "alt": 161,
            "url": "http://en.wikipedia.org/wiki/Circuit_of_the_Americas"
        }
    },
    {
        "season": 2023,
        "round": 19,
        "name": "Mexico City Grand Prix",
        "date": "2023-10-29",
        "time": "20:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Mexico_City_Grand_Prix",
        "fp1": {
            "date": "2023-10-27",
            "time": "18:30:00"
        },
        "fp2": {
            "date": "2023-10-27",
            "time": "22:00:00"
        },
        "fp3": {
            "date": "2023-10-28",
            "time": "17:30:00"
        },
        "quali": {
            "date": "2023-10-28",
            "time": "21:00:00"
        },
        "circuit": {
            "circuit_ref": "rodriguez",
            "name": "Autódromo Hermanos Rodríguez",
            "location": "Mexico City",
            "country": "Mexico",
            "lat": 19.4042,
            "lng": -99.0907,
            "alt": 2227,
            "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Hermanos_Rodr%C3%ADguez"
        }
    },
    {
        "season": 2023,
        "round": 20,
        "name": "São Paulo Grand Prix",
        "date": "2023-11-05",
        "time": "17:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_S%C3%A3o_Paulo_Grand_Prix",
        "fp1": {
            "date": "2023-11-03",
            "time": "14:30:00"
        },
        "fp2": {
            "date": "2023-11-04",
            "time": "14:30:00"
        },
        "quali": {
            "date": "2023-11-03",
            "time": "18:00:00"
        },
        "sprint": {
            "date": "2023-11-04",
            "time": "18:30:00"
        },
        "circuit": {
            "circuit_ref": "interlagos",
            "name": "Autódromo José Carlos Pace",
            "location": "São Paulo",
            "country": "Brazil",
            "lat": -23.7036,
            "lng": -46.6997,
            "alt": 785,
            "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Jos%C3%A9_Carlos_Pace"
        }
    },
    {
        "season": 2023,
        "round": 21,
        "name": "Las Vegas Grand Prix",
        "date": "2023-11-19",
        "time": "06:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Las_Vegas_Grand_Prix",
        "fp1": {
            "date": "2023-11-16",
            "time": "04:30:00"
        },
        "fp2": {
            "date": "2023-11-16",
            "time": "08:00:00"
        },
        "fp3": {
            "date": "2023-11-17",
            "time": "04:30:00"
        },
        "quali": {
            "date": "2023-11-17",
            "time": "08:00:00"
        },
        "circuit": {
            "circuit_ref": "vegas",
            "name": "Las Vegas Strip Street Circuit",
            "location": "Las Vegas",
            "country": "United States",
            "lat": 36.1147,
            "lng": -115.173,
            "alt": 642,
            "url": "https://en.wikipedia.org/wiki/Las_Vegas_Grand_Prix#Circuit"
        }
    },
    {
        "season": 2023,
        "round": 22,
        "name": "Abu Dhabi Grand Prix",
        "date": "2023-11-26",
        "time": "13:00:00",
        "url": "https://en.wikipedia.org/wiki/2023_Abu_Dhabi_Grand_Prix",
        "fp1": {
            "date": "2023-11-24",
            "time": "09:30:00"
        },
        "fp2": {
            "date": "2023-11-24",
            "time": "13:00:00"
        },
        "fp3": {
            "date": "2023-11-25",
            "time": "10:30:00"
        },
        "quali": {
            "date": "2023-11-25",
            "time": "14:00:00"
        },
        "circuit": {
            "circuit_ref": "yas_marina",
            "name": "Yas Marina Circuit",
            "location": "Abu Dhabi",
            "country": "UAE",
            "lat": 24.4672,
            "lng": 54.6031,
            "alt": 3,
            "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit"
        }
    },
    {
        "season": 2024,
        "round": 1,
        "name": "Bahrain Grand Prix",
        "date": "2024-03-02",
        "time": "15:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Bahrain_Grand_Prix",
        "fp1": {
            "date": "2024-02-29",
            "time": "11:30:00"
        },
        "fp2": {
            "date": "2024-02-29",
            "time": "15:00:00"
        },
        "fp3": {
            "date": "2024-03-01",
            "time": "12:30:00"
        },
        "quali": {
            "date": "2024-03-01",
            "time": "16:00:00"
        },
        "circuit": {
            "circuit_ref": "bahrain",
            "name": "Bahrain International Circuit",
            "location": "Sakhir",
            "country": "Bahrain",
            "lat": 26.0325,
            "lng": 50.5106,
            "alt": 7,
            "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
        }
    },
    {
        "season": 2024,
        "round": 2,
        "name": "Saudi Arabian Grand Prix",
        "date": "2024-03-09",
        "time": "17:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Saudi_Arabian_Grand_Prix",
        "fp1": {
            "date": "2024-03-07",
            "time": "13:30:00"
        },
        "fp2": {
            "date": "2024-03-07",
            "time": "17:00:00"
        },
        "fp3": {
            "date": "2024-03-08",
            "time": "13:30:00"
        },
        "quali": {
            "date": "2024-03-08",
            "time": "17:00:00"
        },
        "circuit": {
            "circuit_ref": "jeddah",
            "name": "Jeddah Corniche Circuit",
            "location": "Jeddah",
            "country": "Saudi Arabia",
            "lat": 21.6319,
            "lng": 39.1044,
            "alt": 15,
            "url": "http://en.wikipedia.org/wiki/Jeddah_Street_Circuit"
        }
    },
    {
        "season": 2024,
        "round": 3,
        "name": "Australian Grand Prix",
        "date": "2024-03-24",
        "time": "04:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Australian_Grand_Prix",
        "fp1": {
            "date": "2024-03-22",
            "time": "01:30:00"
        },
        "fp2": {
            "date": "2024-03-22",
            "time": "05:00:00"
        },
        "fp3": {
            "date": "2024-03-23",
            "time": "01:30:00"
        },
        "quali": {
            "date": "2024-03-23",
            "time": "05:00:00"
        },
        "circuit": {
            "circuit_ref": "albert_park",
            "name": "Albert Park Grand Prix Circuit",
            "location": "Melbourne",
            "country": "Australia",
            "lat": -37.8497,
            "lng": 144.968,
            "alt": 10,
            "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit"
        }
    },
    {
        "season": 2024,
        "round": 4,
        "name": "Japanese Grand Prix",
        "date": "2024-04-07",
        "time": "05:00:00",
        "url": "https://en.wikipedia.org/wiki/2024_Japanese_Grand_Prix",
        "fp1": {
            "date": "2024-04-05",
            "time": "02:30:00"
        },
        "fp2": {
            "date": "2024-04-05",
            "time": "06:00:00"
        },
        "fp3": {
            "date": "2024-04-06",
            "time": "02:30:00"
        },
        "quali": {
            "date": "2024-04-06",
            "time": "06:00:00"
        },
        "circuit": {
            "circuit_ref": "suzuka",
            "name": "Suzuka Circuit",
            "location": "Suzuka",
            "country": "Japan",
            "lat": 34.8431,
            "lng": 136.541,
            "alt": 45,
            "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit"
        }
    }
];

const LECLERC_RACES: [StaticRace; 30] = races_from_json![
    {
        "season": 2018,
        "round": 1,
        "name": "Australian Grand Prix",
        "date": "2018-03-25",
        "time": "05:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Australian_Grand_Prix",
        "circuit": {
            "circuit_ref": "albert_park",
            "name": "Albert Park Grand Prix Circuit",
            "location": "Melbourne",
            "country": "Australia",
            "lat": -37.8497,
            "lng": 144.968,
            "alt": 10,
            "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit"
        }
    },
    {
        "season": 2018,
        "round": 2,
        "name": "Bahrain Grand Prix",
        "date": "2018-04-08",
        "time": "15:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Bahrain_Grand_Prix",
        "circuit": {
            "circuit_ref": "bahrain",
            "name": "Bahrain International Circuit",
            "location": "Sakhir",
            "country": "Bahrain",
            "lat": 26.0325,
            "lng": 50.5106,
            "alt": 7,
            "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
        }
    },
    {
        "season": 2018,
        "round": 3,
        "name": "Chinese Grand Prix",
        "date": "2018-04-15",
        "time": "06:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Chinese_Grand_Prix",
        "circuit": {
            "circuit_ref": "shanghai",
            "name": "Shanghai International Circuit",
            "location": "Shanghai",
            "country": "China",
            "lat": 31.3389,
            "lng": 121.22,
            "alt": 5,
            "url": "http://en.wikipedia.org/wiki/Shanghai_International_Circuit"
        }
    },
    {
        "season": 2018,
        "round": 4,
        "name": "Azerbaijan Grand Prix",
        "date": "2018-04-29",
        "time": "12:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Azerbaijan_Grand_Prix",
        "circuit": {
            "circuit_ref": "baku",
            "name": "Baku City Circuit",
            "location": "Baku",
            "country": "Azerbaijan",
            "lat": 40.3725,
            "lng": 49.8533,
            "alt": -7,
            "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit"
        }
    },
    {
        "season": 2018,
        "round": 5,
        "name": "Spanish Grand Prix",
        "date": "2018-05-13",
        "time": "13:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Spanish_Grand_Prix",
        "circuit": {
            "circuit_ref": "catalunya",
            "name": "Circuit de Barcelona-Catalunya",
            "location": "Montmeló",
            "country": "Spain",
            "lat": 41.57,
            "lng": 2.26111,
            "alt": 109,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Barcelona-Catalunya"
        }
    },
    {
        "season": 2018,
        "round": 6,
        "name": "Monaco Grand Prix",
        "date": "2018-05-27",
        "time": "13:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Monaco_Grand_Prix",
        "circuit": {
            "circuit_ref": "monaco",
            "name": "Circuit de Monaco",
            "location": "Monte-Carlo",
            "country": "Monaco",
            "lat": 43.7347,
            "lng": 7.42056,
            "alt": 7,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco"
        }
    },
    {
        "season": 2018,
        "round": 7,
        "name": "Canadian Grand Prix",
        "date": "2018-06-10",
        "time": "18:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Canadian_Grand_Prix",
        "circuit": {
            "circuit_ref": "villeneuve",
            "name": "Circuit Gilles Villeneuve",
            "location": "Montreal",
            "country": "Canada",
            "lat": 45.5,
            "lng": -73.5228,
            "alt": 13,
            "url": "http://en.wikipedia.org/wiki/Circuit_Gilles_Villeneuve"
        }
    },
    {
        "season": 2018,
        "round": 8,
        "name": "French Grand Prix",
        "date": "2018-06-24",
        "time": "14:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_French_Grand_Prix",
        "circuit": {
            "circuit_ref": "ricard",
            "name": "Circuit Paul Ricard",
            "location": "Le Castellet",
            "country": "France",
            "lat": 43.2506,
            "lng": 5.79167,
            "alt": 432,
            "url": "http://en.wikipedia.org/wiki/Paul_Ricard_Circuit"
        }
    },
    {
        "season": 2018,
        "round": 9,
        "name": "Austrian Grand Prix",
        "date": "2018-07-01",
        "time": "13:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Austrian_Grand_Prix",
        "circuit": {
            "circuit_ref": "red_bull_ring",
            "name": "Red Bull Ring",
            "location": "Spielberg",
            "country": "Austria",
            "lat": 47.2197,
            "lng": 14.7647,
            "alt": 678,
            "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring"
        }
    },
    {
        "season": 2018,
        "round": 10,
        "name": "British Grand Prix",
        "date": "2018-07-08",
        "time": "13:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_British_Grand_Prix",
        "circuit": {
            "circuit_ref": "silverstone",
            "name": "Silverstone Circuit",
            "location": "Silverstone",
            "country": "UK",
            "lat": 52.0786,
            "lng": -1.01694,
            "alt": 153,
            "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit"
        }
    },
    {
        "season": 2018,
        "round": 11,
        "name": "German Grand Prix",
        "date": "2018-07-22",
        "time": "13:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_German_Grand_Prix",
        "circuit": {
            "circuit_ref": "hockenheimring",
            "name": "Hockenheimring",
            "location": "Hockenheim",
            "country": "Germany",
            "lat": 49.3278,
            "lng": 8.56583,
            "alt": 103,
            "url": "http://en.wikipedia.org/wiki/Hockenheimring"
        }
    },
    {
        "season": 2018,
        "round": 12,
        "name": "Hungarian Grand Prix",
        "date": "2018-07-29",
        "time": "13:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Hungarian_Grand_Prix",
        "circuit": {
            "circuit_ref": "hungaroring",
            "name": "Hungaroring",
            "location": "Budapest",
            "country": "Hungary",
            "lat": 47.5789,
            "lng": 19.2486,
            "alt": 264,
            "url": "http://en.wikipedia.org/wiki/Hungaroring"
        }
    },
    {
        "season": 2018,
        "round": 13,
        "name": "Belgian Grand Prix",
        "date": "2018-08-26",
        "time": "13:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Belgian_Grand_Prix",
        "circuit": {
            "circuit_ref": "spa",
            "name": "Circuit de Spa-Francorchamps",
            "location": "Spa",
            "country": "Belgium",
            "lat": 50.4372,
            "lng": 5.97139,
            "alt": 401,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
        }
    },
    {
        "season": 2018,
        "round": 14,
        "name": "Italian Grand Prix",
        "date": "2018-09-02",
        "time": "13:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Italian_Grand_Prix",
        "circuit": {
            "circuit_ref": "monza",
            "name": "Autodromo Nazionale di Monza",
            "location": "Monza",
            "country": "Italy",
            "lat": 45.6156,
            "lng": 9.28111,
            "alt": 162,
            "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
        }
    },
    {
        "season": 2018,
        "round": 15,
        "name": "Singapore Grand Prix",
        "date": "2018-09-16",
        "time": "12:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Singapore_Grand_Prix",
        "circuit": {
            "circuit_ref": "marina_bay",
            "name": "Marina Bay Street Circuit",
            "location": "Marina Bay",
            "country": "Singapore",
            "lat": 1.2914,
            "lng": 103.864,
            "alt": 18,
            "url": "http://en.wikipedia.org/wiki/Marina_Bay_Street_Circuit"
        }
    },
    {
        "season": 2018,
        "round": 16,
        "name": "Russian Grand Prix",
        "date": "2018-09-30",
        "time": "11:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Russian_Grand_Prix",
        "circuit": {
            "circuit_ref": "sochi",
            "name": "Sochi Autodrom",
            "location": "Sochi",
            "country": "Russia",
            "lat": 43.4057,
            "lng": 39.9578,
            "alt": 2,
            "url": "http://en.wikipedia.org/wiki/Sochi_Autodrom"
        }
    },
    {
        "season": 2018,
        "round": 17,
        "name": "Japanese Grand Prix",
        "date": "2018-10-07",
        "time": "05:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Japanese_Grand_Prix",
        "circuit": {
            "circuit_ref": "suzuka",
            "name": "Suzuka Circuit",
            "location": "Suzuka",
            "country": "Japan",
            "lat": 34.8431,
            "lng": 136.541,
            "alt": 45,
            "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit"
        }
    },
    {
        "season": 2018,
        "round": 18,
        "name": "United States Grand Prix",
        "date": "2018-10-21",
        "time": "18:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_United_States_Grand_Prix",
        "circuit": {
            "circuit_ref": "americas",
            "name": "Circuit of the Americas",
            "location": "Austin",
            "country": "USA",
            "lat": 30.1328,
            "lng": -97.6411,
            "alt": 161,
            "url": "http://en.wikipedia.org/wiki/Circuit_of_the_Americas"
        }
    },
    {
        "season": 2018,
        "round": 19,
        "name": "Mexican Grand Prix",
        "date": "2018-10-28",
        "time": "19:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Mexican_Grand_Prix",
        "circuit": {
            "circuit_ref": "rodriguez",
            "name": "Autódromo Hermanos Rodríguez",
            "location": "Mexico City",
            "country": "Mexico",
            "lat": 19.4042,
            "lng": -99.0907,
            "alt": 2227,
            "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Hermanos_Rodr%C3%ADguez"
        }
    },
    {
        "season": 2018,
        "round": 20,
        "name": "Brazilian Grand Prix",
        "date": "2018-11-11",
        "time": "17:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Brazilian_Grand_Prix",
        "circuit": {
            "circuit_ref": "interlagos",
            "name": "Autódromo José Carlos Pace",
            "location": "São Paulo",
            "country": "Brazil",
            "lat": -23.7036,
            "lng": -46.6997,
            "alt": 785,
            "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Jos%C3%A9_Carlos_Pace"
        }
    },
    {
        "season": 2018,
        "round": 21,
        "name": "Abu Dhabi Grand Prix",
        "date": "2018-11-25",
        "time": "13:10:00",
        "url": "http://en.wikipedia.org/wiki/2018_Abu_Dhabi_Grand_Prix",
        "circuit": {
            "circuit_ref": "yas_marina",
            "name": "Yas Marina Circuit",
            "location": "Abu Dhabi",
            "country": "UAE",
            "lat": 24.4672,
            "lng": 54.6031,
            "alt": 3,
            "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit"
        }
    },
    {
        "season": 2019,
        "round": 1,
        "name": "Australian Grand Prix",
        "date": "2019-03-17",
        "time": "05:10:00",
        "url": "http://en.wikipedia.org/wiki/2019_Australian_Grand_Prix",
        "circuit": {
            "circuit_ref": "albert_park",
            "name": "Albert Park Grand Prix Circuit",
            "location": "Melbourne",
            "country": "Australia",
            "lat": -37.8497,
            "lng": 144.968,
            "alt": 10,
            "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit"
        }
    },
    {
        "season": 2019,
        "round": 2,
        "name": "Bahrain Grand Prix",
        "date": "2019-03-31",
        "time": "15:10:00",
        "url": "http://en.wikipedia.org/wiki/2019_Bahrain_Grand_Prix",
        "circuit": {
            "circuit_ref": "bahrain",
            "name": "Bahrain International Circuit",
            "location": "Sakhir",
            "country": "Bahrain",
            "lat": 26.0325,
            "lng": 50.5106,
            "alt": 7,
            "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
        }
    },
    {
        "season": 2019,
        "round": 3,
        "name": "Chinese Grand Prix",
        "date": "2019-04-14",
        "time": "06:10:00",
        "url": "http://en.wikipedia.org/wiki/2019_Chinese_Grand_Prix",
        "circuit": {
            "circuit_ref": "shanghai",
            "name": "Shanghai International Circuit",
            "location": "Shanghai",
            "country": "China",
            "lat": 31.3389,
            "lng": 121.22,
            "alt": 5,
            "url": "http://en.wikipedia.org/wiki/Shanghai_International_Circuit"
        }
    },
    {
        "season": 2019,
        "round": 4,
        "name": "Azerbaijan Grand Prix",
        "date": "2019-04-28",
        "time": "12:10:00",
        "url": "http://en.wikipedia.org/wiki/2019_Azerbaijan_Grand_Prix",
        "circuit": {
            "circuit_ref": "baku",
            "name": "Baku City Circuit",
            "location": "Baku",
            "country": "Azerbaijan",
            "lat": 40.3725,
            "lng": 49.8533,
            "alt": -7,
            "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit"
        }
    },
    {
        "season": 2019,
        "round": 5,
        "name": "Spanish Grand Prix",
        "date": "2019-05-12",
        "time": "13:10:00",
        "url": "http://en.wikipedia.org/wiki/2019_Spanish_Grand_Prix",
        "circuit": {
            "circuit_ref": "catalunya",
            "name": "Circuit de Barcelona-Catalunya",
            "location": "Montmeló",
            "country": "Spain",
            "lat": 41.57,
            "lng": 2.26111,
            "alt": 109,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Barcelona-Catalunya"
        }
    },
    {
        "season": 2019,
        "round": 6,
        "name": "Monaco Grand Prix",
        "date": "2019-05-26",
        "time": "13:10:00",
        "url": "http://en.wikipedia.org/wiki/2019_Monaco_Grand_Prix",
        "circuit": {
            "circuit_ref": "monaco",
            "name": "Circuit de Monaco",
            "location": "Monte-Carlo",
            "country": "Monaco",
            "lat": 43.7347,
            "lng": 7.42056,
            "alt": 7,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco"
        }
    },
    {
        "season": 2019,
        "round": 7,
        "name": "Canadian Grand Prix",
        "date": "2019-06-09",
        "time": "18:10:00",
        "url": "http://en.wikipedia.org/wiki/2019_Canadian_Grand_Prix",
        "circuit": {
            "circuit_ref": "villeneuve",
            "name": "Circuit Gilles Villeneuve",
            "location": "Montreal",
            "country": "Canada",
            "lat": 45.5,
            "lng": -73.5228,
            "alt": 13,
            "url": "http://en.wikipedia.org/wiki/Circuit_Gilles_Villeneuve"
        }
    },
    {
        "season": 2019,
        "round": 8,
        "name": "French Grand Prix",
        "date": "2019-06-23",
        "time": "13:10:00",
        "url": "http://en.wikipedia.org/wiki/2019_French_Grand_Prix",
        "circuit": {
            "circuit_ref": "ricard",
            "name": "Circuit Paul Ricard",
            "location": "Le Castellet",
            "country": "France",
            "lat": 43.2506,
            "lng": 5.79167,
            "alt": 432,
            "url": "http://en.wikipedia.org/wiki/Paul_Ricard_Circuit"
        }
    },
    {
        "season": 2019,
        "round": 9,
        "name": "Austrian Grand Prix",
        "date": "2019-06-30",
        "time": "13:10:00",
        "url": "http://en.wikipedia.org/wiki/2019_Austrian_Grand_Prix",
        "circuit": {
            "circuit_ref": "red_bull_ring",
            "name": "Red Bull Ring",
            "location": "Spielberg",
            "country": "Austria",
            "lat": 47.2197,
            "lng": 14.7647,
            "alt": 678,
            "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring"
        }
    }
];

const FERRARI_RACES: [StaticRace; 30] = races_from_json![
    {
        "season": 1950,
        "round": 2,
        "name": "Monaco Grand Prix",
        "date": "1950-05-21",
        "url": "http://en.wikipedia.org/wiki/1950_Monaco_Grand_Prix",
        "circuit": {
            "circuit_ref": "monaco",
            "name": "Circuit de Monaco",
            "location": "Monte-Carlo",
            "country": "Monaco",
            "lat": 43.7347,
            "lng": 7.42056,
            "alt": 7,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco"
        }
    },
    {
        "season": 1950,
        "round": 4,
        "name": "Swiss Grand Prix",
        "date": "1950-06-04",
        "url": "http://en.wikipedia.org/wiki/1950_Swiss_Grand_Prix",
        "circuit": {
            "circuit_ref": "bremgarten",
            "name": "Circuit Bremgarten",
            "location": "Bern",
            "country": "Switzerland",
            "lat": 46.9589,
            "lng": 7.40194,
            "alt": 551,
            "url": "http://en.wikipedia.org/wiki/Circuit_Bremgarten"
        }
    },
    {
        "season": 1950,
        "round": 5,
        "name": "Belgian Grand Prix",
        "date": "1950-06-18",
        "url": "http://en.wikipedia.org/wiki/1950_Belgian_Grand_Prix",
        "circuit": {
            "circuit_ref": "spa",
            "name": "Circuit de Spa-Francorchamps",
            "location": "Spa",
            "country": "Belgium",
            "lat": 50.4372,
            "lng": 5.97139,
            "alt": 401,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
        }
    },
    {
        "season": 1950,
        "round": 6,
        "name": "French Grand Prix",
        "date": "1950-07-02",
        "url": "http://en.wikipedia.org/wiki/1950_French_Grand_Prix",
        "circuit": {
            "circuit_ref": "reims",
            "name": "Reims-Gueux",
            "location": "Reims",
            "country": "France",
            "lat": 49.2542,
            "lng": 3.93083,
            "alt": 88,
            "url": "http://en.wikipedia.org/wiki/Reims-Gueux"
        }
    },
    {
        "season": 1950,
        "round": 7,
        "name": "Italian Grand Prix",
        "date": "1950-09-03",
        "url": "http://en.wikipedia.org/wiki/1950_Italian_Grand_Prix",
        "circuit": {
            "circuit_ref": "monza",
            "name": "Autodromo Nazionale di Monza",
            "location": "Monza",
            "country": "Italy",
            "lat": 45.6156,
            "lng": 9.28111,
            "alt": 162,
            "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
        }
    },
    {
        "season": 1951,
        "round": 1,
        "name": "Swiss Grand Prix",
        "date": "1951-05-27",
        "url": "http://en.wikipedia.org/wiki/1951_Swiss_Grand_Prix",
        "circuit": {
            "circuit_ref": "bremgarten",
            "name": "Circuit Bremgarten",
            "location": "Bern",
            "country": "Switzerland",
            "lat": 46.9589,
            "lng": 7.40194,
            "alt": 551,
            "url": "http://en.wikipedia.org/wiki/Circuit_Bremgarten"
        }
    },
    {
        "season": 1951,
        "round": 3,
        "name": "Belgian Grand Prix",
        "date": "1951-06-17",
        "url": "http://en.wikipedia.org/wiki/1951_Belgian_Grand_Prix",
        "circuit": {
            "circuit_ref": "spa",
            "name": "Circuit de Spa-Francorchamps",
            "location": "Spa",
            "country": "Belgium",
            "lat": 50.4372,
            "lng": 5.97139,
            "alt": 401,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
        }
    },
    {
        "season": 1951,
        "round": 4,
        "name": "French Grand Prix",
        "date": "1951-07-01",
        "url": "http://en.wikipedia.org/wiki/1951_French_Grand_Prix",
        "circuit": {
            "circuit_ref": "reims",
            "name": "Reims-Gueux",
            "location": "Reims",
            "country": "France",
            "lat": 49.2542,
            "lng": 3.93083,
            "alt": 88,
            "url": "http://en.wikipedia.org/wiki/Reims-Gueux"
        }
    },
    {
        "season": 1951,
        "round": 5,
        "name": "British Grand Prix",
        "date": "1951-07-14",
        "url": "http://en.wikipedia.org/wiki/1951_British_Grand_Prix",
        "circuit": {
            "circuit_ref": "silverstone",
            "name": "Silverstone Circuit",
            "location": "Silverstone",
            "country": "UK",
            "lat": 52.0786,
            "lng": -1.01694,
            "alt": 153,
            "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit"
        }
    },
    {
        "season": 1951,
        "round": 6,
        "name": "German Grand Prix",
        "date": "1951-07-29",
        "url": "http://en.wikipedia.org/wiki/1951_German_Grand_Prix",
        "circuit": {
            "circuit_ref": "nurburgring",
            "name": "Nürburgring",
            "location": "Nürburg",
            "country": "Germany",
            "lat": 50.3356,
            "lng": 6.9475,
            "alt": 578,
            "url": "http://en.wikipedia.org/wiki/N%C3%BCrburgring"
        }
    },
    {
        "season": 1951,
        "round": 7,
        "name": "Italian Grand Prix",
        "date": "1951-09-16",
        "url": "http://en.wikipedia.org/wiki/1951_Italian_Grand_Prix",
        "circuit": {
            "circuit_ref": "monza",
            "name": "Autodromo Nazionale di Monza",
            "location": "Monza",
            "country": "Italy",
            "lat": 45.6156,
            "lng": 9.28111,
            "alt": 162,
            "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
        }
    },
    {
        "season": 1951,
        "round": 8,
        "name": "Spanish Grand Prix",
        "date": "1951-10-28",
        "url": "http://en.wikipedia.org/wiki/1951_Spanish_Grand_Prix",
        "circuit": {
            "circuit_ref": "pedralbes",
            "name": "Circuit de Pedralbes",
            "location": "Barcelona",
            "country": "Spain",
            "lat": 41.3903,
            "lng": 2.11667,
            "alt": 85,
            "url": "http://en.wikipedia.org/wiki/Pedralbes_Circuit"
        }
    },
    {
        "season": 1952,
        "round": 1,
        "name": "Swiss Grand Prix",
        "date": "1952-05-18",
        "url": "http://en.wikipedia.org/wiki/1952_Swiss_Grand_Prix",
        "circuit": {
            "circuit_ref": "bremgarten",
            "name": "Circuit Bremgarten",
            "location": "Bern",
            "country": "Switzerland",
            "lat": 46.9589,
            "lng": 7.40194,
            "alt": 551,
            "url": "http://en.wikipedia.org/wiki/Circuit_Bremgarten"
        }
    },
    {
        "season": 1952,
        "round": 2,
        "name": "Indianapolis 500",
        "date": "1952-05-30",
        "url": "http://en.wikipedia.org/wiki/1952_Indianapolis_500",
        "circuit": {
            "circuit_ref": "indianapolis",
            "name": "Indianapolis Motor Speedway",
            "location": "Indianapolis",
            "country": "USA",
            "lat": 39.795,
            "lng": -86.2347,
            "alt": 223,
            "url": "http://en.wikipedia.org/wiki/Indianapolis_Motor_Speedway"
        }
    },
    {
        "season": 1952,
        "round": 3,
        "name": "Belgian Grand Prix",
        "date": "1952-06-22",
        "url": "http://en.wikipedia.org/wiki/1952_Belgian_Grand_Prix",
        "circuit": {
            "circuit_ref": "spa",
            "name": "Circuit de Spa-Francorchamps",
            "location": "Spa",
            "country": "Belgium",
            "lat": 50.4372,
            "lng": 5.97139,
            "alt": 401,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
        }
    },
    {
        "season": 1952,
        "round": 4,
        "name": "French Grand Prix",
        "date": "1952-07-06",
        "url": "http://en.wikipedia.org/wiki/1952_French_Grand_Prix",
        "circuit": {
            "circuit_ref": "essarts",
            "name": "Rouen-Les-Essarts",
            "location": "Rouen",
            "country": "France",
            "lat": 49.3306,
            "lng": 1.00458,
            "alt": 81,
            "url": "http://en.wikipedia.org/wiki/Rouen-Les-Essarts"
        }
    },
    {
        "season": 1952,
        "round": 5,
        "name": "British Grand Prix",
        "date": "1952-07-19",
        "url": "http://en.wikipedia.org/wiki/1952_British_Grand_Prix",
        "circuit": {
            "circuit_ref": "silverstone",
            "name": "Silverstone Circuit",
            "location": "Silverstone",
            "country": "UK",
            "lat": 52.0786,
            "lng": -1.01694,
            "alt": 153,
            "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit"
        }
    },
    {
        "season": 1952,
        "round": 6,
        "name": "German Grand Prix",
        "date": "1952-08-03",
        "url": "http://en.wikipedia.org/wiki/1952_German_Grand_Prix",
        "circuit": {
            "circuit_ref": "nurburgring",
            "name": "Nürburgring",
            "location": "Nürburg",
            "country": "Germany",
            "lat": 50.3356,
            "lng": 6.9475,
            "alt": 578,
            "url": "http://en.wikipedia.org/wiki/N%C3%BCrburgring"
        }
    },
    {
        "season": 1952,
        "round": 7,
        "name": "Dutch Grand Prix",
        "date": "1952-08-17",
        "url": "http://en.wikipedia.org/wiki/1952_Dutch_Grand_Prix",
        "circuit": {
            "circuit_ref": "zandvoort",
            "name": "Circuit Park Zandvoort",
            "location": "Zandvoort",
            "country": "Netherlands",
            "lat": 52.3888,
            "lng": 4.54092,
            "alt": 6,
            "url": "http://en.wikipedia.org/wiki/Circuit_Zandvoort"
        }
    },
    {
        "season": 1952,
        "round": 8,
        "name": "Italian Grand Prix",
        "date": "1952-09-07",
        "url": "http://en.wikipedia.org/wiki/1952_Italian_Grand_Prix",
        "circuit": {
            "circuit_ref": "monza",
            "name": "Autodromo Nazionale di Monza",
            "location": "Monza",
            "country": "Italy",
            "lat": 45.6156,
            "lng": 9.28111,
            "alt": 162,
            "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
        }
    },
    {
        "season": 1953,
        "round": 1,
        "name": "Argentine Grand Prix",
        "date": "1953-01-18",
        "url": "http://en.wikipedia.org/wiki/1953_Argentine_Grand_Prix",
        "circuit": {
            "circuit_ref": "galvez",
            "name": "Autódromo Juan y Oscar Gálvez",
            "location": "Buenos Aires",
            "country": "Argentina",
            "lat": -34.6943,
            "lng": -58.4593,
            "alt": 8,
            "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Oscar_Alfredo_G%C3%A1lvez"
        }
    },
    {
        "season": 1953,
        "round": 3,
        "name": "Dutch Grand Prix",
        "date": "1953-06-07",
        "url": "http://en.wikipedia.org/wiki/1953_Dutch_Grand_Prix",
        "circuit": {
            "circuit_ref": "zandvoort",
            "name": "Circuit Park Zandvoort",
            "location": "Zandvoort",
            "country": "Netherlands",
            "lat": 52.3888,
            "lng": 4.54092,
            "alt": 6,
            "url": "http://en.wikipedia.org/wiki/Circuit_Zandvoort"
        }
    },
    {
        "season": 1953,
        "round": 4,
        "name": "Belgian Grand Prix",
        "date": "1953-06-21",
        "url": "http://en.wikipedia.org/wiki/1953_Belgian_Grand_Prix",
        "circuit": {
            "circuit_ref": "spa",
            "name": "Circuit de Spa-Francorchamps",
            "location": "Spa",
            "country": "Belgium",
            "lat": 50.4372,
            "lng": 5.97139,
            "alt": 401,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
        }
    },
    {
        "season": 1953,
        "round": 5,
        "name": "French Grand Prix",
        "date": "1953-07-05",
        "url": "http://en.wikipedia.org/wiki/1953_French_Grand_Prix",
        "circuit": {
            "circuit_ref": "reims",
            "name": "Reims-Gueux",
            "location": "Reims",
            "country": "France",
            "lat": 49.2542,
            "lng": 3.93083,
            "alt": 88,
            "url": "http://en.wikipedia.org/wiki/Reims-Gueux"
        }
    },
    {
        "season": 1953,
        "round": 6,
        "name": "British Grand Prix",
        "date": "1953-07-18",
        "url": "http://en.wikipedia.org/wiki/1953_British_Grand_Prix",
        "circuit": {
            "circuit_ref": "silverstone",
            "name": "Silverstone Circuit",
            "location": "Silverstone",
            "country": "UK",
            "lat": 52.0786,
            "lng": -1.01694,
            "alt": 153,
            "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit"
        }
    },
    {
        "season": 1953,
        "round": 7,
        "name": "German Grand Prix",
        "date": "1953-08-02",
        "url": "http://en.wikipedia.org/wiki/1953_German_Grand_Prix",
        "circuit": {
            "circuit_ref": "nurburgring",
            "name": "Nürburgring",
            "location": "Nürburg",
            "country": "Germany",
            "lat": 50.3356,
            "lng": 6.9475,
            "alt": 578,
            "url": "http://en.wikipedia.org/wiki/N%C3%BCrburgring"
        }
    },
    {
        "season": 1953,
        "round": 8,
        "name": "Swiss Grand Prix",
        "date": "1953-08-23",
        "url": "http://en.wikipedia.org/wiki/1953_Swiss_Grand_Prix",
        "circuit": {
            "circuit_ref": "bremgarten",
            "name": "Circuit Bremgarten",
            "location": "Bern",
            "country": "Switzerland",
            "lat": 46.9589,
            "lng": 7.40194,
            "alt": 551,
            "url": "http://en.wikipedia.org/wiki/Circuit_Bremgarten"
        }
    },
    {
        "season": 1953,
        "round": 9,
        "name": "Italian Grand Prix",
        "date": "1953-09-13",
        "url": "http://en.wikipedia.org/wiki/1953_Italian_Grand_Prix",
        "circuit": {
            "circuit_ref": "monza",
            "name": "Autodromo Nazionale di Monza",
            "location": "Monza",
            "country": "Italy",
            "lat": 45.6156,
            "lng": 9.28111,
            "alt": 162,
            "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza"
        }
    },
    {
        "season": 1954,
        "round": 1,
        "name": "Argentine Grand Prix",
        "date": "1954-01-17",
        "url": "http://en.wikipedia.org/wiki/1954_Argentine_Grand_Prix",
        "circuit": {
            "circuit_ref": "galvez",
            "name": "Autódromo Juan y Oscar Gálvez",
            "location": "Buenos Aires",
            "country": "Argentina",
            "lat": -34.6943,
            "lng": -58.4593,
            "alt": 8,
            "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Oscar_Alfredo_G%C3%A1lvez"
        }
    },
    {
        "season": 1954,
        "round": 3,
        "name": "Belgian Grand Prix",
        "date": "1954-06-20",
        "url": "http://en.wikipedia.org/wiki/1954_Belgian_Grand_Prix",
        "circuit": {
            "circuit_ref": "spa",
            "name": "Circuit de Spa-Francorchamps",
            "location": "Spa",
            "country": "Belgium",
            "lat": 50.4372,
            "lng": 5.97139,
            "alt": 401,
            "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
        }
    }
];
