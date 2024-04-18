use shared::prelude::*;

pub mod common;

#[test]
fn test_get_races_by_year() {
    common::Test::<&[StaticRace], Vec<RaceResponse>>::new(
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
    .test_ok();
}

#[test]
fn test_get_races_by_year_and_round() {
    common::Test::<&[StaticRace], Vec<RaceResponse>>::new(
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
    .test_ok();
}

#[derive(Debug)]
struct StaticRace<'a> {
    season: i32,
    round: i32,
    name: &'a str,
    date: &'a str,
    time: Option<&'a str>,
    url: Option<&'a str>,
    fp1: Option<StaticDateAndTime<'a>>,
    fp2: Option<StaticDateAndTime<'a>>,
    fp3: Option<StaticDateAndTime<'a>>,
    quali: Option<StaticDateAndTime<'a>>,
    sprint: Option<StaticDateAndTime<'a>>,

    circuit: StaticCircuit<'a>,
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
struct StaticDateAndTime<'a> {
    date: &'a str,
    time: &'a str,
}

fn compare<T, U>(t: Option<T>, u: Option<U>) -> bool
where
    T: PartialEq<U>,
{
    match (t, u) {
        (Some(t), Some(u)) => t == u,
        (None, None) => true,
        _ => false,
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

impl PartialEq<RaceResponse> for StaticRace<'_> {
    fn eq(&self, other: &RaceResponse) -> bool {
        let circuit = &other.circuit;
        let other = &other.race;

        self.season == other.season
            && self.round == other.round
            && self.name == other.name
            && common::parse_date(self.date) == other.date
            && self.time.map(common::parse_time) == other.time
            && self.url == other.url.as_deref()
            && compare(self.fp1.as_ref(), other.fp1.as_ref())
            && compare(self.fp2.as_ref(), other.fp2.as_ref())
            && compare(self.fp3.as_ref(), other.fp3.as_ref())
            && compare(self.quali.as_ref(), other.quali.as_ref())
            && compare(self.sprint.as_ref(), other.sprint.as_ref())
            && self.circuit == *circuit
    }
}

impl PartialEq<DateAndTime> for StaticDateAndTime<'_> {
    fn eq(&self, other: &DateAndTime) -> bool {
        let date = common::parse_date(self.date);
        let time = common::parse_time(self.time);

        date == other.date && time == other.time
    }
}

macro_rules! __races_impl {
    (@fields [$($fields:tt)*];
        "season": $season:expr,
        "round": $round:expr,
        "name": $name:literal,
        "date": $date:literal,
        $($tt:tt)*
    ) => {
        __races_impl!(@fields [$($fields)*
            season: $season,
            round: $round,
            name: $name,
            date: $date,
        ]; $($tt)*)
    };
    (@fields [$($fields:tt)*];
        "time": $time:literal,
        $($tt:tt)*
    ) => {
        __races_impl!(@fields [$($fields)* time: Some($time),]; $($tt)*)
    };
    (@fields [$($fields:tt)*];
        "url": $url:literal,
        $($tt:tt)*
    ) => {
        __races_impl!(@fields [$($fields)* url: Some($url),]; $($tt)*)
    };
    (@fields [$($fields:tt)*];
        "fp1": {
            "date": $date:literal,
            "time": $time:literal
        },
        $($tt:tt)*
    ) => {
        __races_impl!(@fields [$($fields)* fp1: Some(StaticDateAndTime { date: $date, time: $time }),]; $($tt)*)
    };
    (@fields [$($fields:tt)*];
        "fp2": {
            "date": $date:literal,
            "time": $time:literal
        },
        $($tt:tt)*
    ) => {
        __races_impl!(@fields [$($fields)* fp2: Some(StaticDateAndTime { date: $date, time: $time }),]; $($tt)*)
    };
    (@fields [$($fields:tt)*];
        "fp3": {
            "date": $date:literal,
            "time": $time:literal
        },
        $($tt:tt)*
    ) => {
        __races_impl!(@fields [$($fields)* fp3: Some(StaticDateAndTime { date: $date, time: $time }), sprint: None,]; $($tt)*)
    };
    (@fields [$($fields:tt)*];
        "quali": {
            "date": $date:literal,
            "time": $time:literal
        },
        $($tt:tt)*
    ) => {
        __races_impl!(@fields [$($fields)* quali: Some(StaticDateAndTime { date: $date, time: $time }),]; $($tt)*)
    };
    (@fields [$($fields:tt)*];
        "sprint": {
            "date": $date:literal,
            "time": $time:literal
        },
        $($tt:tt)*
    ) => {
        __races_impl!(@fields [$($fields)* sprint: Some(StaticDateAndTime { date: $date, time: $time }), fp3: None,]; $($tt)*)
    };
    (@fields [$($fields:tt)*];
        "circuit": {
            "circuit_ref": $ref:literal,
            "name": $name:literal,
            "location": $location:literal,
            "country": $country:literal,
            "lat": $lat:expr,
            "lng": $lng:expr,
            "alt": $alt:expr,
            "url": $url:literal
        }
    ) => {
        StaticRace {
            $($fields)*
            circuit: StaticCircuit {
                circuit_ref: $ref,
                name: $name,
                location: Some($location),
                country: Some($country),
                lat: Some($lat),
                lng: Some($lng),
                alt: Some($alt),
                url: $url,
            },
        }
    };
    (@internal [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@internal [$($expr:expr),*]; $(,)?{
        $($fields:tt)*
    } $($tt:tt)*) => {
        __races_impl!(@internal [$($expr,)* __races_impl!(@fields []; $($fields)*)]; $($tt)*)
    }
}

macro_rules! races_from_json {
    ($($tt:tt)*) => {
        __races_impl!(@internal []; $($tt)*)
    };
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
