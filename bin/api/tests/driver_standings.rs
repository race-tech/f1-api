use shared::prelude::*;

pub mod common;

use common::models::StaticStanding;

#[tokio::test]
async fn test_get_driver_standings() {
    common::Test::<'_, &[StaticStanding], Vec<InnerStandingResponse>>::new(
        "/api/f1/drivers/standing/",
        Series::F1,
        &ALL_STANDINGS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 106,
        total: 3168,
    }))
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_driver_standings_by_year() {
    common::Test::<'_, &[StaticStanding<'_>], Vec<InnerStandingResponse>>::new(
        "/api/f1/drivers/standing/?year=2023",
        Series::F1,
        &SEASON_2023_STANDINGS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 1,
        total: 22,
    }))
    .test_ok()
    .await
}

const ALL_STANDINGS: [StaticStanding; 1] = driver_standings_from_json![
    {
        "season": 1950,
        "round": 7,
        "driver_standings": [
            {
                "points": 0.0,
                "position": 74,
                "position_text": "74",
                "wins": 0,
                "driver": {
                    "driver_ref": "cantrell",
                    "forename": "Bill",
                    "surname": "Cantrell",
                    "dob": "1908-01-31",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/William_Cantrell"
                }
            },
            {
                "points": 0.0,
                "position": 67,
                "position_text": "67",
                "wins": 0,
                "driver": {
                    "driver_ref": "rolt",
                    "forename": "Tony",
                    "surname": "Rolt",
                    "dob": "1918-10-16",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Tony_Rolt"
                }
            },
            {
                "points": 0.0,
                "position": 40,
                "position_text": "40",
                "wins": 0,
                "driver": {
                    "driver_ref": "fry",
                    "forename": "Joe",
                    "surname": "Fry",
                    "dob": "1915-10-26",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Joe_Fry"
                }
            },
            {
                "points": 0.0,
                "position": 72,
                "position_text": "72",
                "wins": 0,
                "driver": {
                    "driver_ref": "pietsch",
                    "forename": "Paul",
                    "surname": "Pietsch",
                    "dob": "1911-06-20",
                    "nationality": "German",
                    "url": "http://en.wikipedia.org/wiki/Paul_Pietsch"
                }
            },
            {
                "points": 0.0,
                "position": 71,
                "position_text": "71",
                "wins": 0,
                "driver": {
                    "driver_ref": "sanesi",
                    "forename": "Consalvo",
                    "surname": "Sanesi",
                    "dob": "1911-03-28",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Consalvo_Sanesi"
                }
            },
            {
                "points": 0.0,
                "position": 66,
                "position_text": "66",
                "wins": 0,
                "driver": {
                    "driver_ref": "comotti",
                    "forename": "Franco",
                    "surname": "Comotti",
                    "dob": "1906-07-24",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Franco_Comotti"
                }
            },
            {
                "points": 0.0,
                "position": 65,
                "position_text": "65",
                "wins": 0,
                "driver": {
                    "driver_ref": "louveau",
                    "forename": "Henri",
                    "surname": "Louveau",
                    "dob": "1910-01-25",
                    "nationality": "French",
                    "url": "http://en.wikipedia.org/wiki/Henri_Louveau"
                }
            },
            {
                "points": 0.0,
                "position": 64,
                "position_text": "64",
                "wins": 0,
                "driver": {
                    "driver_ref": "biondetti",
                    "forename": "Clemente",
                    "surname": "Biondetti",
                    "dob": "1898-08-18",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Clemente_Biondetti"
                }
            },
            {
                "points": 0.0,
                "position": 61,
                "position_text": "61",
                "wins": 0,
                "driver": {
                    "driver_ref": "taruffi",
                    "forename": "Piero",
                    "surname": "Taruffi",
                    "dob": "1906-10-12",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Piero_Taruffi"
                }
            },
            {
                "points": 0.0,
                "position": 58,
                "position_text": "58",
                "wins": 0,
                "driver": {
                    "driver_ref": "guy_mairesse",
                    "forename": "Guy",
                    "surname": "Mairesse",
                    "dob": "1910-08-10",
                    "nationality": "French",
                    "url": "http://en.wikipedia.org/wiki/Guy_Mairesse"
                }
            },
            {
                "points": 0.0,
                "position": 26,
                "position_text": "26",
                "wins": 0,
                "driver": {
                    "driver_ref": "pozzi",
                    "forename": "Charles",
                    "surname": "Pozzi",
                    "dob": "1909-08-27",
                    "nationality": "French",
                    "url": "http://en.wikipedia.org/wiki/Charles_Pozzi"
                }
            },
            {
                "points": 0.0,
                "position": 29,
                "position_text": "29",
                "wins": 0,
                "driver": {
                    "driver_ref": "levegh",
                    "forename": "Pierre",
                    "surname": "Levegh",
                    "dob": "1905-12-22",
                    "nationality": "French",
                    "url": "http://en.wikipedia.org/wiki/Pierre_Levegh"
                }
            },
            {
                "points": 0.0,
                "position": 38,
                "position_text": "38",
                "wins": 0,
                "driver": {
                    "driver_ref": "branca",
                    "forename": "Toni",
                    "surname": "Branca",
                    "dob": "1916-09-15",
                    "nationality": "Swiss",
                    "url": "http://en.wikipedia.org/wiki/Toni_Branca"
                }
            },
            {
                "points": 0.0,
                "position": 32,
                "position_text": "32",
                "wins": 0,
                "driver": {
                    "driver_ref": "pagani",
                    "forename": "Nello",
                    "surname": "Pagani",
                    "dob": "1911-10-11",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Nello_Pagani"
                }
            },
            {
                "points": 0.0,
                "position": 80,
                "position_text": "80",
                "wins": 0,
                "driver": {
                    "driver_ref": "dinsmore",
                    "forename": "Duke",
                    "surname": "Dinsmore",
                    "dob": "1913-04-10",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Duke_Dinsmore"
                }
            },
            {
                "points": 0.0,
                "position": 79,
                "position_text": "79",
                "wins": 0,
                "driver": {
                    "driver_ref": "dick_rathmann",
                    "forename": "Dick",
                    "surname": "Rathmann",
                    "dob": "1924-01-06",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Dick_Rathmann"
                }
            },
            {
                "points": 0.0,
                "position": 78,
                "position_text": "78",
                "wins": 0,
                "driver": {
                    "driver_ref": "hanks",
                    "forename": "Sam",
                    "surname": "Hanks",
                    "dob": "1914-07-13",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Sam_Hanks"
                }
            },
            {
                "points": 0.0,
                "position": 77,
                "position_text": "77",
                "wins": 0,
                "driver": {
                    "driver_ref": "jackson",
                    "forename": "Jimmy",
                    "surname": "Jackson",
                    "dob": "1910-07-25",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Jimmy_Jackson_(driver)"
                }
            },
            {
                "points": 0.0,
                "position": 76,
                "position_text": "76",
                "wins": 0,
                "driver": {
                    "driver_ref": "agabashian",
                    "forename": "Fred",
                    "surname": "Agabashian",
                    "dob": "1913-08-21",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Fred_Agabashian"
                }
            },
            {
                "points": 0.0,
                "position": 75,
                "position_text": "75",
                "wins": 0,
                "driver": {
                    "driver_ref": "levrett",
                    "forename": "Bayliss",
                    "surname": "Levrett",
                    "dob": "1914-02-14",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Bayliss_Levrett"
                }
            },
            {
                "points": 0.0,
                "position": 73,
                "position_text": "73",
                "wins": 0,
                "driver": {
                    "driver_ref": "schindler",
                    "forename": "Bill",
                    "surname": "Schindler",
                    "dob": "1909-03-06",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Bill_Schindler"
                }
            },
            {
                "points": 0.0,
                "position": 81,
                "position_text": "81",
                "wins": 0,
                "driver": {
                    "driver_ref": "banks",
                    "forename": "Henry",
                    "surname": "Banks",
                    "dob": "1913-06-14",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Henry_Banks"
                }
            },
            {
                "points": 0.0,
                "position": 55,
                "position_text": "55",
                "wins": 0,
                "driver": {
                    "driver_ref": "rathmann",
                    "forename": "Jim",
                    "surname": "Rathmann",
                    "dob": "1928-07-16",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Jim_Rathmann"
                }
            },
            {
                "points": 0.0,
                "position": 54,
                "position_text": "54",
                "wins": 0,
                "driver": {
                    "driver_ref": "holmes",
                    "forename": "Jackie",
                    "surname": "Holmes",
                    "dob": "1920-09-04",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Jackie_Holmes"
                }
            },
            {
                "points": 0.0,
                "position": 53,
                "position_text": "53",
                "wins": 0,
                "driver": {
                    "driver_ref": "ader",
                    "forename": "Walt",
                    "surname": "Ader",
                    "dob": "1913-12-15",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Walt_Ader"
                }
            },
            {
                "points": 0.0,
                "position": 52,
                "position_text": "52",
                "wins": 0,
                "driver": {
                    "driver_ref": "hoyt",
                    "forename": "Jerry",
                    "surname": "Hoyt",
                    "dob": "1929-01-29",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Jerry_Hoyt"
                }
            },
            {
                "points": 0.0,
                "position": 51,
                "position_text": "51",
                "wins": 0,
                "driver": {
                    "driver_ref": "webb",
                    "forename": "Travis",
                    "surname": "Webb",
                    "dob": "1910-10-08",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Travis_Webb"
                }
            },
            {
                "points": 0.0,
                "position": 50,
                "position_text": "50",
                "wins": 0,
                "driver": {
                    "driver_ref": "walt_brown",
                    "forename": "Walt",
                    "surname": "Brown",
                    "dob": "1911-12-30",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Walt_Brown_(auto_racer)"
                }
            },
            {
                "points": 0.0,
                "position": 49,
                "position_text": "49",
                "wins": 0,
                "driver": {
                    "driver_ref": "mcdowell",
                    "forename": "Johnny",
                    "surname": "McDowell",
                    "dob": "1915-01-29",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Johnny_McDowell"
                }
            },
            {
                "points": 0.0,
                "position": 48,
                "position_text": "48",
                "wins": 0,
                "driver": {
                    "driver_ref": "davies",
                    "forename": "Jimmy",
                    "surname": "Davies",
                    "dob": "1929-08-08",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Jimmy_Davies"
                }
            }
        ]
    }
];

