use shared::prelude::*;

pub mod common;

#[test]
fn test_get_pit_stops() {
    common::Test::<StaticPitStops, PitStopsResponse>::new(
        "/api/f1/pit_stops?year=2023&round=1",
        Series::F1,
        BAHRAIN_2023_STOPS_PAGE_1,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 2,
        total: 50,
    }))
    .test_ok();
}

#[test]
fn test_get_pit_stops_by_page() {
    common::Test::<StaticPitStops, PitStopsResponse>::new(
        "/api/f1/pit_stops?year=2023&round=1&page=2",
        Series::F1,
        BAHRAIN_2023_STOPS_PAGE_2,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 2,
        max_page: 2,
        total: 50,
    }))
    .test_ok();
}

#[derive(Debug)]
struct StaticPitStops<'a> {
    url: Option<&'a str>,
    race_name: &'a str,
    date: &'a str,
    time: Option<&'a str>,
    circuit: StaticCircuit<'a>,
    pit_stops: &'a [StaticPitStop<'a>],
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
struct StaticPitStop<'a> {
    driver_ref: &'a str,
    lap: i32,
    stop: i32,
    time: &'a str,
    duration: Option<&'a str>,
}

impl PartialEq<PitStopsResponse> for StaticPitStops<'_> {
    fn eq(&self, other: &PitStopsResponse) -> bool {
        let date = chrono::NaiveDate::parse_from_str(self.date, "%Y-%m-%d").unwrap();
        let time = self
            .time
            .map(|t| chrono::NaiveTime::parse_from_str(t, "%H:%M:%S").unwrap());

        self.url == other.url.as_deref()
            && self.race_name == other.race_name
            && date == other.date
            && time == other.time
            && self.circuit == other.circuit
            && self.pit_stops == other.pit_stops
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

impl PartialEq<PitStop> for StaticPitStop<'_> {
    fn eq(&self, other: &PitStop) -> bool {
        let time = chrono::NaiveTime::parse_from_str(self.time, "%H:%M:%S").unwrap();

        self.driver_ref == other.driver_ref
            && self.lap == other.lap
            && self.stop == other.stop
            && time == other.time
            && self.duration == other.duration.as_deref()
    }
}

macro_rules! __stops_impl {
    (@stops [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@stops [$($expr:expr),*]; $(,)?{
        "driver_ref": $ref:literal,
        "lap": $lap:expr,
        "stop": $stop:expr,
        "time": $time:literal,
        "duration": $duration:literal
    } $($tt:tt)*) => {
        __stops_impl!(@stops [$($expr,)* StaticPitStop {
            driver_ref: $ref,
            lap: $lap,
            stop: $stop,
            time: $time,
            duration: Some($duration),
        }]; $($tt)*)
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
        "pit_stops": [$($tt:tt)*]
    }) => {
        StaticPitStops {
            url: Some($url),
            race_name: $name,
            date: $date,
            time: Some($time),
            circuit: __stops_impl!(@circuit $($ct)*),
            pit_stops: &__stops_impl!(@stops []; $($tt)*),
        }
    };
}

macro_rules! stops_from_json {
    ($($tt:tt)*) => {
        __stops_impl!(@internal $($tt)*)
    };
}

