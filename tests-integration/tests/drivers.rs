use rocket::http::Status;

use shared::prelude::*;

pub mod common;

struct TestDriver {
    uri: &'static str,
    series: Series,
    pagination: Option<Pagination>,
    expected: &'static [StaticDriver<'static>],
}

fn test_drivers_ok(test: TestDriver) {
    let client = common::setup();

    let resp = common::get(&client, test.uri);
    assert_eq!(resp.status(), Status::Ok);
    let json = resp.into_json::<Response<Vec<Driver>>>().unwrap();

    assert_eq!(json.series, test.series);
    assert_eq!(json.pagination, test.pagination);
    assert_eq!(json.data.len(), test.expected.len());

    json.data
        .iter()
        .take(test.expected.len())
        .zip(test.expected)
        .for_each(|(l, r)| assert_eq!(r, l));
}

#[test]
fn test_get_drivers_by_circuit_ref() {
    let test = TestDriver {
        uri: "/api/f1/drivers?circuit_ref=spa",
        series: Series::F1,
        pagination: Some(Pagination {
            limit: 30,
            page: 1,
            max_page: 11,
            total: 326,
        }),
        expected: &SPA_DRIVERS,
    };

    test_drivers_ok(test);
}

#[test]
fn test_get_drivers_by_circuit_ref_and_result() {
    let test = TestDriver {
        uri: "/api/f1/drivers?result=1&circuit_ref=spa",
        series: Series::F1,
        pagination: Some(Pagination {
            limit: 30,
            page: 1,
            max_page: 1,
            total: 28,
        }),
        expected: &SPA_WINNERS_DRIVERS,
    };

    test_drivers_ok(test);
}

#[derive(Debug)]
struct StaticDriver<'a> {
    driver_ref: &'a str,
    number: Option<i32>,
    code: Option<&'a str>,
    forename: &'a str,
    surname: &'a str,
    dob: Option<&'a str>,
    nationality: Option<&'a str>,
    url: &'a str,
}

impl PartialEq<Driver> for StaticDriver<'_> {
    fn eq(&self, other: &Driver) -> bool {
        self.driver_ref.eq(&other.driver_ref)
            && self.number.eq(&other.number)
            && self.code.eq(&other.code.as_deref())
            && self.forename.eq(&other.forename)
            && self.surname.eq(&other.surname)
            && self
                .dob
                .map(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").expect("invalid date"))
                .eq(&other.dob)
            && self.nationality.eq(&other.nationality.as_deref())
            && self.url.eq(&other.url)
    }
}

impl PartialEq<&Driver> for StaticDriver<'_> {
    fn eq(&self, other: &&Driver) -> bool {
        self.driver_ref.eq(&other.driver_ref)
            && self.number.eq(&other.number)
            && self.code.eq(&other.code.as_deref())
            && self.forename.eq(&other.forename)
            && self.surname.eq(&other.surname)
            && self
                .dob
                .map(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").expect("invalid date"))
                .eq(&other.dob)
            && self.nationality.eq(&other.nationality.as_deref())
            && self.url.eq(&other.url)
    }
}

macro_rules! __drivers_from_json_impl {
    (@internal [$($drivers:expr),*];) => {
        [$($drivers),*]
    };
    (@internal [$($drivers:expr),*];) => {
        [$($drivers),*]
    };
    (@internal [$($drivers:expr),*]; $(,)?{
        "driver_ref": $ref:literal,
        "forename": $forename:literal,
        "surname": $surname:literal,
        "dob": $dob:literal,
        "nationality": $nationality:literal,
        "url": $url:literal
    } $($tt:tt)*) => {
        __drivers_from_json_impl!(@internal [$($drivers,)* StaticDriver {
            driver_ref: $ref,
            number: None,
            code: None,
            forename: $forename,
            surname: $surname,
            dob: Some($dob),
            nationality: Some($nationality),
            url: $url,
        }]; $($tt)*)
    };
    (@internal [$($drivers:expr),*]; $(,)?{
        "driver_ref": $ref:literal,
        "code": $code:literal,
        "forename": $forename:literal,
        "surname": $surname:literal,
        "dob": $dob:literal,
        "nationality": $nationality:literal,
        "url": $url:literal
    } $($tt:tt)*) => {
        __drivers_from_json_impl!(@internal [$($drivers,)* StaticDriver {
            driver_ref: $ref,
            number: None,
            code: Some($code),
            forename: $forename,
            surname: $surname,
            dob: Some($dob),
            nationality: Some($nationality),
            url: $url,
        }]; $($tt)*)
    };
    (@internal [$($drivers:expr),*]; $(,)?{
        "driver_ref": $ref:literal,
        "number": $number:expr,
        "code": $code:literal,
        "forename": $forename:literal,
        "surname": $surname:literal,
        "dob": $dob:literal,
        "nationality": $nationality:literal,
        "url": $url:literal
    } $($tt:tt)*) => {
        __drivers_from_json_impl!(@internal [$($drivers,)* StaticDriver {
            driver_ref: $ref,
            number: Some($number),
            code: Some($code),
            forename: $forename,
            surname: $surname,
            dob: Some($dob),
            nationality: Some($nationality),
            url: $url,
        }]; $($tt)*)
    };
}

