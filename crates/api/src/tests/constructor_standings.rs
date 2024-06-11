use serde_json::json;

use super::common::Test;

#[tokio::test]
async fn test_get_constructor_standings() {
    let value = json!({
        "constructorsStandings": [
                    {
                        "round": 11,
                        "season": 1958,
                        "standings": [
                            {
                                "points": 0.0,
                                "position": 8,
                                "positionText": "8",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 9,
                                "positionText": "9",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 7,
                                "positionText": "7",
                                "wins": 0
                            },
                            {
                                "points": 3.0,
                                "position": 6,
                                "positionText": "6",
                                "wins": 0
                            },
                            {
                                "points": 6.0,
                                "position": 5,
                                "positionText": "5",
                                "wins": 0
                            },
                            {
                                "points": 18.0,
                                "position": 4,
                                "positionText": "4",
                                "wins": 0
                            },
                            {
                                "points": 31.0,
                                "position": 3,
                                "positionText": "3",
                                "wins": 2
                            },
                            {
                                "points": 40.0,
                                "position": 2,
                                "positionText": "2",
                                "wins": 2
                            },
                            {
                                "points": 48.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 6
                            }
                        ]
                    },
                    {
                        "round": 9,
                        "season": 1959,
                        "standings": [
                            {
                                "points": 0.0,
                                "position": 15,
                                "positionText": "15",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 16,
                                "positionText": "16",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 7,
                                "positionText": "7",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 11,
                                "positionText": "11",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 12,
                                "positionText": "12",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 13,
                                "positionText": "13",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 14,
                                "positionText": "14",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 10,
                                "positionText": "10",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 5,
                                "positionText": "5",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 9,
                                "positionText": "9",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 8,
                                "positionText": "8",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 6,
                                "positionText": "6",
                                "wins": 0
                            },
                            {
                                "points": 5.0,
                                "position": 4,
                                "positionText": "4",
                                "wins": 0
                            },
                            {
                                "points": 18.0,
                                "position": 3,
                                "positionText": "3",
                                "wins": 1
                            },
                            {
                                "points": 32.0,
                                "position": 2,
                                "positionText": "2",
                                "wins": 2
                            },
                            {
                                "points": 40.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 5
                            }
                        ]
                    },
                    {
                        "round": 10,
                        "season": 1960,
                        "standings": [
                            {
                                "points": 0.0,
                                "position": 9,
                                "positionText": "9",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 13,
                                "positionText": "13",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 11,
                                "positionText": "11",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 8,
                                "positionText": "8",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 12,
                                "positionText": "12",
                                "wins": 0
                            }
                        ]
                    }
                ]
    });

    Test::new(
        r#"{
            constructorsStandings(options: null, pagination: { limit: 30, page: 1 }) {
                round
                season
                standings {
                    points
                    position
                    positionText
                    wins
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_constructor_standings_by_ref() {
    let value: serde_json::Value = json!({
        "constructorsStandings": [
                    {
                        "round": 11,
                        "season": 1958,
                        "standings": [
                            {
                                "points": 40.0,
                                "position": 2,
                                "positionText": "2",
                                "wins": 2
                            }
                        ]
                    },
                    {
                        "round": 9,
                        "season": 1959,
                        "standings": [
                            {
                                "points": 32.0,
                                "position": 2,
                                "positionText": "2",
                                "wins": 2
                            }
                        ]
                    },
                    {
                        "round": 10,
                        "season": 1960,
                        "standings": [
                            {
                                "points": 26.0,
                                "position": 3,
                                "positionText": "3",
                                "wins": 1
                            }
                        ]
                    },
                    {
                        "round": 8,
                        "season": 1961,
                        "standings": [
                            {
                                "points": 45.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 5
                            }
                        ]
                    },
                    {
                        "round": 9,
                        "season": 1962,
                        "standings": [
                            {
                                "points": 18.0,
                                "position": 6,
                                "positionText": "6",
                                "wins": 0
                            }
                        ]
                    },
                    {
                        "round": 10,
                        "season": 1963,
                        "standings": [
                            {
                                "points": 26.0,
                                "position": 4,
                                "positionText": "4",
                                "wins": 1
                            }
                        ]
                    },
                    {
                        "round": 10,
                        "season": 1964,
                        "standings": [
                            {
                                "points": 45.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 3
                            }
                        ]
                    },
                    {
                        "round": 10,
                        "season": 1965,
                        "standings": [
                            {
                                "points": 26.0,
                                "position": 4,
                                "positionText": "4",
                                "wins": 0
                            }
                        ]
                    },
                    {
                        "round": 9,
                        "season": 1966,
                        "standings": [
                            {
                                "points": 31.0,
                                "position": 2,
                                "positionText": "2",
                                "wins": 2
                            }
                        ]
                    },
                    {
                        "round": 11,
                        "season": 1967,
                        "standings": [
                            {
                                "points": 20.0,
                                "position": 5,
                                "positionText": "5",
                                "wins": 0
                            }
                        ]
                    },
                    {
                        "round": 12,
                        "season": 1968,
                        "standings": [
                            {
                                "points": 32.0,
                                "position": 4,
                                "positionText": "4",
                                "wins": 1
                            }
                        ]
                    },
                    {
                        "round": 11,
                        "season": 1969,
                        "standings": [
                            {
                                "points": 7.0,
                                "position": 6,
                                "positionText": "6",
                                "wins": 0
                            }
                        ]
                    },
                    {
                        "round": 13,
                        "season": 1970,
                        "standings": [
                            {
                                "points": 52.0,
                                "position": 2,
                                "positionText": "2",
                                "wins": 4
                            }
                        ]
                    },
                    {
                        "round": 11,
                        "season": 1971,
                        "standings": [
                            {
                                "points": 33.0,
                                "position": 3,
                                "positionText": "3",
                                "wins": 2
                            }
                        ]
                    },
                    {
                        "round": 12,
                        "season": 1972,
                        "standings": [
                            {
                                "points": 33.0,
                                "position": 4,
                                "positionText": "4",
                                "wins": 1
                            }
                        ]
                    },
                    {
                        "round": 15,
                        "season": 1973,
                        "standings": [
                            {
                                "points": 12.0,
                                "position": 6,
                                "positionText": "6",
                                "wins": 0
                            }
                        ]
                    },
                    {
                        "round": 15,
                        "season": 1974,
                        "standings": [
                            {
                                "points": 65.0,
                                "position": 2,
                                "positionText": "2",
                                "wins": 3
                            }
                        ]
                    },
                    {
                        "round": 14,
                        "season": 1975,
                        "standings": [
                            {
                                "points": 72.5,
                                "position": 1,
                                "positionText": "1",
                                "wins": 6
                            }
                        ]
                    },
                    {
                        "round": 16,
                        "season": 1976,
                        "standings": [
                            {
                                "points": 83.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 6
                            }
                        ]
                    },
                    {
                        "round": 17,
                        "season": 1977,
                        "standings": [
                            {
                                "points": 95.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 4
                            }
                        ]
                    },
                    {
                        "round": 16,
                        "season": 1978,
                        "standings": [
                            {
                                "points": 58.0,
                                "position": 2,
                                "positionText": "2",
                                "wins": 5
                            }
                        ]
                    },
                    {
                        "round": 15,
                        "season": 1979,
                        "standings": [
                            {
                                "points": 113.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 6
                            }
                        ]
                    },
                    {
                        "round": 14,
                        "season": 1980,
                        "standings": [
                            {
                                "points": 8.0,
                                "position": 10,
                                "positionText": "10",
                                "wins": 0
                            }
                        ]
                    },
                    {
                        "round": 15,
                        "season": 1981,
                        "standings": [
                            {
                                "points": 34.0,
                                "position": 5,
                                "positionText": "5",
                                "wins": 2
                            }
                        ]
                    },
                    {
                        "round": 16,
                        "season": 1982,
                        "standings": [
                            {
                                "points": 74.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 3
                            }
                        ]
                    },
                    {
                        "round": 15,
                        "season": 1983,
                        "standings": [
                            {
                                "points": 89.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 4
                            }
                        ]
                    },
                    {
                        "round": 16,
                        "season": 1984,
                        "standings": [
                            {
                                "points": 57.5,
                                "position": 2,
                                "positionText": "2",
                                "wins": 1
                            }
                        ]
                    },
                    {
                        "round": 16,
                        "season": 1985,
                        "standings": [
                            {
                                "points": 82.0,
                                "position": 2,
                                "positionText": "2",
                                "wins": 2
                            }
                        ]
                    },
                    {
                        "round": 16,
                        "season": 1986,
                        "standings": [
                            {
                                "points": 37.0,
                                "position": 4,
                                "positionText": "4",
                                "wins": 0
                            }
                        ]
                    },
                    {
                        "round": 16,
                        "season": 1987,
                        "standings": [
                            {
                                "points": 53.0,
                                "position": 4,
                                "positionText": "4",
                                "wins": 2
                            }
                        ]
                    }
                ]
    });

    Test::new(
        r#"{
            constructorsStandings(
                options: { constructorRef: "ferrari" }
                pagination: { limit: 30, page: 1 }
            ) {
                round
                season
                standings {
                    points
                    position
                    positionText
                    wins
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_constructor_standings_by_ref_and_result() {
    let value: serde_json::Value = json!({
        "constructorsStandings": [
                    {
                        "round": 8,
                        "season": 1961,
                        "standings": [
                            {
                                "points": 45.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 5
                            }
                        ]
                    },
                    {
                        "round": 10,
                        "season": 1964,
                        "standings": [
                            {
                                "points": 45.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 3
                            }
                        ]
                    },
                    {
                        "round": 14,
                        "season": 1975,
                        "standings": [
                            {
                                "points": 72.5,
                                "position": 1,
                                "positionText": "1",
                                "wins": 6
                            }
                        ]
                    },
                    {
                        "round": 16,
                        "season": 1976,
                        "standings": [
                            {
                                "points": 83.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 6
                            }
                        ]
                    },
                    {
                        "round": 17,
                        "season": 1977,
                        "standings": [
                            {
                                "points": 95.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 4
                            }
                        ]
                    },
                    {
                        "round": 15,
                        "season": 1979,
                        "standings": [
                            {
                                "points": 113.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 6
                            }
                        ]
                    },
                    {
                        "round": 16,
                        "season": 1982,
                        "standings": [
                            {
                                "points": 74.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 3
                            }
                        ]
                    },
                    {
                        "round": 15,
                        "season": 1983,
                        "standings": [
                            {
                                "points": 89.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 4
                            }
                        ]
                    },
                    {
                        "round": 16,
                        "season": 1999,
                        "standings": [
                            {
                                "points": 128.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 6
                            }
                        ]
                    },
                    {
                        "round": 17,
                        "season": 2000,
                        "standings": [
                            {
                                "points": 170.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 10
                            }
                        ]
                    },
                    {
                        "round": 17,
                        "season": 2001,
                        "standings": [
                            {
                                "points": 179.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 9
                            }
                        ]
                    },
                    {
                        "round": 17,
                        "season": 2002,
                        "standings": [
                            {
                                "points": 221.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 15
                            }
                        ]
                    },
                    {
                        "round": 16,
                        "season": 2003,
                        "standings": [
                            {
                                "points": 158.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 8
                            }
                        ]
                    },
                    {
                        "round": 18,
                        "season": 2004,
                        "standings": [
                            {
                                "points": 262.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 15
                            }
                        ]
                    },
                    {
                        "round": 17,
                        "season": 2007,
                        "standings": [
                            {
                                "points": 204.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 9
                            }
                        ]
                    },
                    {
                        "round": 18,
                        "season": 2008,
                        "standings": [
                            {
                                "points": 172.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 8
                            }
                        ]
                    }
                ]
    });

    Test::new(
        r#"{
            constructorsStandings(
                options: { constructorRef: "ferrari", position: 1 }
                pagination: { limit: 30, page: 1 }
            ) {
                round
                season
                standings {
                    points
                    position
                    positionText
                    wins
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}
