use shared::prelude::*;

pub mod common;

use common::models::StaticDriver;

#[tokio::test]
async fn test_get_driver() {
    common::Test::<StaticDriver, Driver>::new(
        "/api/f1/drivers?driver_ref=leclerc",
        Series::F1,
        LECLERC,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_drivers_by_circuit_ref() {
    common::Test::<&[StaticDriver], Vec<Driver>>::new(
        "/api/f1/drivers?circuit_ref=spa",
        Series::F1,
        &SPA_DRIVERS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 11,
        total: 326,
    }))
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_drivers_by_circuit_ref_and_result() {
    common::Test::<&[StaticDriver], Vec<Driver>>::new(
        "/api/f1/drivers?result=1&circuit_ref=spa",
        Series::F1,
        &SPA_WINNERS_DRIVERS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 1,
        total: 28,
    }))
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_drivers_by_driver_standing() {
    common::Test::<&[StaticDriver], Vec<Driver>>::new(
        "/api/f1/drivers?driver_standing=1",
        Series::F1,
        &CHAMPIONSHIP_WINNERS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 2,
        total: 34,
    }))
    .test_ok()
    .await
}

const LECLERC: StaticDriver = StaticDriver {
    driver_ref: "leclerc",
    number: Some(16),
    code: Some("LEC"),
    forename: "Charles",
    surname: "Leclerc",
    dob: Some("1997-10-16"),
    nationality: Some("Monegasque"),
    url: "http://en.wikipedia.org/wiki/Charles_Leclerc",
};

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

const CHAMPIONSHIP_WINNERS: [StaticDriver; 30] = drivers_from_json![
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
        "driver_ref": "villeneuve",
        "code": "VIL",
        "forename": "Jacques",
        "surname": "Villeneuve",
        "dob": "1971-04-09",
        "nationality": "Canadian",
        "url": "http://en.wikipedia.org/wiki/Jacques_Villeneuve"
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
        "driver_ref": "senna",
        "forename": "Ayrton",
        "surname": "Senna",
        "dob": "1960-03-21",
        "nationality": "Brazilian",
        "url": "http://en.wikipedia.org/wiki/Ayrton_Senna"
    },
    {
        "driver_ref": "piquet",
        "forename": "Nelson",
        "surname": "Piquet",
        "dob": "1952-08-17",
        "nationality": "Brazilian",
        "url": "http://en.wikipedia.org/wiki/Nelson_Piquet"
    },
    {
        "driver_ref": "lauda",
        "forename": "Niki",
        "surname": "Lauda",
        "dob": "1949-02-22",
        "nationality": "Austrian",
        "url": "http://en.wikipedia.org/wiki/Niki_Lauda"
    },
    {
        "driver_ref": "keke_rosberg",
        "forename": "Keke",
        "surname": "Rosberg",
        "dob": "1948-12-06",
        "nationality": "Finnish",
        "url": "http://en.wikipedia.org/wiki/Keke_Rosberg"
    },
    {
        "driver_ref": "jones",
        "forename": "Alan",
        "surname": "Jones",
        "dob": "1946-11-02",
        "nationality": "Australian",
        "url": "http://en.wikipedia.org/wiki/Alan_Jones_(Formula_1)"
    },
    {
        "driver_ref": "scheckter",
        "forename": "Jody",
        "surname": "Scheckter",
        "dob": "1950-01-29",
        "nationality": "South African",
        "url": "http://en.wikipedia.org/wiki/Jody_Scheckter"
    },
    {
        "driver_ref": "mario_andretti",
        "forename": "Mario",
        "surname": "Andretti",
        "dob": "1940-02-28",
        "nationality": "American",
        "url": "http://en.wikipedia.org/wiki/Mario_Andretti"
    },
    {
        "driver_ref": "hunt",
        "forename": "James",
        "surname": "Hunt",
        "dob": "1947-08-29",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/James_Hunt"
    },
    {
        "driver_ref": "emerson_fittipaldi",
        "forename": "Emerson",
        "surname": "Fittipaldi",
        "dob": "1946-12-12",
        "nationality": "Brazilian",
        "url": "http://en.wikipedia.org/wiki/Emerson_Fittipaldi"
    },
    {
        "driver_ref": "stewart",
        "forename": "Jackie",
        "surname": "Stewart",
        "dob": "1939-06-11",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Jackie_Stewart"
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
        "driver_ref": "rindt",
        "forename": "Jochen",
        "surname": "Rindt",
        "dob": "1942-04-18",
        "nationality": "Austrian",
        "url": "http://en.wikipedia.org/wiki/Jochen_Rindt"
    },
    {
        "driver_ref": "hill",
        "forename": "Graham",
        "surname": "Hill",
        "dob": "1929-02-15",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Graham_Hill"
    },
    {
        "driver_ref": "hulme",
        "forename": "Denny",
        "surname": "Hulme",
        "dob": "1936-06-18",
        "nationality": "New Zealander",
        "url": "http://en.wikipedia.org/wiki/Denny_Hulme"
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
        "driver_ref": "clark",
        "forename": "Jim",
        "surname": "Clark",
        "dob": "1936-03-04",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Jim_Clark"
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
        "driver_ref": "phil_hill",
        "forename": "Phil",
        "surname": "Hill",
        "dob": "1927-04-20",
        "nationality": "American",
        "url": "http://en.wikipedia.org/wiki/Phil_Hill"
    },
    {
        "driver_ref": "hawthorn",
        "forename": "Mike",
        "surname": "Hawthorn",
        "dob": "1929-04-10",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Mike_Hawthorn"
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
    }
];