macro_rules! drivers_from_json {
    ($($tt:tt)*) => {
        __drivers_from_json_impl!(@internal []; $($tt)*)
    };
}

use __drivers_from_json_impl;
use drivers_from_json;

const SPA_DRIVERS: [StaticDriver; 30] = drivers_from_json![
    {
        "driver_ref": "massa",
        "number": 19,
        "code": "MAS",
        "forename": "Felipe",
        "surname": "Massa",
        "dob": "1981-04-25",
        "nationality": "Brazilian",
        "url": "http://en.wikipedia.org/wiki/Felipe_Massa"
    },
    {
        "driver_ref": "heidfeld",
        "code": "HEI",
        "forename": "Nick",
        "surname": "Heidfeld",
        "dob": "1977-05-10",
        "nationality": "German",
        "url": "http://en.wikipedia.org/wiki/Nick_Heidfeld"
    },
    {
        "driver_ref": "hamilton",
        "number": 44,
        "code": "HAM",
        "forename": "Lewis",
        "surname": "Hamilton",
        "dob": "1985-01-07",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Lewis_Hamilton"
    },
    {
        "driver_ref": "alonso",
        "number": 14,
        "code": "ALO",
        "forename": "Fernando",
        "surname": "Alonso",
        "dob": "1981-07-29",
        "nationality": "Spanish",
        "url": "http://en.wikipedia.org/wiki/Fernando_Alonso"
    },
    {
        "driver_ref": "vettel",
        "number": 5,
        "code": "VET",
        "forename": "Sebastian",
        "surname": "Vettel",
        "dob": "1987-07-03",
        "nationality": "German",
        "url": "http://en.wikipedia.org/wiki/Sebastian_Vettel"
    },
    {
        "driver_ref": "kubica",
        "number": 88,
        "code": "KUB",
        "forename": "Robert",
        "surname": "Kubica",
        "dob": "1984-12-07",
        "nationality": "Polish",
        "url": "http://en.wikipedia.org/wiki/Robert_Kubica"
    },
    {
        "driver_ref": "bourdais",
        "code": "BOU",
        "forename": "Sébastien",
        "surname": "Bourdais",
        "dob": "1979-02-28",
        "nationality": "French",
        "url": "http://en.wikipedia.org/wiki/S%C3%A9bastien_Bourdais"
    },
    {
        "driver_ref": "webber",
        "code": "WEB",
        "forename": "Mark",
        "surname": "Webber",
        "dob": "1976-08-27",
        "nationality": "Australian",
        "url": "http://en.wikipedia.org/wiki/Mark_Webber_(racing_driver)"
    },
    {
        "driver_ref": "glock",
        "code": "GLO",
        "forename": "Timo",
        "surname": "Glock",
        "dob": "1982-03-18",
        "nationality": "German",
        "url": "http://en.wikipedia.org/wiki/Timo_Glock"
    },
    {
        "driver_ref": "kovalainen",
        "code": "KOV",
        "forename": "Heikki",
        "surname": "Kovalainen",
        "dob": "1981-10-19",
        "nationality": "Finnish",
        "url": "http://en.wikipedia.org/wiki/Heikki_Kovalainen"
    },
    {
        "driver_ref": "coulthard",
        "code": "COU",
        "forename": "David",
        "surname": "Coulthard",
        "dob": "1971-03-27",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/David_Coulthard"
    },
    {
        "driver_ref": "rosberg",
        "number": 6,
        "code": "ROS",
        "forename": "Nico",
        "surname": "Rosberg",
        "dob": "1985-06-27",
        "nationality": "German",
        "url": "http://en.wikipedia.org/wiki/Nico_Rosberg"
    },
    {
        "driver_ref": "sutil",
        "number": 99,
        "code": "SUT",
        "forename": "Adrian",
        "surname": "Sutil",
        "dob": "1983-01-11",
        "nationality": "German",
        "url": "http://en.wikipedia.org/wiki/Adrian_Sutil"
    },
    {
        "driver_ref": "nakajima",
        "code": "NAK",
        "forename": "Kazuki",
        "surname": "Nakajima",
        "dob": "1985-01-11",
        "nationality": "Japanese",
        "url": "http://en.wikipedia.org/wiki/Kazuki_Nakajima"
    },
    {
        "driver_ref": "button",
        "number": 22,
        "code": "BUT",
        "forename": "Jenson",
        "surname": "Button",
        "dob": "1980-01-19",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Jenson_Button"
    },
    {
        "driver_ref": "trulli",
        "code": "TRU",
        "forename": "Jarno",
        "surname": "Trulli",
        "dob": "1974-07-13",
        "nationality": "Italian",
        "url": "http://en.wikipedia.org/wiki/Jarno_Trulli"
    },
    {
        "driver_ref": "fisichella",
        "code": "FIS",
        "forename": "Giancarlo",
        "surname": "Fisichella",
        "dob": "1973-01-14",
        "nationality": "Italian",
        "url": "http://en.wikipedia.org/wiki/Giancarlo_Fisichella"
    },
    {
        "driver_ref": "raikkonen",
        "number": 7,
        "code": "RAI",
        "forename": "Kimi",
        "surname": "Räikkönen",
        "dob": "1979-10-17",
        "nationality": "Finnish",
        "url": "http://en.wikipedia.org/wiki/Kimi_R%C3%A4ikk%C3%B6nen"
    },
    {
        "driver_ref": "barrichello",
        "code": "BAR",
        "forename": "Rubens",
        "surname": "Barrichello",
        "dob": "1972-05-23",
        "nationality": "Brazilian",
        "url": "http://en.wikipedia.org/wiki/Rubens_Barrichello"
    },
    {
        "driver_ref": "piquet_jr",
        "code": "PIQ",
        "forename": "Nelson",
        "surname": "Piquet Jr.",
        "dob": "1985-07-25",
        "nationality": "Brazilian",
        "url": "http://en.wikipedia.org/wiki/Nelson_Piquet,_Jr."
    },
    {
        "driver_ref": "ralf_schumacher",
        "code": "SCH",
        "forename": "Ralf",
        "surname": "Schumacher",
        "dob": "1975-06-30",
        "nationality": "German",
        "url": "http://en.wikipedia.org/wiki/Ralf_Schumacher"
    },
    {
        "driver_ref": "liuzzi",
        "code": "LIU",
        "forename": "Vitantonio",
        "surname": "Liuzzi",
        "dob": "1980-08-06",
        "nationality": "Italian",
        "url": "http://en.wikipedia.org/wiki/Vitantonio_Liuzzi"
    },
    {
        "driver_ref": "sato",
        "code": "SAT",
        "forename": "Takuma",
        "surname": "Sato",
        "dob": "1977-01-28",
        "nationality": "Japanese",
        "url": "http://en.wikipedia.org/wiki/Takuma_Sato"
    },
    {
        "driver_ref": "davidson",
        "code": "DAV",
        "forename": "Anthony",
        "surname": "Davidson",
        "dob": "1979-04-18",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Anthony_Davidson"
    },
    {
        "driver_ref": "yamamoto",
        "code": "YAM",
        "forename": "Sakon",
        "surname": "Yamamoto",
        "dob": "1982-07-09",
        "nationality": "Japanese",
        "url": "http://en.wikipedia.org/wiki/Sakon_Yamamoto"
    },
    {
        "driver_ref": "wurz",
        "code": "WUR",
        "forename": "Alexander",
        "surname": "Wurz",
        "dob": "1974-02-15",
        "nationality": "Austrian",
        "url": "http://en.wikipedia.org/wiki/Alexander_Wurz"
    },
    {
        "driver_ref": "villeneuve",
        "code": "VIL",
        "forename": "Jacques",
        "surname": "Villeneuve",
        "dob": "1971-04-09",
        "nationality": "Canadian",
        "url": "http://en.wikipedia.org/wiki/Jacques_Villeneuve"
    },
    {
        "driver_ref": "monteiro",
        "code": "TMO",
        "forename": "Tiago",
        "surname": "Monteiro",
        "dob": "1976-07-24",
        "nationality": "Portuguese",
        "url": "http://en.wikipedia.org/wiki/Tiago_Monteiro"
    },
    {
        "driver_ref": "klien",
        "code": "KLI",
        "forename": "Christian",
        "surname": "Klien",
        "dob": "1983-02-07",
        "nationality": "Austrian",
        "url": "http://en.wikipedia.org/wiki/Christian_Klien"
    },
    {
        "driver_ref": "karthikeyan",
        "code": "KAR",
        "forename": "Narain",
        "surname": "Karthikeyan",
        "dob": "1977-01-14",
        "nationality": "Indian",
        "url": "http://en.wikipedia.org/wiki/Narain_Karthikeyan"
    }
];