const BAHRAIN_2023_STOPS_PAGE_1: StaticPitStops = stops_from_json! {
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
            "pit_stops": [
                {
                    "driver_ref": "gasly",
                    "lap": 9,
                    "stop": 1,
                    "time": "18:18:56",
                    "duration": "25.885"
                },
                {
                    "driver_ref": "norris",
                    "lap": 10,
                    "stop": 1,
                    "time": "18:20:31",
                    "duration": "32.766"
                },
                {
                    "driver_ref": "tsunoda",
                    "lap": 10,
                    "stop": 1,
                    "time": "18:20:34",
                    "duration": "25.267"
                },
                {
                    "driver_ref": "bottas",
                    "lap": 11,
                    "stop": 1,
                    "time": "18:22:06",
                    "duration": "25.399"
                },
                {
                    "driver_ref": "albon",
                    "lap": 11,
                    "stop": 1,
                    "time": "18:22:12",
                    "duration": "25.054"
                },
                {
                    "driver_ref": "hulkenberg",
                    "lap": 11,
                    "stop": 1,
                    "time": "18:22:21",
                    "duration": "26.075"
                },
                {
                    "driver_ref": "de_vries",
                    "lap": 11,
                    "stop": 1,
                    "time": "18:22:21",
                    "duration": "25.018"
                },
                {
                    "driver_ref": "hamilton",
                    "lap": 12,
                    "stop": 1,
                    "time": "18:23:40",
                    "duration": "24.682"
                },
                {
                    "driver_ref": "ocon",
                    "lap": 12,
                    "stop": 1,
                    "time": "18:23:50",
                    "duration": "24.966"
                },
                {
                    "driver_ref": "sargeant",
                    "lap": 12,
                    "stop": 1,
                    "time": "18:23:55",
                    "duration": "25.600"
                },
                {
                    "driver_ref": "zhou",
                    "lap": 12,
                    "stop": 1,
                    "time": "18:24:01",
                    "duration": "25.787"
                },
                {
                    "driver_ref": "leclerc",
                    "lap": 13,
                    "stop": 1,
                    "time": "18:25:06",
                    "duration": "24.345"
                },
                {
                    "driver_ref": "sainz",
                    "lap": 13,
                    "stop": 1,
                    "time": "18:25:17",
                    "duration": "24.500"
                },
                {
                    "driver_ref": "russell",
                    "lap": 13,
                    "stop": 1,
                    "time": "18:25:24",
                    "duration": "27.062"
                },
                {
                    "driver_ref": "max_verstappen",
                    "lap": 14,
                    "stop": 1,
                    "time": "18:26:35",
                    "duration": "24.289"
                },
                {
                    "driver_ref": "alonso",
                    "lap": 14,
                    "stop": 1,
                    "time": "18:27:02",
                    "duration": "25.800"
                },
                {
                    "driver_ref": "stroll",
                    "lap": 15,
                    "stop": 1,
                    "time": "18:28:45",
                    "duration": "25.449"
                },
                {
                    "driver_ref": "kevin_magnussen",
                    "lap": 15,
                    "stop": 1,
                    "time": "18:29:10",
                    "duration": "25.439"
                },
                {
                    "driver_ref": "ocon",
                    "lap": 15,
                    "stop": 2,
                    "time": "18:29:11",
                    "duration": "41.462"
                },
                {
                    "driver_ref": "perez",
                    "lap": 17,
                    "stop": 1,
                    "time": "18:31:42",
                    "duration": "24.264"
                },
                {
                    "driver_ref": "norris",
                    "lap": 17,
                    "stop": 2,
                    "time": "18:32:38",
                    "duration": "33.661"
                },
                {
                    "driver_ref": "gasly",
                    "lap": 25,
                    "stop": 2,
                    "time": "18:45:48",
                    "duration": "24.879"
                },
                {
                    "driver_ref": "albon",
                    "lap": 26,
                    "stop": 2,
                    "time": "18:47:25",
                    "duration": "24.920"
                },
                {
                    "driver_ref": "tsunoda",
                    "lap": 26,
                    "stop": 2,
                    "time": "18:47:26",
                    "duration": "24.372"
                },
                {
                    "driver_ref": "hulkenberg",
                    "lap": 26,
                    "stop": 2,
                    "time": "18:47:41",
                    "duration": "37.769"
                },
                {
                    "driver_ref": "de_vries",
                    "lap": 27,
                    "stop": 2,
                    "time": "18:49:16",
                    "duration": "25.234"
                },
                {
                    "driver_ref": "norris",
                    "lap": 27,
                    "stop": 3,
                    "time": "18:49:35",
                    "duration": "33.467"
                },
                {
                    "driver_ref": "bottas",
                    "lap": 29,
                    "stop": 2,
                    "time": "18:52:16",
                    "duration": "25.643"
                },
                {
                    "driver_ref": "kevin_magnussen",
                    "lap": 29,
                    "stop": 2,
                    "time": "18:52:40",
                    "duration": "25.344"
                },
                {
                    "driver_ref": "hamilton",
                    "lap": 30,
                    "stop": 2,
                    "time": "18:53:36",
                    "duration": "24.690"
                }
            ]
        }
};

