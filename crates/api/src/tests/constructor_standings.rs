use shared::prelude::*;

use super::common::models::*;
use super::common::Test;
use crate::constructor_standings_from_json;

#[tokio::test]
async fn test_get_constructor_standings() {
    Test::<&[StaticStanding], Vec<InnerStandingResponse>>::new(
        "/api/f1/constructors/standings",
        Series::F1,
        &ALL_STANDINGS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 31,
        total: 910,
    }))
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_constructor_standings_by_ref() {
    Test::<&[StaticStanding], Vec<InnerStandingResponse>>::new(
        "/api/f1/constructors/standings?constructor_ref=ferrari",
        Series::F1,
        &FERRARI_STANDINGS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 3,
        total: 66,
    }))
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_constructor_standings_by_ref_and_result() {
    Test::<&[StaticStanding], Vec<InnerStandingResponse>>::new(
        "/api/f1/constructors/standings?constructor_ref=ferrari&position=1",
        Series::F1,
        &FERRARI_WINS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 1,
        total: 16,
    }))
    .test_ok()
    .await
}

const ALL_STANDINGS: [StaticStanding; 3] = constructor_standings_from_json![
    {
        "season": 1958,
        "round": 11,
        "constructor_standings": [
            {
                "points": 0.0,
                "position": 8,
                "position_text": "8",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "connaught",
                    "name": "Connaught",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Connaught_Engineering"
                }
            },
            {
                "points": 0.0,
                "position": 9,
                "position_text": "9",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "osca",
                    "name": "OSCA",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Officine_Specializate_Costruzione_Automobili"
                }
            },
            {
                "points": 0.0,
                "position": 7,
                "position_text": "7",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "porsche",
                    "name": "Porsche",
                    "nationality": "German",
                    "url": "http://en.wikipedia.org/wiki/Porsche_in_Formula_One"
                }
            },
            {
                "points": 3.0,
                "position": 6,
                "position_text": "6",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "team_lotus",
                    "name": "Team Lotus",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Team_Lotus"
                }
            },
            {
                "points": 6.0,
                "position": 5,
                "position_text": "5",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "maserati",
                    "name": "Maserati",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Maserati"
                }
            },
            {
                "points": 18.0,
                "position": 4,
                "position_text": "4",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "brm",
                    "name": "BRM",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/BRM"
                }
            },
            {
                "points": 31.0,
                "position": 3,
                "position_text": "3",
                "wins": 2,
                "constructor": {
                    "constructor_ref": "cooper",
                    "name": "Cooper",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Cooper_Car_Company"
                }
            },
            {
                "points": 40.0,
                "position": 2,
                "position_text": "2",
                "wins": 2,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            },
            {
                "points": 48.0,
                "position": 1,
                "position_text": "1",
                "wins": 6,
                "constructor": {
                    "constructor_ref": "vanwall",
                    "name": "Vanwall",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Vanwall"
                }
            }
        ]
    },
    {
        "season": 1959,
        "round": 9,
        "constructor_standings": [
            {
                "points": 0.0,
                "position": 15,
                "position_text": "15",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "vanwall",
                    "name": "Vanwall",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Vanwall"
                }
            },
            {
                "points": 0.0,
                "position": 16,
                "position_text": "16",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "fry",
                    "name": "Fry",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Fry_(racing_team)"
                }
            },
            {
                "points": 0.0,
                "position": 7,
                "position_text": "7",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "kurtis_kraft",
                    "name": "Kurtis Kraft",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Kurtis_Kraft"
                }
            },
            {
                "points": 0.0,
                "position": 11,
                "position_text": "11",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "cooper-osca",
                    "name": "Cooper-OSCA",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Cooper_Car_Company"
                }
            },
            {
                "points": 0.0,
                "position": 12,
                "position_text": "12",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "tec-mec",
                    "name": "Tec-Mec",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Tec-Mec"
                }
            },
            {
                "points": 0.0,
                "position": 13,
                "position_text": "13",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "connaught",
                    "name": "Connaught",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Connaught_Engineering"
                }
            },
            {
                "points": 0.0,
                "position": 14,
                "position_text": "14",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "jbw",
                    "name": "JBW",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/JBW"
                }
            },
            {
                "points": 0.0,
                "position": 10,
                "position_text": "10",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "cooper-borgward",
                    "name": "Cooper-Borgward",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Cooper_Car_Company"
                }
            },
            {
                "points": 0.0,
                "position": 5,
                "position_text": "5",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "aston_martin",
                    "name": "Aston Martin",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Aston_Martin_in_Formula_One"
                }
            },
            {
                "points": 0.0,
                "position": 9,
                "position_text": "9",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "maserati",
                    "name": "Maserati",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Maserati"
                }
            },
            {
                "points": 0.0,
                "position": 8,
                "position_text": "8",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "porsche",
                    "name": "Porsche",
                    "nationality": "German",
                    "url": "http://en.wikipedia.org/wiki/Porsche_in_Formula_One"
                }
            },
            {
                "points": 0.0,
                "position": 6,
                "position_text": "6",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "cooper-maserati",
                    "name": "Cooper-Maserati",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Cooper_Car_Company"
                }
            },
            {
                "points": 5.0,
                "position": 4,
                "position_text": "4",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "team_lotus",
                    "name": "Team Lotus",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Team_Lotus"
                }
            },
            {
                "points": 18.0,
                "position": 3,
                "position_text": "3",
                "wins": 1,
                "constructor": {
                    "constructor_ref": "brm",
                    "name": "BRM",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/BRM"
                }
            },
            {
                "points": 32.0,
                "position": 2,
                "position_text": "2",
                "wins": 2,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            },
            {
                "points": 40.0,
                "position": 1,
                "position_text": "1",
                "wins": 5,
                "constructor": {
                    "constructor_ref": "cooper-climax",
                    "name": "Cooper-Climax",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Cooper_Car_Company"
                }
            }
        ]
    },
    {
        "season": 1960,
        "round": 10,
        "constructor_standings": [
            {
                "points": 0.0,
                "position": 9,
                "position_text": "9",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "scarab",
                    "name": "Scarab",
                    "nationality": "American",
                    "url": "http://en.wikipedia.org/wiki/Scarab_(constructor)"
                }
            },
            {
                "points": 0.0,
                "position": 13,
                "position_text": "13",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "vanwall",
                    "name": "Vanwall",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/Vanwall"
                }
            },
            {
                "points": 0.0,
                "position": 11,
                "position_text": "11",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "maserati",
                    "name": "Maserati",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Maserati"
                }
            },
            {
                "points": 0.0,
                "position": 8,
                "position_text": "8",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "behra-porsche",
                    "name": "Behra-Porsche",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Behra-Porsche"
                }
            },
            {
                "points": 0.0,
                "position": 12,
                "position_text": "12",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "jbw",
                    "name": "JBW",
                    "nationality": "British",
                    "url": "http://en.wikipedia.org/wiki/JBW"
                }
            }
        ]
    }
];

