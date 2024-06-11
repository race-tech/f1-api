use serde_json::json;

use super::common::Test;

#[tokio::test]
async fn test_get_pit_stops() {
    let value: serde_json::Value = json!({
        "pitStops": {
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
                    "pitStops": [
                        {
                            "driverRef": "gasly",
                            "lap": 9,
                            "stop": 1,
                            "time": "18:18:56",
                            "duration": "25.885"
                        },
                        {
                            "driverRef": "norris",
                            "lap": 10,
                            "stop": 1,
                            "time": "18:20:31",
                            "duration": "32.766"
                        },
                        {
                            "driverRef": "tsunoda",
                            "lap": 10,
                            "stop": 1,
                            "time": "18:20:34",
                            "duration": "25.267"
                        },
                        {
                            "driverRef": "bottas",
                            "lap": 11,
                            "stop": 1,
                            "time": "18:22:06",
                            "duration": "25.399"
                        },
                        {
                            "driverRef": "albon",
                            "lap": 11,
                            "stop": 1,
                            "time": "18:22:12",
                            "duration": "25.054"
                        },
                        {
                            "driverRef": "hulkenberg",
                            "lap": 11,
                            "stop": 1,
                            "time": "18:22:21",
                            "duration": "26.075"
                        },
                        {
                            "driverRef": "de_vries",
                            "lap": 11,
                            "stop": 1,
                            "time": "18:22:21",
                            "duration": "25.018"
                        },
                        {
                            "driverRef": "hamilton",
                            "lap": 12,
                            "stop": 1,
                            "time": "18:23:40",
                            "duration": "24.682"
                        },
                        {
                            "driverRef": "ocon",
                            "lap": 12,
                            "stop": 1,
                            "time": "18:23:50",
                            "duration": "24.966"
                        },
                        {
                            "driverRef": "sargeant",
                            "lap": 12,
                            "stop": 1,
                            "time": "18:23:55",
                            "duration": "25.600"
                        },
                        {
                            "driverRef": "zhou",
                            "lap": 12,
                            "stop": 1,
                            "time": "18:24:01",
                            "duration": "25.787"
                        },
                        {
                            "driverRef": "leclerc",
                            "lap": 13,
                            "stop": 1,
                            "time": "18:25:06",
                            "duration": "24.345"
                        },
                        {
                            "driverRef": "sainz",
                            "lap": 13,
                            "stop": 1,
                            "time": "18:25:17",
                            "duration": "24.500"
                        },
                        {
                            "driverRef": "russell",
                            "lap": 13,
                            "stop": 1,
                            "time": "18:25:24",
                            "duration": "27.062"
                        },
                        {
                            "driverRef": "max_verstappen",
                            "lap": 14,
                            "stop": 1,
                            "time": "18:26:35",
                            "duration": "24.289"
                        },
                        {
                            "driverRef": "alonso",
                            "lap": 14,
                            "stop": 1,
                            "time": "18:27:02",
                            "duration": "25.800"
                        },
                        {
                            "driverRef": "stroll",
                            "lap": 15,
                            "stop": 1,
                            "time": "18:28:45",
                            "duration": "25.449"
                        },
                        {
                            "driverRef": "kevin_magnussen",
                            "lap": 15,
                            "stop": 1,
                            "time": "18:29:10",
                            "duration": "25.439"
                        },
                        {
                            "driverRef": "ocon",
                            "lap": 15,
                            "stop": 2,
                            "time": "18:29:11",
                            "duration": "41.462"
                        },
                        {
                            "driverRef": "perez",
                            "lap": 17,
                            "stop": 1,
                            "time": "18:31:42",
                            "duration": "24.264"
                        },
                        {
                            "driverRef": "norris",
                            "lap": 17,
                            "stop": 2,
                            "time": "18:32:38",
                            "duration": "33.661"
                        },
                        {
                            "driverRef": "gasly",
                            "lap": 25,
                            "stop": 2,
                            "time": "18:45:48",
                            "duration": "24.879"
                        },
                        {
                            "driverRef": "albon",
                            "lap": 26,
                            "stop": 2,
                            "time": "18:47:25",
                            "duration": "24.920"
                        },
                        {
                            "driverRef": "tsunoda",
                            "lap": 26,
                            "stop": 2,
                            "time": "18:47:26",
                            "duration": "24.372"
                        },
                        {
                            "driverRef": "hulkenberg",
                            "lap": 26,
                            "stop": 2,
                            "time": "18:47:41",
                            "duration": "37.769"
                        },
                        {
                            "driverRef": "de_vries",
                            "lap": 27,
                            "stop": 2,
                            "time": "18:49:16",
                            "duration": "25.234"
                        },
                        {
                            "driverRef": "norris",
                            "lap": 27,
                            "stop": 3,
                            "time": "18:49:35",
                            "duration": "33.467"
                        },
                        {
                            "driverRef": "bottas",
                            "lap": 29,
                            "stop": 2,
                            "time": "18:52:16",
                            "duration": "25.643"
                        },
                        {
                            "driverRef": "kevin_magnussen",
                            "lap": 29,
                            "stop": 2,
                            "time": "18:52:40",
                            "duration": "25.344"
                        },
                        {
                            "driverRef": "hamilton",
                            "lap": 30,
                            "stop": 2,
                            "time": "18:53:36",
                            "duration": "24.690"
                        }
                    ]
                }
    });

    Test::new(
        r#"{
            pitStops(options: { year: 2023, round: 1 }, pagination: { limit: 30, page: 1 }) {
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
                pitStops {
                    driverRef
                    lap
                    stop
                    time
                    duration
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_pit_stops_by_page() {
    let value: serde_json::Value = json!({
        "pitStops": {
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
                    "pitStops": [
                        {
                            "driverRef": "stroll",
                            "lap": 30,
                            "stop": 2,
                            "time": "18:53:45",
                            "duration": "24.639"
                        },
                        {
                            "driverRef": "sargeant",
                            "lap": 30,
                            "stop": 2,
                            "time": "18:54:09",
                            "duration": "25.670"
                        },
                        {
                            "driverRef": "sainz",
                            "lap": 31,
                            "stop": 2,
                            "time": "18:55:11",
                            "duration": "24.227"
                        },
                        {
                            "driverRef": "russell",
                            "lap": 31,
                            "stop": 2,
                            "time": "18:55:21",
                            "duration": "24.406"
                        },
                        {
                            "driverRef": "zhou",
                            "lap": 32,
                            "stop": 2,
                            "time": "18:57:31",
                            "duration": "25.282"
                        },
                        {
                            "driverRef": "ocon",
                            "lap": 32,
                            "stop": 3,
                            "time": "18:58:03",
                            "duration": "44.518"
                        },
                        {
                            "driverRef": "leclerc",
                            "lap": 33,
                            "stop": 2,
                            "time": "18:58:16",
                            "duration": "24.644"
                        },
                        {
                            "driverRef": "perez",
                            "lap": 34,
                            "stop": 2,
                            "time": "18:59:45",
                            "duration": "25.091"
                        },
                        {
                            "driverRef": "alonso",
                            "lap": 34,
                            "stop": 2,
                            "time": "19:00:09",
                            "duration": "24.869"
                        },
                        {
                            "driverRef": "max_verstappen",
                            "lap": 36,
                            "stop": 2,
                            "time": "19:02:47",
                            "duration": "24.910"
                        },
                        {
                            "driverRef": "norris",
                            "lap": 37,
                            "stop": 4,
                            "time": "19:06:27",
                            "duration": "33.236"
                        },
                        {
                            "driverRef": "albon",
                            "lap": 40,
                            "stop": 3,
                            "time": "19:10:51",
                            "duration": "24.458"
                        },
                        {
                            "driverRef": "gasly",
                            "lap": 40,
                            "stop": 3,
                            "time": "19:10:52",
                            "duration": "24.736"
                        },
                        {
                            "driverRef": "tsunoda",
                            "lap": 40,
                            "stop": 3,
                            "time": "19:10:53",
                            "duration": "25.818"
                        },
                        {
                            "driverRef": "sargeant",
                            "lap": 40,
                            "stop": 3,
                            "time": "19:11:05",
                            "duration": "25.047"
                        },
                        {
                            "driverRef": "kevin_magnussen",
                            "lap": 40,
                            "stop": 3,
                            "time": "19:11:20",
                            "duration": "25.125"
                        },
                        {
                            "driverRef": "hulkenberg",
                            "lap": 40,
                            "stop": 3,
                            "time": "19:11:26",
                            "duration": "27.080"
                        },
                        {
                            "driverRef": "norris",
                            "lap": 47,
                            "stop": 5,
                            "time": "19:23:45",
                            "duration": "30.710"
                        },
                        {
                            "driverRef": "zhou",
                            "lap": 54,
                            "stop": 3,
                            "time": "19:34:31",
                            "duration": "30.167"
                        },
                        {
                            "driverRef": "norris",
                            "lap": 54,
                            "stop": 6,
                            "time": "19:35:35",
                            "duration": "31.494"
                        }
                    ]
                }
    });

    Test::new(
        r#"{
            pitStops(options: { year: 2023, round: 1 }, pagination: { limit: 30, page: 2 }) {
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
                pitStops {
                    driverRef
                    lap
                    stop
                    time
                    duration
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}
