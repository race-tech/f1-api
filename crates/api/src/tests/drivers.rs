use serde_json::json;

use super::common::Test;

#[tokio::test]
async fn test_get_driver() {
    let value: serde_json::Value = json!({
        "driver": {
                    "driverRef": "leclerc",
                    "number": 16,
                    "code": "LEC",
                    "forename": "Charles",
                    "surname": "Leclerc",
                    "dob": "1997-10-16",
                    "nationality": "Monegasque",
                    "url": "http://en.wikipedia.org/wiki/Charles_Leclerc"
                }
    });

    Test::new(
        r#"{
            driver(driverRef: "leclerc") {
                driverRef
                number
                code
                forename
                surname
                dob
                nationality
                url
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_drivers_by_circuit_ref() {
    let value: serde_json::Value = json!({
        "data": [
                    {
                        "driverRef": "massa",
                        "number": 19,
                        "code": "MAS",
                        "forename": "Felipe",
                        "surname": "Massa",
                        "dob": "1981-04-25",
                        "nationality": "Brazilian",
                        "url": "http://en.wikipedia.org/wiki/Felipe_Massa"
                    },
                    {
                        "driverRef": "heidfeld",
                        "number": null,
                        "code": "HEI",
                        "forename": "Nick",
                        "surname": "Heidfeld",
                        "dob": "1977-05-10",
                        "nationality": "German",
                        "url": "http://en.wikipedia.org/wiki/Nick_Heidfeld"
                    },
                    {
                        "driverRef": "hamilton",
                        "number": 44,
                        "code": "HAM",
                        "forename": "Lewis",
                        "surname": "Hamilton",
                        "dob": "1985-01-07",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Lewis_Hamilton"
                    },
                    {
                        "driverRef": "alonso",
                        "number": 14,
                        "code": "ALO",
                        "forename": "Fernando",
                        "surname": "Alonso",
                        "dob": "1981-07-29",
                        "nationality": "Spanish",
                        "url": "http://en.wikipedia.org/wiki/Fernando_Alonso"
                    },
                    {
                        "driverRef": "vettel",
                        "number": 5,
                        "code": "VET",
                        "forename": "Sebastian",
                        "surname": "Vettel",
                        "dob": "1987-07-03",
                        "nationality": "German",
                        "url": "http://en.wikipedia.org/wiki/Sebastian_Vettel"
                    },
                    {
                        "driverRef": "kubica",
                        "number": 88,
                        "code": "KUB",
                        "forename": "Robert",
                        "surname": "Kubica",
                        "dob": "1984-12-07",
                        "nationality": "Polish",
                        "url": "http://en.wikipedia.org/wiki/Robert_Kubica"
                    },
                    {
                        "driverRef": "bourdais",
                        "number": null,
                        "code": "BOU",
                        "forename": "Sébastien",
                        "surname": "Bourdais",
                        "dob": "1979-02-28",
                        "nationality": "French",
                        "url": "http://en.wikipedia.org/wiki/S%C3%A9bastien_Bourdais"
                    },
                    {
                        "driverRef": "webber",
                        "number": null,
                        "code": "WEB",
                        "forename": "Mark",
                        "surname": "Webber",
                        "dob": "1976-08-27",
                        "nationality": "Australian",
                        "url": "http://en.wikipedia.org/wiki/Mark_Webber_(racing_driver)"
                    },
                    {
                        "driverRef": "glock",
                        "number": null,
                        "code": "GLO",
                        "forename": "Timo",
                        "surname": "Glock",
                        "dob": "1982-03-18",
                        "nationality": "German",
                        "url": "http://en.wikipedia.org/wiki/Timo_Glock"
                    },
                    {
                        "driverRef": "kovalainen",
                        "number": null,
                        "code": "KOV",
                        "forename": "Heikki",
                        "surname": "Kovalainen",
                        "dob": "1981-10-19",
                        "nationality": "Finnish",
                        "url": "http://en.wikipedia.org/wiki/Heikki_Kovalainen"
                    },
                    {
                        "driverRef": "coulthard",
                        "number": null,
                        "code": "COU",
                        "forename": "David",
                        "surname": "Coulthard",
                        "dob": "1971-03-27",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/David_Coulthard"
                    },
                    {
                        "driverRef": "rosberg",
                        "number": 6,
                        "code": "ROS",
                        "forename": "Nico",
                        "surname": "Rosberg",
                        "dob": "1985-06-27",
                        "nationality": "German",
                        "url": "http://en.wikipedia.org/wiki/Nico_Rosberg"
                    },
                    {
                        "driverRef": "sutil",
                        "number": 99,
                        "code": "SUT",
                        "forename": "Adrian",
                        "surname": "Sutil",
                        "dob": "1983-01-11",
                        "nationality": "German",
                        "url": "http://en.wikipedia.org/wiki/Adrian_Sutil"
                    },
                    {
                        "driverRef": "nakajima",
                        "number": null,
                        "code": "NAK",
                        "forename": "Kazuki",
                        "surname": "Nakajima",
                        "dob": "1985-01-11",
                        "nationality": "Japanese",
                        "url": "http://en.wikipedia.org/wiki/Kazuki_Nakajima"
                    },
                    {
                        "driverRef": "button",
                        "number": 22,
                        "code": "BUT",
                        "forename": "Jenson",
                        "surname": "Button",
                        "dob": "1980-01-19",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Jenson_Button"
                    },
                    {
                        "driverRef": "trulli",
                        "number": null,
                        "code": "TRU",
                        "forename": "Jarno",
                        "surname": "Trulli",
                        "dob": "1974-07-13",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Jarno_Trulli"
                    },
                    {
                        "driverRef": "fisichella",
                        "number": null,
                        "code": "FIS",
                        "forename": "Giancarlo",
                        "surname": "Fisichella",
                        "dob": "1973-01-14",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Giancarlo_Fisichella"
                    },
                    {
                        "driverRef": "raikkonen",
                        "number": 7,
                        "code": "RAI",
                        "forename": "Kimi",
                        "surname": "Räikkönen",
                        "dob": "1979-10-17",
                        "nationality": "Finnish",
                        "url": "http://en.wikipedia.org/wiki/Kimi_R%C3%A4ikk%C3%B6nen"
                    },
                    {
                        "driverRef": "barrichello",
                        "number": null,
                        "code": "BAR",
                        "forename": "Rubens",
                        "surname": "Barrichello",
                        "dob": "1972-05-23",
                        "nationality": "Brazilian",
                        "url": "http://en.wikipedia.org/wiki/Rubens_Barrichello"
                    },
                    {
                        "driverRef": "piquet_jr",
                        "number": null,
                        "code": "PIQ",
                        "forename": "Nelson",
                        "surname": "Piquet Jr.",
                        "dob": "1985-07-25",
                        "nationality": "Brazilian",
                        "url": "http://en.wikipedia.org/wiki/Nelson_Piquet,_Jr."
                    },
                    {
                        "driverRef": "ralf_schumacher",
                        "number": null,
                        "code": "SCH",
                        "forename": "Ralf",
                        "surname": "Schumacher",
                        "dob": "1975-06-30",
                        "nationality": "German",
                        "url": "http://en.wikipedia.org/wiki/Ralf_Schumacher"
                    },
                    {
                        "driverRef": "liuzzi",
                        "number": null,
                        "code": "LIU",
                        "forename": "Vitantonio",
                        "surname": "Liuzzi",
                        "dob": "1980-08-06",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Vitantonio_Liuzzi"
                    },
                    {
                        "driverRef": "sato",
                        "number": null,
                        "code": "SAT",
                        "forename": "Takuma",
                        "surname": "Sato",
                        "dob": "1977-01-28",
                        "nationality": "Japanese",
                        "url": "http://en.wikipedia.org/wiki/Takuma_Sato"
                    },
                    {
                        "driverRef": "davidson",
                        "number": null,
                        "code": "DAV",
                        "forename": "Anthony",
                        "surname": "Davidson",
                        "dob": "1979-04-18",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Anthony_Davidson"
                    },
                    {
                        "driverRef": "yamamoto",
                        "number": null,
                        "code": "YAM",
                        "forename": "Sakon",
                        "surname": "Yamamoto",
                        "dob": "1982-07-09",
                        "nationality": "Japanese",
                        "url": "http://en.wikipedia.org/wiki/Sakon_Yamamoto"
                    },
                    {
                        "driverRef": "wurz",
                        "number": null,
                        "code": "WUR",
                        "forename": "Alexander",
                        "surname": "Wurz",
                        "dob": "1974-02-15",
                        "nationality": "Austrian",
                        "url": "http://en.wikipedia.org/wiki/Alexander_Wurz"
                    },
                    {
                        "driverRef": "villeneuve",
                        "number": null,
                        "code": "VIL",
                        "forename": "Jacques",
                        "surname": "Villeneuve",
                        "dob": "1971-04-09",
                        "nationality": "Canadian",
                        "url": "http://en.wikipedia.org/wiki/Jacques_Villeneuve"
                    },
                    {
                        "driverRef": "monteiro",
                        "number": null,
                        "code": "TMO",
                        "forename": "Tiago",
                        "surname": "Monteiro",
                        "dob": "1976-07-24",
                        "nationality": "Portuguese",
                        "url": "http://en.wikipedia.org/wiki/Tiago_Monteiro"
                    },
                    {
                        "driverRef": "klien",
                        "number": null,
                        "code": "KLI",
                        "forename": "Christian",
                        "surname": "Klien",
                        "dob": "1983-02-07",
                        "nationality": "Austrian",
                        "url": "http://en.wikipedia.org/wiki/Christian_Klien"
                    },
                    {
                        "driverRef": "karthikeyan",
                        "number": null,
                        "code": "KAR",
                        "forename": "Narain",
                        "surname": "Karthikeyan",
                        "dob": "1977-01-14",
                        "nationality": "Indian",
                        "url": "http://en.wikipedia.org/wiki/Narain_Karthikeyan"
                    }
                ]
    });

    Test::new(
        r#"{
            drivers(options: { circuitRef: "spa" }, pagination: { limit: 30, page: 1 }) {
                data {
                    driverRef
                    number
                    code
                    forename
                    surname
                    dob
                    nationality
                    url
                }
            }
        }"#,
        value,
    )
    .specify_field("drivers")
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_drivers_by_circuit_ref_and_result() {
    let value: serde_json::Value = json!({
        "data": [
                    {
                        "driverRef": "massa",
                        "number": 19,
                        "code": "MAS",
                        "forename": "Felipe",
                        "surname": "Massa",
                        "dob": "1981-04-25",
                        "nationality": "Brazilian",
                        "url": "http://en.wikipedia.org/wiki/Felipe_Massa"
                    },
                    {
                        "driverRef": "raikkonen",
                        "number": 7,
                        "code": "RAI",
                        "forename": "Kimi",
                        "surname": "Räikkönen",
                        "dob": "1979-10-17",
                        "nationality": "Finnish",
                        "url": "http://en.wikipedia.org/wiki/Kimi_R%C3%A4ikk%C3%B6nen"
                    },
                    {
                        "driverRef": "michael_schumacher",
                        "number": null,
                        "code": "MSC",
                        "forename": "Michael",
                        "surname": "Schumacher",
                        "dob": "1969-01-03",
                        "nationality": "German",
                        "url": "http://en.wikipedia.org/wiki/Michael_Schumacher"
                    },
                    {
                        "driverRef": "hakkinen",
                        "number": null,
                        "code": null,
                        "forename": "Mika",
                        "surname": "Häkkinen",
                        "dob": "1968-09-28",
                        "nationality": "Finnish",
                        "url": "http://en.wikipedia.org/wiki/Mika_H%C3%A4kkinen"
                    },
                    {
                        "driverRef": "coulthard",
                        "number": null,
                        "code": "COU",
                        "forename": "David",
                        "surname": "Coulthard",
                        "dob": "1971-03-27",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/David_Coulthard"
                    },
                    {
                        "driverRef": "damon_hill",
                        "number": null,
                        "code": null,
                        "forename": "Damon",
                        "surname": "Hill",
                        "dob": "1960-09-17",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Damon_Hill"
                    },
                    {
                        "driverRef": "senna",
                        "number": null,
                        "code": null,
                        "forename": "Ayrton",
                        "surname": "Senna",
                        "dob": "1960-03-21",
                        "nationality": "Brazilian",
                        "url": "http://en.wikipedia.org/wiki/Ayrton_Senna"
                    },
                    {
                        "driverRef": "prost",
                        "number": null,
                        "code": null,
                        "forename": "Alain",
                        "surname": "Prost",
                        "dob": "1955-02-24",
                        "nationality": "French",
                        "url": "http://en.wikipedia.org/wiki/Alain_Prost"
                    },
                    {
                        "driverRef": "mansell",
                        "number": null,
                        "code": null,
                        "forename": "Nigel",
                        "surname": "Mansell",
                        "dob": "1953-08-08",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Nigel_Mansell"
                    },
                    {
                        "driverRef": "rodriguez",
                        "number": null,
                        "code": null,
                        "forename": "Pedro",
                        "surname": "Rodríguez",
                        "dob": "1940-01-18",
                        "nationality": "Mexican",
                        "url": "http://en.wikipedia.org/wiki/Pedro_Rodr%C3%ADguez_(racing_driver)"
                    },
                    {
                        "driverRef": "mclaren",
                        "number": null,
                        "code": null,
                        "forename": "Bruce",
                        "surname": "McLaren",
                        "dob": "1937-08-30",
                        "nationality": "New Zealander",
                        "url": "http://en.wikipedia.org/wiki/Bruce_McLaren"
                    },
                    {
                        "driverRef": "gurney",
                        "number": null,
                        "code": null,
                        "forename": "Dan",
                        "surname": "Gurney",
                        "dob": "1931-04-13",
                        "nationality": "American",
                        "url": "http://en.wikipedia.org/wiki/Dan_Gurney"
                    },
                    {
                        "driverRef": "surtees",
                        "number": null,
                        "code": null,
                        "forename": "John",
                        "surname": "Surtees",
                        "dob": "1934-02-11",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/John_Surtees"
                    },
                    {
                        "driverRef": "clark",
                        "number": null,
                        "code": null,
                        "forename": "Jim",
                        "surname": "Clark",
                        "dob": "1936-03-04",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Jim_Clark"
                    },
                    {
                        "driverRef": "phil_hill",
                        "number": null,
                        "code": null,
                        "forename": "Phil",
                        "surname": "Hill",
                        "dob": "1927-04-20",
                        "nationality": "American",
                        "url": "http://en.wikipedia.org/wiki/Phil_Hill"
                    },
                    {
                        "driverRef": "jack_brabham",
                        "number": null,
                        "code": null,
                        "forename": "Jack",
                        "surname": "Brabham",
                        "dob": "1926-04-02",
                        "nationality": "Australian",
                        "url": "http://en.wikipedia.org/wiki/Jack_Brabham"
                    },
                    {
                        "driverRef": "brooks",
                        "number": null,
                        "code": null,
                        "forename": "Tony",
                        "surname": "Brooks",
                        "dob": "1932-02-25",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Tony_Brooks"
                    },
                    {
                        "driverRef": "collins",
                        "number": null,
                        "code": null,
                        "forename": "Peter",
                        "surname": "Collins",
                        "dob": "1931-11-06",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Peter_Collins_(racing_driver)"
                    },
                    {
                        "driverRef": "fangio",
                        "number": null,
                        "code": null,
                        "forename": "Juan",
                        "surname": "Fangio",
                        "dob": "1911-06-24",
                        "nationality": "Argentine",
                        "url": "http://en.wikipedia.org/wiki/Juan_Manuel_Fangio"
                    },
                    {
                        "driverRef": "ascari",
                        "number": null,
                        "code": null,
                        "forename": "Alberto",
                        "surname": "Ascari",
                        "dob": "1918-07-13",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Alberto_Ascari"
                    },
                    {
                        "driverRef": "farina",
                        "number": null,
                        "code": null,
                        "forename": "Nino",
                        "surname": "Farina",
                        "dob": "1906-10-30",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Nino_Farina"
                    },
                    {
                        "driverRef": "hamilton",
                        "number": 44,
                        "code": "HAM",
                        "forename": "Lewis",
                        "surname": "Hamilton",
                        "dob": "1985-01-07",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Lewis_Hamilton"
                    },
                    {
                        "driverRef": "vettel",
                        "number": 5,
                        "code": "VET",
                        "forename": "Sebastian",
                        "surname": "Vettel",
                        "dob": "1987-07-03",
                        "nationality": "German",
                        "url": "http://en.wikipedia.org/wiki/Sebastian_Vettel"
                    },
                    {
                        "driverRef": "button",
                        "number": 22,
                        "code": "BUT",
                        "forename": "Jenson",
                        "surname": "Button",
                        "dob": "1980-01-19",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Jenson_Button"
                    },
                    {
                        "driverRef": "ricciardo",
                        "number": 3,
                        "code": "RIC",
                        "forename": "Daniel",
                        "surname": "Ricciardo",
                        "dob": "1989-07-01",
                        "nationality": "Australian",
                        "url": "http://en.wikipedia.org/wiki/Daniel_Ricciardo"
                    },
                    {
                        "driverRef": "rosberg",
                        "number": 6,
                        "code": "ROS",
                        "forename": "Nico",
                        "surname": "Rosberg",
                        "dob": "1985-06-27",
                        "nationality": "German",
                        "url": "http://en.wikipedia.org/wiki/Nico_Rosberg"
                    },
                    {
                        "driverRef": "leclerc",
                        "number": 16,
                        "code": "LEC",
                        "forename": "Charles",
                        "surname": "Leclerc",
                        "dob": "1997-10-16",
                        "nationality": "Monegasque",
                        "url": "http://en.wikipedia.org/wiki/Charles_Leclerc"
                    },
                    {
                        "driverRef": "max_verstappen",
                        "number": 33,
                        "code": "VER",
                        "forename": "Max",
                        "surname": "Verstappen",
                        "dob": "1997-09-30",
                        "nationality": "Dutch",
                        "url": "http://en.wikipedia.org/wiki/Max_Verstappen"
                    }
                ]
    });

    Test::new(
        r#"{
            drivers(
                options: { circuitRef: "spa", result: 1 }
                pagination: { limit: 30, page: 1 }
            ) {
                data {
                    driverRef
                    number
                    code
                    forename
                    surname
                    dob
                    nationality
                    url
                }
            }
        }"#,
        value,
    )
    .specify_field("drivers")
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_drivers_by_driver_standing() {
    let value: serde_json::Value = json!({
        "data": [
                    {
                        "driverRef": "hamilton",
                        "number": 44,
                        "code": "HAM",
                        "forename": "Lewis",
                        "surname": "Hamilton",
                        "dob": "1985-01-07",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Lewis_Hamilton"
                    },
                    {
                        "driverRef": "raikkonen",
                        "number": 7,
                        "code": "RAI",
                        "forename": "Kimi",
                        "surname": "Räikkönen",
                        "dob": "1979-10-17",
                        "nationality": "Finnish",
                        "url": "http://en.wikipedia.org/wiki/Kimi_R%C3%A4ikk%C3%B6nen"
                    },
                    {
                        "driverRef": "alonso",
                        "number": 14,
                        "code": "ALO",
                        "forename": "Fernando",
                        "surname": "Alonso",
                        "dob": "1981-07-29",
                        "nationality": "Spanish",
                        "url": "http://en.wikipedia.org/wiki/Fernando_Alonso"
                    },
                    {
                        "driverRef": "michael_schumacher",
                        "number": null,
                        "code": "MSC",
                        "forename": "Michael",
                        "surname": "Schumacher",
                        "dob": "1969-01-03",
                        "nationality": "German",
                        "url": "http://en.wikipedia.org/wiki/Michael_Schumacher"
                    },
                    {
                        "driverRef": "hakkinen",
                        "number": null,
                        "code": null,
                        "forename": "Mika",
                        "surname": "Häkkinen",
                        "dob": "1968-09-28",
                        "nationality": "Finnish",
                        "url": "http://en.wikipedia.org/wiki/Mika_H%C3%A4kkinen"
                    },
                    {
                        "driverRef": "villeneuve",
                        "number": null,
                        "code": "VIL",
                        "forename": "Jacques",
                        "surname": "Villeneuve",
                        "dob": "1971-04-09",
                        "nationality": "Canadian",
                        "url": "http://en.wikipedia.org/wiki/Jacques_Villeneuve"
                    },
                    {
                        "driverRef": "damon_hill",
                        "number": null,
                        "code": null,
                        "forename": "Damon",
                        "surname": "Hill",
                        "dob": "1960-09-17",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Damon_Hill"
                    },
                    {
                        "driverRef": "prost",
                        "number": null,
                        "code": null,
                        "forename": "Alain",
                        "surname": "Prost",
                        "dob": "1955-02-24",
                        "nationality": "French",
                        "url": "http://en.wikipedia.org/wiki/Alain_Prost"
                    },
                    {
                        "driverRef": "mansell",
                        "number": null,
                        "code": null,
                        "forename": "Nigel",
                        "surname": "Mansell",
                        "dob": "1953-08-08",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Nigel_Mansell"
                    },
                    {
                        "driverRef": "senna",
                        "number": null,
                        "code": null,
                        "forename": "Ayrton",
                        "surname": "Senna",
                        "dob": "1960-03-21",
                        "nationality": "Brazilian",
                        "url": "http://en.wikipedia.org/wiki/Ayrton_Senna"
                    },
                    {
                        "driverRef": "piquet",
                        "number": null,
                        "code": null,
                        "forename": "Nelson",
                        "surname": "Piquet",
                        "dob": "1952-08-17",
                        "nationality": "Brazilian",
                        "url": "http://en.wikipedia.org/wiki/Nelson_Piquet"
                    },
                    {
                        "driverRef": "lauda",
                        "number": null,
                        "code": null,
                        "forename": "Niki",
                        "surname": "Lauda",
                        "dob": "1949-02-22",
                        "nationality": "Austrian",
                        "url": "http://en.wikipedia.org/wiki/Niki_Lauda"
                    },
                    {
                        "driverRef": "keke_rosberg",
                        "number": null,
                        "code": null,
                        "forename": "Keke",
                        "surname": "Rosberg",
                        "dob": "1948-12-06",
                        "nationality": "Finnish",
                        "url": "http://en.wikipedia.org/wiki/Keke_Rosberg"
                    },
                    {
                        "driverRef": "jones",
                        "number": null,
                        "code": null,
                        "forename": "Alan",
                        "surname": "Jones",
                        "dob": "1946-11-02",
                        "nationality": "Australian",
                        "url": "http://en.wikipedia.org/wiki/Alan_Jones_(Formula_1)"
                    },
                    {
                        "driverRef": "scheckter",
                        "number": null,
                        "code": null,
                        "forename": "Jody",
                        "surname": "Scheckter",
                        "dob": "1950-01-29",
                        "nationality": "South African",
                        "url": "http://en.wikipedia.org/wiki/Jody_Scheckter"
                    },
                    {
                        "driverRef": "mario_andretti",
                        "number": null,
                        "code": null,
                        "forename": "Mario",
                        "surname": "Andretti",
                        "dob": "1940-02-28",
                        "nationality": "American",
                        "url": "http://en.wikipedia.org/wiki/Mario_Andretti"
                    },
                    {
                        "driverRef": "hunt",
                        "number": null,
                        "code": null,
                        "forename": "James",
                        "surname": "Hunt",
                        "dob": "1947-08-29",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/James_Hunt"
                    },
                    {
                        "driverRef": "emerson_fittipaldi",
                        "number": null,
                        "code": null,
                        "forename": "Emerson",
                        "surname": "Fittipaldi",
                        "dob": "1946-12-12",
                        "nationality": "Brazilian",
                        "url": "http://en.wikipedia.org/wiki/Emerson_Fittipaldi"
                    },
                    {
                        "driverRef": "stewart",
                        "number": null,
                        "code": null,
                        "forename": "Jackie",
                        "surname": "Stewart",
                        "dob": "1939-06-11",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Jackie_Stewart"
                    },
                    {
                        "driverRef": "button",
                        "number": 22,
                        "code": "BUT",
                        "forename": "Jenson",
                        "surname": "Button",
                        "dob": "1980-01-19",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Jenson_Button"
                    },
                    {
                        "driverRef": "rindt",
                        "number": null,
                        "code": null,
                        "forename": "Jochen",
                        "surname": "Rindt",
                        "dob": "1942-04-18",
                        "nationality": "Austrian",
                        "url": "http://en.wikipedia.org/wiki/Jochen_Rindt"
                    },
                    {
                        "driverRef": "hill",
                        "number": null,
                        "code": null,
                        "forename": "Graham",
                        "surname": "Hill",
                        "dob": "1929-02-15",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Graham_Hill"
                    },
                    {
                        "driverRef": "hulme",
                        "number": null,
                        "code": null,
                        "forename": "Denny",
                        "surname": "Hulme",
                        "dob": "1936-06-18",
                        "nationality": "New Zealander",
                        "url": "http://en.wikipedia.org/wiki/Denny_Hulme"
                    },
                    {
                        "driverRef": "jack_brabham",
                        "number": null,
                        "code": null,
                        "forename": "Jack",
                        "surname": "Brabham",
                        "dob": "1926-04-02",
                        "nationality": "Australian",
                        "url": "http://en.wikipedia.org/wiki/Jack_Brabham"
                    },
                    {
                        "driverRef": "clark",
                        "number": null,
                        "code": null,
                        "forename": "Jim",
                        "surname": "Clark",
                        "dob": "1936-03-04",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Jim_Clark"
                    },
                    {
                        "driverRef": "surtees",
                        "number": null,
                        "code": null,
                        "forename": "John",
                        "surname": "Surtees",
                        "dob": "1934-02-11",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/John_Surtees"
                    },
                    {
                        "driverRef": "phil_hill",
                        "number": null,
                        "code": null,
                        "forename": "Phil",
                        "surname": "Hill",
                        "dob": "1927-04-20",
                        "nationality": "American",
                        "url": "http://en.wikipedia.org/wiki/Phil_Hill"
                    },
                    {
                        "driverRef": "hawthorn",
                        "number": null,
                        "code": null,
                        "forename": "Mike",
                        "surname": "Hawthorn",
                        "dob": "1929-04-10",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Mike_Hawthorn"
                    },
                    {
                        "driverRef": "fangio",
                        "number": null,
                        "code": null,
                        "forename": "Juan",
                        "surname": "Fangio",
                        "dob": "1911-06-24",
                        "nationality": "Argentine",
                        "url": "http://en.wikipedia.org/wiki/Juan_Manuel_Fangio"
                    },
                    {
                        "driverRef": "ascari",
                        "number": null,
                        "code": null,
                        "forename": "Alberto",
                        "surname": "Ascari",
                        "dob": "1918-07-13",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Alberto_Ascari"
                    }
                ]
    });

    Test::new(
        r#"{
            drivers(options: { driverStanding: 1 }, pagination: { limit: 30, page: 1 }) {
                data {
                    driverRef
                    number
                    code
                    forename
                    surname
                    dob
                    nationality
                    url
                }
            }
        }"#,
        value,
    )
    .specify_field("drivers")
    .test_ok()
    .await
}
