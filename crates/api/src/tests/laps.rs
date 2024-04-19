use shared::prelude::*;

use super::common::models::*;
use super::common::Test;
use crate::laps_from_json;

#[tokio::test]
async fn test_get_laps() {
    Test::<StaticLaps, LapsResponse>::new(
        "/api/f1/laps?year=2023&round=1",
        Series::F1,
        BAHRAIN_2023_LAPS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 36,
        total: 1055,
    }))
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_laps_by_driver_ref() {
    Test::<StaticLaps, LapsResponse>::new(
        "/api/f1/laps?year=2023&round=1&driver_ref=max_verstappen",
        Series::F1,
        BAHRAIN_VERSTAPPEN_LAPS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 2,
        total: 57,
    }))
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_laps_by_driver_ref_and_page() {
    Test::<StaticLaps, LapsResponse>::new(
        "/api/f1/laps?year=2023&round=1&driver_ref=max_verstappen&page=2",
        Series::F1,
        BAHRAIN_VERSTAPPEN_LAPS_PAGE_2,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 2,
        max_page: 2,
        total: 57,
    }))
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_laps_by_driver_ref_and_lap_number() {
    Test::<StaticLaps, LapsResponse>::new(
        "/api/f1/laps?year=2023&round=1&driver_ref=max_verstappen&lap_number=10",
        Series::F1,
        BAHRAIN_VERSTAPPEN_LAP_10,
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

const BAHRAIN_2023_LAPS: StaticLaps = laps_from_json! {
    {
            "url": "https://en.wikipedia.org/wiki/2023_Bahrain_Grand_Prix",
            "race_name": "Bahrain Grand Prix",
            "date": "2023-03-05",
            "time": "15:00:00",
            "circuit": {
                "circuit_ref": "bahrain",
                "name": "Bahrain International Circuit",
                "location": "Sakhir",
                "country": "Bahrain",
                "lat": 26.0325,
                "lng": 50.5106,
                "alt": 7,
                "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
            },
            "laps": [
                {
                    "number": 1,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:39.019"
                        },
                        {
                            "driver_ref": "leclerc",
                            "position": 2,
                            "time": "1:40.230"
                        },
                        {
                            "driver_ref": "perez",
                            "position": 3,
                            "time": "1:41.063"
                        },
                        {
                            "driver_ref": "sainz",
                            "position": 4,
                            "time": "1:41.659"
                        },
                        {
                            "driver_ref": "hamilton",
                            "position": 5,
                            "time": "1:42.288"
                        },
                        {
                            "driver_ref": "russell",
                            "position": 6,
                            "time": "1:42.662"
                        },
                        {
                            "driver_ref": "alonso",
                            "position": 7,
                            "time": "1:43.608"
                        },
                        {
                            "driver_ref": "bottas",
                            "position": 8,
                            "time": "1:44.154"
                        },
                        {
                            "driver_ref": "stroll",
                            "position": 9,
                            "time": "1:44.670"
                        },
                        {
                            "driver_ref": "norris",
                            "position": 10,
                            "time": "1:45.364"
                        },
                        {
                            "driver_ref": "ocon",
                            "position": 11,
                            "time": "1:45.967"
                        },
                        {
                            "driver_ref": "albon",
                            "position": 12,
                            "time": "1:46.453"
                        },
                        {
                            "driver_ref": "sargeant",
                            "position": 13,
                            "time": "1:46.855"
                        },
                        {
                            "driver_ref": "hulkenberg",
                            "position": 14,
                            "time": "1:47.339"
                        },
                        {
                            "driver_ref": "tsunoda",
                            "position": 15,
                            "time": "1:47.791"
                        },
                        {
                            "driver_ref": "piastri",
                            "position": 16,
                            "time": "1:48.214"
                        },
                        {
                            "driver_ref": "zhou",
                            "position": 17,
                            "time": "1:48.467"
                        },
                        {
                            "driver_ref": "kevin_magnussen",
                            "position": 18,
                            "time": "1:49.330"
                        },
                        {
                            "driver_ref": "gasly",
                            "position": 19,
                            "time": "1:49.649"
                        },
                        {
                            "driver_ref": "de_vries",
                            "position": 20,
                            "time": "1:49.959"
                        }
                    ]
                },
                {
                    "number": 2,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.974"
                        },
                        {
                            "driver_ref": "leclerc",
                            "position": 2,
                            "time": "1:38.750"
                        },
                        {
                            "driver_ref": "perez",
                            "position": 3,
                            "time": "1:38.862"
                        },
                        {
                            "driver_ref": "sainz",
                            "position": 4,
                            "time": "1:38.933"
                        },
                        {
                            "driver_ref": "hamilton",
                            "position": 5,
                            "time": "1:39.166"
                        },
                        {
                            "driver_ref": "russell",
                            "position": 6,
                            "time": "1:39.590"
                        },
                        {
                            "driver_ref": "alonso",
                            "position": 7,
                            "time": "1:39.792"
                        },
                        {
                            "driver_ref": "bottas",
                            "position": 8,
                            "time": "1:39.956"
                        },
                        {
                            "driver_ref": "stroll",
                            "position": 9,
                            "time": "1:39.892"
                        },
                        {
                            "driver_ref": "norris",
                            "position": 10,
                            "time": "1:40.433"
                        }
                    ]
                }
            ]
        }
};