const FERRARI_STANDINGS: [StaticStanding; 30] = constructor_standings_from_json![
    {
        "season": 1958,
        "round": 11,
        "constructor_standings": [
            {
                "points": 40.0,
                "position": 2,
                "position_text": "2",
                "wins": 2,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1959,
        "round": 9,
        "constructor_standings": [
            {
                "points": 32.0,
                "position": 2,
                "position_text": "2",
                "wins": 2,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1960,
        "round": 10,
        "constructor_standings": [
            {
                "points": 26.0,
                "position": 3,
                "position_text": "3",
                "wins": 1,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1961,
        "round": 8,
        "constructor_standings": [
            {
                "points": 45.0,
                "position": 1,
                "position_text": "1",
                "wins": 5,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1962,
        "round": 9,
        "constructor_standings": [
            {
                "points": 18.0,
                "position": 6,
                "position_text": "6",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1963,
        "round": 10,
        "constructor_standings": [
            {
                "points": 26.0,
                "position": 4,
                "position_text": "4",
                "wins": 1,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1964,
        "round": 10,
        "constructor_standings": [
            {
                "points": 45.0,
                "position": 1,
                "position_text": "1",
                "wins": 3,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1965,
        "round": 10,
        "constructor_standings": [
            {
                "points": 26.0,
                "position": 4,
                "position_text": "4",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1966,
        "round": 9,
        "constructor_standings": [
            {
                "points": 31.0,
                "position": 2,
                "position_text": "2",
                "wins": 2,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1967,
        "round": 11,
        "constructor_standings": [
            {
                "points": 20.0,
                "position": 5,
                "position_text": "5",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1968,
        "round": 12,
        "constructor_standings": [
            {
                "points": 32.0,
                "position": 4,
                "position_text": "4",
                "wins": 1,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1969,
        "round": 11,
        "constructor_standings": [
            {
                "points": 7.0,
                "position": 6,
                "position_text": "6",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1970,
        "round": 13,
        "constructor_standings": [
            {
                "points": 52.0,
                "position": 2,
                "position_text": "2",
                "wins": 4,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1971,
        "round": 11,
        "constructor_standings": [
            {
                "points": 33.0,
                "position": 3,
                "position_text": "3",
                "wins": 2,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1972,
        "round": 12,
        "constructor_standings": [
            {
                "points": 33.0,
                "position": 4,
                "position_text": "4",
                "wins": 1,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1973,
        "round": 15,
        "constructor_standings": [
            {
                "points": 12.0,
                "position": 6,
                "position_text": "6",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1974,
        "round": 15,
        "constructor_standings": [
            {
                "points": 65.0,
                "position": 2,
                "position_text": "2",
                "wins": 3,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1975,
        "round": 14,
        "constructor_standings": [
            {
                "points": 72.5,
                "position": 1,
                "position_text": "1",
                "wins": 6,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1976,
        "round": 16,
        "constructor_standings": [
            {
                "points": 83.0,
                "position": 1,
                "position_text": "1",
                "wins": 6,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1977,
        "round": 17,
        "constructor_standings": [
            {
                "points": 95.0,
                "position": 1,
                "position_text": "1",
                "wins": 4,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1978,
        "round": 16,
        "constructor_standings": [
            {
                "points": 58.0,
                "position": 2,
                "position_text": "2",
                "wins": 5,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1979,
        "round": 15,
        "constructor_standings": [
            {
                "points": 113.0,
                "position": 1,
                "position_text": "1",
                "wins": 6,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1980,
        "round": 14,
        "constructor_standings": [
            {
                "points": 8.0,
                "position": 10,
                "position_text": "10",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1981,
        "round": 15,
        "constructor_standings": [
            {
                "points": 34.0,
                "position": 5,
                "position_text": "5",
                "wins": 2,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1982,
        "round": 16,
        "constructor_standings": [
            {
                "points": 74.0,
                "position": 1,
                "position_text": "1",
                "wins": 3,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1983,
        "round": 15,
        "constructor_standings": [
            {
                "points": 89.0,
                "position": 1,
                "position_text": "1",
                "wins": 4,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1984,
        "round": 16,
        "constructor_standings": [
            {
                "points": 57.5,
                "position": 2,
                "position_text": "2",
                "wins": 1,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1985,
        "round": 16,
        "constructor_standings": [
            {
                "points": 82.0,
                "position": 2,
                "position_text": "2",
                "wins": 2,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1986,
        "round": 16,
        "constructor_standings": [
            {
                "points": 37.0,
                "position": 4,
                "position_text": "4",
                "wins": 0,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1987,
        "round": 16,
        "constructor_standings": [
            {
                "points": 53.0,
                "position": 4,
                "position_text": "4",
                "wins": 2,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    }
];

const FERRARI_WINS: [StaticStanding; 16] = constructor_standings_from_json![
    {
        "season": 1961,
        "round": 8,
        "constructor_standings": [
            {
                "points": 45.0,
                "position": 1,
                "position_text": "1",
                "wins": 5,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1964,
        "round": 10,
        "constructor_standings": [
            {
                "points": 45.0,
                "position": 1,
                "position_text": "1",
                "wins": 3,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1975,
        "round": 14,
        "constructor_standings": [
            {
                "points": 72.5,
                "position": 1,
                "position_text": "1",
                "wins": 6,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1976,
        "round": 16,
        "constructor_standings": [
            {
                "points": 83.0,
                "position": 1,
                "position_text": "1",
                "wins": 6,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1977,
        "round": 17,
        "constructor_standings": [
            {
                "points": 95.0,
                "position": 1,
                "position_text": "1",
                "wins": 4,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1979,
        "round": 15,
        "constructor_standings": [
            {
                "points": 113.0,
                "position": 1,
                "position_text": "1",
                "wins": 6,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1982,
        "round": 16,
        "constructor_standings": [
            {
                "points": 74.0,
                "position": 1,
                "position_text": "1",
                "wins": 3,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1983,
        "round": 15,
        "constructor_standings": [
            {
                "points": 89.0,
                "position": 1,
                "position_text": "1",
                "wins": 4,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 1999,
        "round": 16,
        "constructor_standings": [
            {
                "points": 128.0,
                "position": 1,
                "position_text": "1",
                "wins": 6,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 2000,
        "round": 17,
        "constructor_standings": [
            {
                "points": 170.0,
                "position": 1,
                "position_text": "1",
                "wins": 10,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 2001,
        "round": 17,
        "constructor_standings": [
            {
                "points": 179.0,
                "position": 1,
                "position_text": "1",
                "wins": 9,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 2002,
        "round": 17,
        "constructor_standings": [
            {
                "points": 221.0,
                "position": 1,
                "position_text": "1",
                "wins": 15,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 2003,
        "round": 16,
        "constructor_standings": [
            {
                "points": 158.0,
                "position": 1,
                "position_text": "1",
                "wins": 8,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 2004,
        "round": 18,
        "constructor_standings": [
            {
                "points": 262.0,
                "position": 1,
                "position_text": "1",
                "wins": 15,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 2007,
        "round": 17,
        "constructor_standings": [
            {
                "points": 204.0,
                "position": 1,
                "position_text": "1",
                "wins": 9,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    },
    {
        "season": 2008,
        "round": 18,
        "constructor_standings": [
            {
                "points": 172.0,
                "position": 1,
                "position_text": "1",
                "wins": 8,
                "constructor": {
                    "constructor_ref": "ferrari",
                    "name": "Ferrari",
                    "nationality": "Italian",
                    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                }
            }
        ]
    }
];