const SPA_WINNERS_DRIVERS: [StaticDriver; 28] = drivers_from_json![
    {
        "driver_ref": "massa",
        "number": 19,
        "code": "MAS",
        "forename": "Felipe",
        "surname": "Massa",
        "dob": "1981-04-25",
        "nationality": "Brazilian",
        "url": "http://en.wikipedia.org/wiki/Felipe_Massa"
    },
    {
        "driver_ref": "raikkonen",
        "number": 7,
        "code": "RAI",
        "forename": "Kimi",
        "surname": "Räikkönen",
        "dob": "1979-10-17",
        "nationality": "Finnish",
        "url": "http://en.wikipedia.org/wiki/Kimi_R%C3%A4ikk%C3%B6nen"
    },
    {
        "driver_ref": "michael_schumacher",
        "code": "MSC",
        "forename": "Michael",
        "surname": "Schumacher",
        "dob": "1969-01-03",
        "nationality": "German",
        "url": "http://en.wikipedia.org/wiki/Michael_Schumacher"
    },
    {
        "driver_ref": "hakkinen",
        "forename": "Mika",
        "surname": "Häkkinen",
        "dob": "1968-09-28",
        "nationality": "Finnish",
        "url": "http://en.wikipedia.org/wiki/Mika_H%C3%A4kkinen"
    },
    {
        "driver_ref": "coulthard",
        "code": "COU",
        "forename": "David",
        "surname": "Coulthard",
        "dob": "1971-03-27",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/David_Coulthard"
    },
    {
        "driver_ref": "damon_hill",
        "forename": "Damon",
        "surname": "Hill",
        "dob": "1960-09-17",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Damon_Hill"
    },
    {
        "driver_ref": "senna",
        "forename": "Ayrton",
        "surname": "Senna",
        "dob": "1960-03-21",
        "nationality": "Brazilian",
        "url": "http://en.wikipedia.org/wiki/Ayrton_Senna"
    },
    {
        "driver_ref": "prost",
        "forename": "Alain",
        "surname": "Prost",
        "dob": "1955-02-24",
        "nationality": "French",
        "url": "http://en.wikipedia.org/wiki/Alain_Prost"
    },
    {
        "driver_ref": "mansell",
        "forename": "Nigel",
        "surname": "Mansell",
        "dob": "1953-08-08",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Nigel_Mansell"
    },
    {
        "driver_ref": "rodriguez",
        "forename": "Pedro",
        "surname": "Rodríguez",
        "dob": "1940-01-18",
        "nationality": "Mexican",
        "url": "http://en.wikipedia.org/wiki/Pedro_Rodr%C3%ADguez_(racing_driver)"
    },
    {
        "driver_ref": "mclaren",
        "forename": "Bruce",
        "surname": "McLaren",
        "dob": "1937-08-30",
        "nationality": "New Zealander",
        "url": "http://en.wikipedia.org/wiki/Bruce_McLaren"
    },
    {
        "driver_ref": "gurney",
        "forename": "Dan",
        "surname": "Gurney",
        "dob": "1931-04-13",
        "nationality": "American",
        "url": "http://en.wikipedia.org/wiki/Dan_Gurney"
    },
    {
        "driver_ref": "surtees",
        "forename": "John",
        "surname": "Surtees",
        "dob": "1934-02-11",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/John_Surtees"
    },
    {
        "driver_ref": "clark",
        "forename": "Jim",
        "surname": "Clark",
        "dob": "1936-03-04",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Jim_Clark"
    },
    {
        "driver_ref": "phil_hill",
        "forename": "Phil",
        "surname": "Hill",
        "dob": "1927-04-20",
        "nationality": "American",
        "url": "http://en.wikipedia.org/wiki/Phil_Hill"
    },
    {
        "driver_ref": "jack_brabham",
        "forename": "Jack",
        "surname": "Brabham",
        "dob": "1926-04-02",
        "nationality": "Australian",
        "url": "http://en.wikipedia.org/wiki/Jack_Brabham"
    },
    {
        "driver_ref": "brooks",
        "forename": "Tony",
        "surname": "Brooks",
        "dob": "1932-02-25",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Tony_Brooks"
    },
    {
        "driver_ref": "collins",
        "forename": "Peter",
        "surname": "Collins",
        "dob": "1931-11-06",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Peter_Collins_(racing_driver)"
    },
    {
        "driver_ref": "fangio",
        "forename": "Juan",
        "surname": "Fangio",
        "dob": "1911-06-24",
        "nationality": "Argentine",
        "url": "http://en.wikipedia.org/wiki/Juan_Manuel_Fangio"
    },
    {
        "driver_ref": "ascari",
        "forename": "Alberto",
        "surname": "Ascari",
        "dob": "1918-07-13",
        "nationality": "Italian",
        "url": "http://en.wikipedia.org/wiki/Alberto_Ascari"
    },
    {
        "driver_ref": "farina",
        "forename": "Nino",
        "surname": "Farina",
        "dob": "1906-10-30",
        "nationality": "Italian",
        "url": "http://en.wikipedia.org/wiki/Nino_Farina"
    },
    {
        "driver_ref": "hamilton",
        "number": 44,
        "code": "HAM",
        "forename": "Lewis",
        "surname": "Hamilton",
        "dob": "1985-01-07",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Lewis_Hamilton"
    },
    {
        "driver_ref": "vettel",
        "number": 5,
        "code": "VET",
        "forename": "Sebastian",
        "surname": "Vettel",
        "dob": "1987-07-03",
        "nationality": "German",
        "url": "http://en.wikipedia.org/wiki/Sebastian_Vettel"
    },
    {
        "driver_ref": "button",
        "number": 22,
        "code": "BUT",
        "forename": "Jenson",
        "surname": "Button",
        "dob": "1980-01-19",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Jenson_Button"
    },
    {
        "driver_ref": "ricciardo",
        "number": 3,
        "code": "RIC",
        "forename": "Daniel",
        "surname": "Ricciardo",
        "dob": "1989-07-01",
        "nationality": "Australian",
        "url": "http://en.wikipedia.org/wiki/Daniel_Ricciardo"
    },
    {
        "driver_ref": "rosberg",
        "number": 6,
        "code": "ROS",
        "forename": "Nico",
        "surname": "Rosberg",
        "dob": "1985-06-27",
        "nationality": "German",
        "url": "http://en.wikipedia.org/wiki/Nico_Rosberg"
    },
    {
        "driver_ref": "leclerc",
        "number": 16,
        "code": "LEC",
        "forename": "Charles",
        "surname": "Leclerc",
        "dob": "1997-10-16",
        "nationality": "Monegasque",
        "url": "http://en.wikipedia.org/wiki/Charles_Leclerc"
    },
    {
        "driver_ref": "max_verstappen",
        "number": 33,
        "code": "VER",
        "forename": "Max",
        "surname": "Verstappen",
        "dob": "1997-09-30",
        "nationality": "Dutch",
        "url": "http://en.wikipedia.org/wiki/Max_Verstappen"
    }
];