const BAHRAIN_VERSTAPPEN_LAPS: StaticLaps = laps_from_json! {
    {
            "url": "https://en.wikipedia.org/wiki/2023_Bahrain_Grand_Prix",
            "race_name": "Bahrain Grand Prix",
            "date": "2023-03-05",
            "time": "15:00:00",
            "circuit": {
                "circuit_ref": "bahrain",
                "name": "Bahrain International Circuit",
                "location": "Sakhir",
                "country": "Bahrain",
                "lat": 26.0325,
                "lng": 50.5106,
                "alt": 7,
                "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
            },
            "laps": [
                {
                    "number": 1,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:39.019"
                        }
                    ]
                },
                {
                    "number": 2,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.974"
                        }
                    ]
                },
                {
                    "number": 3,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:38.006"
                        }
                    ]
                },
                {
                    "number": 4,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.976"
                        }
                    ]
                },
                {
                    "number": 5,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:38.035"
                        }
                    ]
                },
                {
                    "number": 6,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.986"
                        }
                    ]
                },
                {
                    "number": 7,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:38.021"
                        }
                    ]
                },
                {
                    "number": 8,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:38.154"
                        }
                    ]
                },
                {
                    "number": 9,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:38.278"
                        }
                    ]
                },
                {
                    "number": 10,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:38.369"
                        }
                    ]
                },
                {
                    "number": 11,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:38.483"
                        }
                    ]
                },
                {
                    "number": 12,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:38.591"
                        }
                    ]
                },
                {
                    "number": 13,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:38.482"
                        }
                    ]
                },
                {
                    "number": 14,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:41.295"
                        }
                    ]
                },
                {
                    "number": 15,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 2,
                            "time": "1:58.378"
                        }
                    ]
                },
                {
                    "number": 16,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 2,
                            "time": "1:37.801"
                        }
                    ]
                },
                {
                    "number": 17,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 2,
                            "time": "1:37.648"
                        }
                    ]
                },
                {
                    "number": 18,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.614"
                        }
                    ]
                },
                {
                    "number": 19,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.712"
                        }
                    ]
                },
                {
                    "number": 20,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.788"
                        }
                    ]
                },
                {
                    "number": 21,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.582"
                        }
                    ]
                },
                {
                    "number": 22,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.651"
                        }
                    ]
                },
                {
                    "number": 23,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.552"
                        }
                    ]
                },
                {
                    "number": 24,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.387"
                        }
                    ]
                },
                {
                    "number": 25,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.372"
                        }
                    ]
                },
                {
                    "number": 26,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.682"
                        }
                    ]
                },
                {
                    "number": 27,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.379"
                        }
                    ]
                },
                {
                    "number": 28,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.651"
                        }
                    ]
                },
                {
                    "number": 29,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.571"
                        }
                    ]
                },
                {
                    "number": 30,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.549"
                        }
                    ]
                }
            ]
        }
};

