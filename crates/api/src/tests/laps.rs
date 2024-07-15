use serde_json::json;

use super::common::Test;

#[tokio::test]
async fn test_get_laps() {
    let value: serde_json::Value = json!({
        "lap_times": {
                    "url": "https://en.wikipedia.org/wiki/2023_Bahrain_Grand_Prix",
                    "raceName": "Bahrain Grand Prix",
                    "date": "2023-03-05",
                    "time": "15:00:00",
                    "circuit": {
                        "circuitRef": "bahrain",
                        "name": "Bahrain International Circuit",
                        "location": "Sakhir",
                        "country": "Bahrain",
                        "lat": 26.032499313354492,
                        "lng": 50.51060104370117,
                        "alt": 7,
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
                    },
                    "laps": [
                        {
                            "number": 1,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:39.019"
                                },
                                {
                                    "driverRef": "leclerc",
                                    "position": 2,
                                    "time": "1:40.230"
                                },
                                {
                                    "driverRef": "perez",
                                    "position": 3,
                                    "time": "1:41.063"
                                },
                                {
                                    "driverRef": "sainz",
                                    "position": 4,
                                    "time": "1:41.659"
                                },
                                {
                                    "driverRef": "hamilton",
                                    "position": 5,
                                    "time": "1:42.288"
                                },
                                {
                                    "driverRef": "russell",
                                    "position": 6,
                                    "time": "1:42.662"
                                },
                                {
                                    "driverRef": "alonso",
                                    "position": 7,
                                    "time": "1:43.608"
                                },
                                {
                                    "driverRef": "bottas",
                                    "position": 8,
                                    "time": "1:44.154"
                                },
                                {
                                    "driverRef": "stroll",
                                    "position": 9,
                                    "time": "1:44.670"
                                },
                                {
                                    "driverRef": "norris",
                                    "position": 10,
                                    "time": "1:45.364"
                                },
                                {
                                    "driverRef": "ocon",
                                    "position": 11,
                                    "time": "1:45.967"
                                },
                                {
                                    "driverRef": "albon",
                                    "position": 12,
                                    "time": "1:46.453"
                                },
                                {
                                    "driverRef": "sargeant",
                                    "position": 13,
                                    "time": "1:46.855"
                                },
                                {
                                    "driverRef": "hulkenberg",
                                    "position": 14,
                                    "time": "1:47.339"
                                },
                                {
                                    "driverRef": "tsunoda",
                                    "position": 15,
                                    "time": "1:47.791"
                                },
                                {
                                    "driverRef": "piastri",
                                    "position": 16,
                                    "time": "1:48.214"
                                },
                                {
                                    "driverRef": "zhou",
                                    "position": 17,
                                    "time": "1:48.467"
                                },
                                {
                                    "driverRef": "kevin_magnussen",
                                    "position": 18,
                                    "time": "1:49.330"
                                },
                                {
                                    "driverRef": "gasly",
                                    "position": 19,
                                    "time": "1:49.649"
                                },
                                {
                                    "driverRef": "de_vries",
                                    "position": 20,
                                    "time": "1:49.959"
                                }
                            ]
                        },
                        {
                            "number": 2,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.974"
                                },
                                {
                                    "driverRef": "leclerc",
                                    "position": 2,
                                    "time": "1:38.750"
                                },
                                {
                                    "driverRef": "perez",
                                    "position": 3,
                                    "time": "1:38.862"
                                },
                                {
                                    "driverRef": "sainz",
                                    "position": 4,
                                    "time": "1:38.933"
                                },
                                {
                                    "driverRef": "hamilton",
                                    "position": 5,
                                    "time": "1:39.166"
                                },
                                {
                                    "driverRef": "russell",
                                    "position": 6,
                                    "time": "1:39.590"
                                },
                                {
                                    "driverRef": "alonso",
                                    "position": 7,
                                    "time": "1:39.792"
                                },
                                {
                                    "driverRef": "bottas",
                                    "position": 8,
                                    "time": "1:39.956"
                                },
                                {
                                    "driverRef": "stroll",
                                    "position": 9,
                                    "time": "1:39.892"
                                },
                                {
                                    "driverRef": "norris",
                                    "position": 10,
                                    "time": "1:40.433"
                                }
                            ]
                        }
                    ]
                }
    });

    Test::new(
        r#"{
            lap_times(options: { year: 2023, round: 1 }, pagination: { limit: 30, page: 1 }) {
                url
                raceName
                date
                time
                circuit {
                    circuitRef
                    name
                    location
                    country
                    lat
                    lng
                    alt
                    url
                }
                laps {
                    number
                    timings {
                        driverRef
                        position
                        time
                    }
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_laps_by_driver_ref() {
    let value: serde_json::Value = json!({
        "lap_times": {
                    "url": "https://en.wikipedia.org/wiki/2023_Bahrain_Grand_Prix",
                    "raceName": "Bahrain Grand Prix",
                    "date": "2023-03-05",
                    "time": "15:00:00",
                    "circuit": {
                        "circuitRef": "bahrain",
                        "name": "Bahrain International Circuit",
                        "location": "Sakhir",
                        "country": "Bahrain",
                        "lat": 26.032499313354492,
                        "lng": 50.51060104370117,
                        "alt": 7,
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
                    },
                    "laps": [
                        {
                            "number": 1,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:39.019"
                                }
                            ]
                        },
                        {
                            "number": 2,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.974"
                                }
                            ]
                        },
                        {
                            "number": 3,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:38.006"
                                }
                            ]
                        },
                        {
                            "number": 4,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.976"
                                }
                            ]
                        },
                        {
                            "number": 5,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:38.035"
                                }
                            ]
                        },
                        {
                            "number": 6,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.986"
                                }
                            ]
                        },
                        {
                            "number": 7,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:38.021"
                                }
                            ]
                        },
                        {
                            "number": 8,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:38.154"
                                }
                            ]
                        },
                        {
                            "number": 9,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:38.278"
                                }
                            ]
                        },
                        {
                            "number": 10,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:38.369"
                                }
                            ]
                        },
                        {
                            "number": 11,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:38.483"
                                }
                            ]
                        },
                        {
                            "number": 12,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:38.591"
                                }
                            ]
                        },
                        {
                            "number": 13,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:38.482"
                                }
                            ]
                        },
                        {
                            "number": 14,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:41.295"
                                }
                            ]
                        },
                        {
                            "number": 15,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 2,
                                    "time": "1:58.378"
                                }
                            ]
                        },
                        {
                            "number": 16,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 2,
                                    "time": "1:37.801"
                                }
                            ]
                        },
                        {
                            "number": 17,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 2,
                                    "time": "1:37.648"
                                }
                            ]
                        },
                        {
                            "number": 18,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.614"
                                }
                            ]
                        },
                        {
                            "number": 19,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.712"
                                }
                            ]
                        },
                        {
                            "number": 20,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.788"
                                }
                            ]
                        },
                        {
                            "number": 21,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.582"
                                }
                            ]
                        },
                        {
                            "number": 22,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.651"
                                }
                            ]
                        },
                        {
                            "number": 23,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.552"
                                }
                            ]
                        },
                        {
                            "number": 24,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.387"
                                }
                            ]
                        },
                        {
                            "number": 25,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.372"
                                }
                            ]
                        },
                        {
                            "number": 26,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.682"
                                }
                            ]
                        },
                        {
                            "number": 27,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.379"
                                }
                            ]
                        },
                        {
                            "number": 28,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.651"
                                }
                            ]
                        },
                        {
                            "number": 29,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.571"
                                }
                            ]
                        },
                        {
                            "number": 30,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.549"
                                }
                            ]
                        }
                    ]
                }
    });

    Test::new(
        r#"{
            lap_times(
                options: { year: 2023, round: 1, driverRef: "max_verstappen" }
                pagination: { limit: 30, page: 1 }
            ) {
                url
                raceName
                date
                time
                circuit {
                    circuitRef
                    name
                    location
                    country
                    lat
                    lng
                    alt
                    url
                }
                laps {
                    number
                    timings {
                        driverRef
                        position
                        time
                    }
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_laps_by_driver_ref_and_page() {
    let value: serde_json::Value = json!({
        "lap_times": {
                    "url": "https://en.wikipedia.org/wiki/2023_Bahrain_Grand_Prix",
                    "raceName": "Bahrain Grand Prix",
                    "date": "2023-03-05",
                    "time": "15:00:00",
                    "circuit": {
                        "circuitRef": "bahrain",
                        "name": "Bahrain International Circuit",
                        "location": "Sakhir",
                        "country": "Bahrain",
                        "lat": 26.032499313354492,
                        "lng": 50.51060104370117,
                        "alt": 7,
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
                    },
                    "laps": [
                        {
                            "number": 31,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.431"
                                }
                            ]
                        },
                        {
                            "number": 32,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.691"
                                }
                            ]
                        },
                        {
                            "number": 33,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.686"
                                }
                            ]
                        },
                        {
                            "number": 34,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.839"
                                }
                            ]
                        },
                        {
                            "number": 35,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.850"
                                }
                            ]
                        },
                        {
                            "number": 36,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:40.964"
                                }
                            ]
                        },
                        {
                            "number": 37,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:58.490"
                                }
                            ]
                        },
                        {
                            "number": 38,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:36.297"
                                }
                            ]
                        },
                        {
                            "number": 39,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:36.446"
                                }
                            ]
                        },
                        {
                            "number": 40,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:36.942"
                                }
                            ]
                        },
                        {
                            "number": 41,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "2:00.495"
                                }
                            ]
                        },
                        {
                            "number": 42,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:44.824"
                                }
                            ]
                        },
                        {
                            "number": 43,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:36.890"
                                }
                            ]
                        },
                        {
                            "number": 44,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:36.236"
                                }
                            ]
                        },
                        {
                            "number": 45,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.015"
                                }
                            ]
                        },
                        {
                            "number": 46,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:36.701"
                                }
                            ]
                        },
                        {
                            "number": 47,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.009"
                                }
                            ]
                        },
                        {
                            "number": 48,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:36.918"
                                }
                            ]
                        },
                        {
                            "number": 49,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:36.962"
                                }
                            ]
                        },
                        {
                            "number": 50,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.250"
                                }
                            ]
                        },
                        {
                            "number": 51,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.427"
                                }
                            ]
                        },
                        {
                            "number": 52,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:36.919"
                                }
                            ]
                        },
                        {
                            "number": 53,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:36.890"
                                }
                            ]
                        },
                        {
                            "number": 54,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:36.678"
                                }
                            ]
                        },
                        {
                            "number": 55,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:36.322"
                                }
                            ]
                        },
                        {
                            "number": 56,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:37.205"
                                }
                            ]
                        },
                        {
                            "number": 57,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:36.373"
                                }
                            ]
                        }
                    ]
                }
    });

    Test::new(
        r#"{
            lap_times(
                options: { year: 2023, round: 1, driverRef: "max_verstappen" }
                pagination: { limit: 30, page: 2 }
            ) {
                url
                raceName
                date
                time
                circuit {
                    circuitRef
                    name
                    location
                    country
                    lat
                    lng
                    alt
                    url
                }
                laps {
                    number
                    timings {
                        driverRef
                        position
                        time
                    }
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_laps_by_driver_ref_and_lap_number() {
    let value: serde_json::Value = json!({
        "lap_times": {
                    "url": "https://en.wikipedia.org/wiki/2023_Bahrain_Grand_Prix",
                    "raceName": "Bahrain Grand Prix",
                    "date": "2023-03-05",
                    "time": "15:00:00",
                    "circuit": {
                        "circuitRef": "bahrain",
                        "name": "Bahrain International Circuit",
                        "location": "Sakhir",
                        "country": "Bahrain",
                        "lat": 26.032499313354492,
                        "lng": 50.51060104370117,
                        "alt": 7,
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
                    },
                    "laps": [
                        {
                            "number": 10,
                            "timings": [
                                {
                                    "driverRef": "max_verstappen",
                                    "position": 1,
                                    "time": "1:38.369"
                                }
                            ]
                        }
                    ]
                }
    });

    Test::new(
        r#"{
            lap_times(
                options: { year: 2023, round: 1, driverRef: "max_verstappen", lapNumber: 10 }
                pagination: { limit: 30, page: 1 }
            ) {
                url
                raceName
                date
                time
                circuit {
                    circuitRef
                    name
                    location
                    country
                    lat
                    lng
                    alt
                    url
                }
                laps {
                    number
                    timings {
                        driverRef
                        position
                        time
                    }
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}