const BAHRAIN_2023_STOPS_PAGE_2: StaticPitStops = stops_from_json! {
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
            "pit_stops": [
                {
                    "driver_ref": "stroll",
                    "lap": 30,
                    "stop": 2,
                    "time": "18:53:45",
                    "duration": "24.639"
                },
                {
                    "driver_ref": "sargeant",
                    "lap": 30,
                    "stop": 2,
                    "time": "18:54:09",
                    "duration": "25.670"
                },
                {
                    "driver_ref": "sainz",
                    "lap": 31,
                    "stop": 2,
                    "time": "18:55:11",
                    "duration": "24.227"
                },
                {
                    "driver_ref": "russell",
                    "lap": 31,
                    "stop": 2,
                    "time": "18:55:21",
                    "duration": "24.406"
                },
                {
                    "driver_ref": "zhou",
                    "lap": 32,
                    "stop": 2,
                    "time": "18:57:31",
                    "duration": "25.282"
                },
                {
                    "driver_ref": "ocon",
                    "lap": 32,
                    "stop": 3,
                    "time": "18:58:03",
                    "duration": "44.518"
                },
                {
                    "driver_ref": "leclerc",
                    "lap": 33,
                    "stop": 2,
                    "time": "18:58:16",
                    "duration": "24.644"
                },
                {
                    "driver_ref": "perez",
                    "lap": 34,
                    "stop": 2,
                    "time": "18:59:45",
                    "duration": "25.091"
                },
                {
                    "driver_ref": "alonso",
                    "lap": 34,
                    "stop": 2,
                    "time": "19:00:09",
                    "duration": "24.869"
                },
                {
                    "driver_ref": "max_verstappen",
                    "lap": 36,
                    "stop": 2,
                    "time": "19:02:47",
                    "duration": "24.910"
                },
                {
                    "driver_ref": "norris",
                    "lap": 37,
                    "stop": 4,
                    "time": "19:06:27",
                    "duration": "33.236"
                },
                {
                    "driver_ref": "albon",
                    "lap": 40,
                    "stop": 3,
                    "time": "19:10:51",
                    "duration": "24.458"
                },
                {
                    "driver_ref": "gasly",
                    "lap": 40,
                    "stop": 3,
                    "time": "19:10:52",
                    "duration": "24.736"
                },
                {
                    "driver_ref": "tsunoda",
                    "lap": 40,
                    "stop": 3,
                    "time": "19:10:53",
                    "duration": "25.818"
                },
                {
                    "driver_ref": "sargeant",
                    "lap": 40,
                    "stop": 3,
                    "time": "19:11:05",
                    "duration": "25.047"
                },
                {
                    "driver_ref": "kevin_magnussen",
                    "lap": 40,
                    "stop": 3,
                    "time": "19:11:20",
                    "duration": "25.125"
                },
                {
                    "driver_ref": "hulkenberg",
                    "lap": 40,
                    "stop": 3,
                    "time": "19:11:26",
                    "duration": "27.080"
                },
                {
                    "driver_ref": "norris",
                    "lap": 47,
                    "stop": 5,
                    "time": "19:23:45",
                    "duration": "30.710"
                },
                {
                    "driver_ref": "zhou",
                    "lap": 54,
                    "stop": 3,
                    "time": "19:34:31",
                    "duration": "30.167"
                },
                {
                    "driver_ref": "norris",
                    "lap": 54,
                    "stop": 6,
                    "time": "19:35:35",
                    "duration": "31.494"
                }
            ]
        }
};