const BAHRAIN_VERSTAPPEN_LAPS_PAGE_2: StaticLaps = laps_from_json! {
    {
            "url": "https://en.wikipedia.org/wiki/2023_Bahrain_Grand_Prix",
            "race_name": "Bahrain Grand Prix",
            "date": "2023-03-05",
            "time": "15:00:00",
            "circuit": {
                "circuit_ref": "bahrain",
                "name": "Bahrain International Circuit",
                "location": "Sakhir",
                "country": "Bahrain",
                "lat": 26.0325,
                "lng": 50.5106,
                "alt": 7,
                "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
            },
            "laps": [
                {
                    "number": 31,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.431"
                        }
                    ]
                },
                {
                    "number": 32,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.691"
                        }
                    ]
                },
                {
                    "number": 33,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.686"
                        }
                    ]
                },
                {
                    "number": 34,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.839"
                        }
                    ]
                },
                {
                    "number": 35,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.850"
                        }
                    ]
                },
                {
                    "number": 36,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:40.964"
                        }
                    ]
                },
                {
                    "number": 37,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:58.490"
                        }
                    ]
                },
                {
                    "number": 38,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:36.297"
                        }
                    ]
                },
                {
                    "number": 39,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:36.446"
                        }
                    ]
                },
                {
                    "number": 40,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:36.942"
                        }
                    ]
                },
                {
                    "number": 41,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "2:00.495"
                        }
                    ]
                },
                {
                    "number": 42,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:44.824"
                        }
                    ]
                },
                {
                    "number": 43,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:36.890"
                        }
                    ]
                },
                {
                    "number": 44,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:36.236"
                        }
                    ]
                },
                {
                    "number": 45,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.015"
                        }
                    ]
                },
                {
                    "number": 46,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:36.701"
                        }
                    ]
                },
                {
                    "number": 47,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.009"
                        }
                    ]
                },
                {
                    "number": 48,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:36.918"
                        }
                    ]
                },
                {
                    "number": 49,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:36.962"
                        }
                    ]
                },
                {
                    "number": 50,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.250"
                        }
                    ]
                },
                {
                    "number": 51,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.427"
                        }
                    ]
                },
                {
                    "number": 52,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:36.919"
                        }
                    ]
                },
                {
                    "number": 53,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:36.890"
                        }
                    ]
                },
                {
                    "number": 54,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:36.678"
                        }
                    ]
                },
                {
                    "number": 55,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:36.322"
                        }
                    ]
                },
                {
                    "number": 56,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:37.205"
                        }
                    ]
                },
                {
                    "number": 57,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:36.373"
                        }
                    ]
                }
            ]
        }
};

const BAHRAIN_VERSTAPPEN_LAP_10: StaticLaps = laps_from_json! {
    {
            "url": "https://en.wikipedia.org/wiki/2023_Bahrain_Grand_Prix",
            "race_name": "Bahrain Grand Prix",
            "date": "2023-03-05",
            "time": "15:00:00",
            "circuit": {
                "circuit_ref": "bahrain",
                "name": "Bahrain International Circuit",
                "location": "Sakhir",
                "country": "Bahrain",
                "lat": 26.0325,
                "lng": 50.5106,
                "alt": 7,
                "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
            },
            "laps": [
                {
                    "number": 10,
                    "timings": [
                        {
                            "driver_ref": "max_verstappen",
                            "position": 1,
                            "time": "1:38.369"
                        }
                    ]
                }
            ]
        }
};
