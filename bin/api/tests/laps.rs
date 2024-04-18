use shared::prelude::*;

pub mod common;

#[test]
fn test_get_laps() {
    common::Test::<StaticLaps, LapsResponse>::new(
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
    .test_ok();
}

#[test]
fn test_get_laps_by_driver_ref() {
    common::Test::<StaticLaps, LapsResponse>::new(
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
    .test_ok();
}

#[test]
fn test_get_laps_by_driver_ref_and_page() {
    common::Test::<StaticLaps, LapsResponse>::new(
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
    .test_ok();
}

#[test]
fn test_get_laps_by_driver_ref_and_lap_number() {
    common::Test::<StaticLaps, LapsResponse>::new(
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
    .test_ok();
}

#[derive(Debug)]
struct StaticLaps<'a> {
    url: Option<&'a str>,
    race_name: &'a str,
    date: &'a str,
    time: Option<&'a str>,
    circuit: StaticCircuit<'a>,
    laps: &'a [StaticLap<'a>],
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

#[derive(Debug)]
struct StaticLap<'a> {
    number: i32,
    timings: &'a [StaticTiming<'a>],
}

#[derive(Debug)]
struct StaticTiming<'a> {
    driver_ref: &'a str,
    position: Option<i32>,
    time: Option<&'a str>,
}

impl PartialEq<LapsResponse> for StaticLaps<'_> {
    fn eq(&self, other: &LapsResponse) -> bool {
        let date = chrono::NaiveDate::parse_from_str(self.date, "%Y-%m-%d").unwrap();
        let time = self
            .time
            .map(|t| chrono::NaiveTime::parse_from_str(t, "%H:%M:%S").unwrap());

        self.url == other.url.as_deref()
            && self.race_name == other.race_name
            && date == other.date
            && time == other.time
            && self.circuit == other.circuit
            && self.laps == other.laps
    }
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

impl PartialEq<Lap> for StaticLap<'_> {
    fn eq(&self, other: &Lap) -> bool {
        self.number == other.number && self.timings == other.timings
    }
}

impl PartialEq<LapTiming> for StaticTiming<'_> {
    fn eq(&self, other: &LapTiming) -> bool {
        self.driver_ref == other.driver_ref
            && self.position == other.position
            && self.time == other.time.as_deref()
    }
}

macro_rules! __laps_impl {
    (@timings [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@timings [$($expr:expr),*]; $(,)?{
        "driver_ref": $ref:literal,
        "position": $position:expr,
        "time": $time:literal
    } $($tt:tt)*) => {
        __laps_impl!(@timings [$($expr,)* StaticTiming {
            driver_ref: $ref,
            position: Some($position),
            time: Some($time)
        }]; $($tt)*)
    };
    (@laps [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@laps [$($expr:expr),*]; $(,)?{
        "number": $number:expr,
        "timings": [$($tt:tt)*]
    } $($lt:tt)*) => {
        __laps_impl!(@laps [$($expr,)* StaticLap {
            number: $number,
            timings: &__laps_impl![@timings []; $($tt)*]
        }]; $($lt)*)
    };
    (@circuit
        "circuit_ref": $ref:literal,
        "name": $name:literal,
        "location": $location:literal,
        "country": $country:literal,
        "lat": $lat:expr,
        "lng": $lng:expr,
        "alt": $alt:expr,
        "url": $url:literal
    ) => {
        StaticCircuit {
            circuit_ref: $ref,
            name: $name,
            location: Some($location),
            country: Some($country),
            lat: Some($lat),
            lng: Some($lng),
            alt: Some($alt),
            url: $url,
        }
    };
    (@internal {
        "url": $url:literal,
        "race_name": $name:literal,
        "date": $date:literal,
        "time": $time:literal,
        "circuit": {$($ct:tt)*},
        "laps": [$($lt:tt)*]
    }) => {
        StaticLaps {
            url: Some($url),
            race_name: $name,
            date: $date,
            time: Some($time),
            circuit: __laps_impl!(@circuit $($ct)*),
            laps: &__laps_impl!(@laps []; $($lt)*),
        }
    };
}

macro_rules! laps_from_json {
    ($($tt:tt)*) => {
        __laps_impl!(@internal $($tt)*)
    };
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