const SEASON_2023_STANDINGS: [StaticStanding; 1] = driver_standings_from_json![
    {
        "season": 2023,
        "round": 22,
        "driver_standings": [
            {
                "points": 0.0,
                "position": 22,
                "position_text": "22",
                "wins": 0,
                "driver": {
                    "driver_ref": "de_vries",
                    "number": 21,
                    "code": "DEV",
                    "forename": "Nyck",
                    "surname": "de Vries",
                    "dob": "1995-02-06",
                    "nationality": "Dutch",
                    "url": "http://en.wikipedia.org/wiki/Nyck_de_Vries"
                }
            },
            {
                "points": 1.0,
                "position": 21,
                "position_text": "21",
                "wins": 0,
                "driver": {
                    "driver_ref": "sargeant",
                    "number": 2,
                    "code": "SAR",
                    "forename": "Logan",
                    "surname": "Sargeant",
                    "dob": "2000-12-31",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Logan_Sargeant"
                }
            },
            {
                "points": 2.0,
                "position": 20,
                "position_text": "20",
                "wins": 0,
                "driver": {
                    "driver_ref": "lawson",
                    "number": 40,
                    "code": "LAW",
                    "forename": "Liam",
                    "surname": "Lawson",
                    "dob": "2002-02-11",
                    "nationality": "New Zealander",
                    "url": "http://en.wikipedia.org/wiki/Liam_Lawson"
                }
            },
            {
                "points": 3.0,
                "position": 19,
                "position_text": "19",
                "wins": 0,
                "driver": {
                    "driver_ref": "kevin_magnussen",
                    "number": 20,
                    "code": "MAG",
                    "forename": "Kevin",
                    "surname": "Magnussen",
                    "dob": "1992-10-05",
                    "nationality": "Danish",
                    "url": "http://en.wikipedia.org/wiki/Kevin_Magnussen"
                }
            },
            {
                "points": 6.0,
                "position": 17,
                "position_text": "17",
                "wins": 0,
                "driver": {
                    "driver_ref": "ricciardo",
                    "number": 3,
                    "code": "RIC",
                    "forename": "Daniel",
                    "surname": "Ricciardo",
                    "dob": "1989-07-01",
                    "nationality": "Australian",
                    "url": "http://en.wikipedia.org/wiki/Daniel_Ricciardo"
                }
            },
            {
                "points": 6.0,
                "position": 18,
                "position_text": "18",
                "wins": 0,
                "driver": {
                    "driver_ref": "zhou",
                    "number": 24,
                    "code": "ZHO",
                    "forename": "Guanyu",
                    "surname": "Zhou",
                    "dob": "1999-05-30",
                    "nationality": "Chinese",
                    "url": "http://en.wikipedia.org/wiki/Zhou_Guanyu"
                }
            },
            {
                "points": 9.0,
                "position": 16,
                "position_text": "16",
                "wins": 0,
                "driver": {
                    "driver_ref": "hulkenberg",
                    "number": 27,
                    "code": "HUL",
                    "forename": "Nico",
                    "surname": "Hülkenberg",
                    "dob": "1987-08-19",
                    "nationality": "German",
                    "url": "http://en.wikipedia.org/wiki/Nico_H%C3%BClkenberg"
                }
            },
            {
                "points": 10.0,
                "position": 15,
                "position_text": "15",
                "wins": 0,
                "driver": {
                    "driver_ref": "bottas",
                    "number": 77,
                    "code": "BOT",
                    "forename": "Valtteri",
                    "surname": "Bottas",
                    "dob": "1989-08-28",
                    "nationality": "Finnish",
                    "url": "http://en.wikipedia.org/wiki/Valtteri_Bottas"
                }
            },
            {
                "points": 17.0,
                "position": 14,
                "position_text": "14",
                "wins": 0,
                "driver": {
                    "driver_ref": "tsunoda",
                    "number": 22,
                    "code": "TSU",
                    "forename": "Yuki",
                    "surname": "Tsunoda",
                    "dob": "2000-05-11",
                    "nationality": "Japanese",
                    "url": "http://en.wikipedia.org/wiki/Yuki_Tsunoda"
                }
            },
            {
                "points": 27.0,
                "position": 13,
                "position_text": "13",
                "wins": 0,
                "driver": {
                    "driver_ref": "albon",
                    "number": 23,
                    "code": "ALB",
                    "forename": "Alexander",
                    "surname": "Albon",
                    "dob": "1996-03-23",
                    "nationality": "Thai",
                    "url": "http://en.wikipedia.org/wiki/Alexander_Albon"
                }
            },
            {
                "points": 58.0,
                "position": 12,
                "position_text": "12",
                "wins": 0,
                "driver": {
                    "driver_ref": "ocon",
                    "number": 31,
                    "code": "OCO",
                    "forename": "Esteban",
                    "surname": "Ocon",
                    "dob": "1996-09-17",
                    "nationality": "French",
                    "url": "http://en.wikipedia.org/wiki/Esteban_Ocon"
                }
            },
            {
                "points": 62.0,
                "position": 11,
                "position_text": "11",
                "wins": 0,
                "driver": {
                    "driver_ref": "gasly",
                    "number": 10,
                    "code": "GAS",
                    "forename": "Pierre",
                    "surname": "Gasly",
                    "dob": "1996-02-07",
                    "nationality": "French",
                    "url": "http://en.wikipedia.org/wiki/Pierre_Gasly"
                }
            },
            {
                "points": 74.0,
                "position": 10,
                "position_text": "10",
                "wins": 0,
                "driver": {
                    "driver_ref": "stroll",
                    "number": 18,
                    "code": "STR",
                    "forename": "Lance",
                    "surname": "Stroll",
                    "dob": "1998-10-29",
                    "nationality": "Canadian",
                    "url": "http://en.wikipedia.org/wiki/Lance_Stroll"
                }
            },
            {
                "points": 97.0,
                "position": 9,
                "position_text": "9",
                "wins": 0,
                "driver": {
                    "driver_ref": "piastri",
                    "number": 81,
                    "code": "PIA",
                    "forename": "Oscar",
                    "surname": "Piastri",
                    "dob": "2001-04-06",
                    "nationality": "Australian",
                    "url": "http://en.wikipedia.org/wiki/Oscar_Piastri"
                }
            },
            {
                "points": 175.0,
                "position": 8,
                "position_text": "8",
                "wins": 0,
                "driver": {
                    "driver_ref": "russell",
                    "number": 63,
                    "code": "RUS",
                    "forename": "George",
                    "surname": "Russell",
                    "dob": "1998-02-15",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/George_Russell_(racing_driver)"
                }
            },
            {
                "points": 200.0,
                "position": 7,
                "position_text": "7",
                "wins": 1,
                "driver": {
                    "driver_ref": "sainz",
                    "number": 55,
                    "code": "SAI",
                    "forename": "Carlos",
                    "surname": "Sainz",
                    "dob": "1994-09-01",
                    "nationality": "Spanish",
                    "url": "http://en.wikipedia.org/wiki/Carlos_Sainz_Jr."
                }
            },
            {
                "points": 205.0,
                "position": 6,
                "position_text": "6",
                "wins": 0,
                "driver": {
                    "driver_ref": "norris",
                    "number": 4,
                    "code": "NOR",
                    "forename": "Lando",
                    "surname": "Norris",
                    "dob": "1999-11-13",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Lando_Norris"
                }
            },
            {
                "points": 206.0,
                "position": 4,
                "position_text": "4",
                "wins": 0,
                "driver": {
                    "driver_ref": "alonso",
                    "number": 14,
                    "code": "ALO",
                    "forename": "Fernando",
                    "surname": "Alonso",
                    "dob": "1981-07-29",
                    "nationality": "Spanish",
                    "url": "http://en.wikipedia.org/wiki/Fernando_Alonso"
                }
            },
            {
                "points": 206.0,
                "position": 5,
                "position_text": "5",
                "wins": 0,
                "driver": {
                    "driver_ref": "leclerc",
                    "number": 16,
                    "code": "LEC",
                    "forename": "Charles",
                    "surname": "Leclerc",
                    "dob": "1997-10-16",
                    "nationality": "Monegasque",
                    "url": "http://en.wikipedia.org/wiki/Charles_Leclerc"
                }
            },
            {
                "points": 234.0,
                "position": 3,
                "position_text": "3",
                "wins": 0,
                "driver": {
                    "driver_ref": "hamilton",
                    "number": 44,
                    "code": "HAM",
                    "forename": "Lewis",
                    "surname": "Hamilton",
                    "dob": "1985-01-07",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Lewis_Hamilton"
                }
            },
            {
                "points": 285.0,
                "position": 2,
                "position_text": "2",
                "wins": 2,
                "driver": {
                    "driver_ref": "perez",
                    "number": 11,
                    "code": "PER",
                    "forename": "Sergio",
                    "surname": "Pérez",
                    "dob": "1990-01-26",
                    "nationality": "Mexican",
                    "url": "http://en.wikipedia.org/wiki/Sergio_P%C3%A9rez"
                }
            },
            {
                "points": 575.0,
                "position": 1,
                "position_text": "1",
                "wins": 19,
                "driver": {
                    "driver_ref": "max_verstappen",
                    "number": 33,
                    "code": "VER",
                    "forename": "Max",
                    "surname": "Verstappen",
                    "dob": "1997-09-30",
                    "nationality": "Dutch",
                    "url": "http://en.wikipedia.org/wiki/Max_Verstappen"
                }
            }
        ]
    }
];
