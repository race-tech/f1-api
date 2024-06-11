use serde_json::json;

use super::common::Test;

#[tokio::test]
async fn test_get_driver_standings() {
    let all_standings: serde_json::Value = json!({
        "driversStandings": [
                    {
                        "round": 7,
                        "season": 1950,
                        "standings": [
                            {
                                "points": 0.0,
                                "position": 74,
                                "positionText": "74",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 67,
                                "positionText": "67",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 40,
                                "positionText": "40",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 72,
                                "positionText": "72",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 71,
                                "positionText": "71",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 66,
                                "positionText": "66",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 65,
                                "positionText": "65",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 64,
                                "positionText": "64",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 61,
                                "positionText": "61",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 58,
                                "positionText": "58",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 26,
                                "positionText": "26",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 29,
                                "positionText": "29",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 38,
                                "positionText": "38",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 32,
                                "positionText": "32",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 80,
                                "positionText": "80",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 79,
                                "positionText": "79",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 78,
                                "positionText": "78",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 77,
                                "positionText": "77",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 76,
                                "positionText": "76",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 75,
                                "positionText": "75",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 73,
                                "positionText": "73",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 81,
                                "positionText": "81",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 55,
                                "positionText": "55",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 54,
                                "positionText": "54",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 53,
                                "positionText": "53",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 52,
                                "positionText": "52",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 51,
                                "positionText": "51",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 50,
                                "positionText": "50",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 49,
                                "positionText": "49",
                                "wins": 0
                            },
                            {
                                "points": 0.0,
                                "position": 48,
                                "positionText": "48",
                                "wins": 0
                            }
                        ]
                    }
                ]
    });

    Test::new(
        r#"{
            driversStandings(pagination: { limit: 30, page: 1 }) {
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
        all_standings,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_driver_standings_by_year() {
    let value: serde_json::Value = json!({
        "driversStandings": [
                    {
                        "round": 22,
                        "season": 2023,
                        "standings": [
                            {
                                "points": 0.0,
                                "position": 22,
                                "positionText": "22",
                                "wins": 0
                            },
                            {
                                "points": 1.0,
                                "position": 21,
                                "positionText": "21",
                                "wins": 0
                            },
                            {
                                "points": 2.0,
                                "position": 20,
                                "positionText": "20",
                                "wins": 0
                            },
                            {
                                "points": 3.0,
                                "position": 19,
                                "positionText": "19",
                                "wins": 0
                            },
                            {
                                "points": 6.0,
                                "position": 17,
                                "positionText": "17",
                                "wins": 0
                            },
                            {
                                "points": 6.0,
                                "position": 18,
                                "positionText": "18",
                                "wins": 0
                            },
                            {
                                "points": 9.0,
                                "position": 16,
                                "positionText": "16",
                                "wins": 0
                            },
                            {
                                "points": 10.0,
                                "position": 15,
                                "positionText": "15",
                                "wins": 0
                            },
                            {
                                "points": 17.0,
                                "position": 14,
                                "positionText": "14",
                                "wins": 0
                            },
                            {
                                "points": 27.0,
                                "position": 13,
                                "positionText": "13",
                                "wins": 0
                            },
                            {
                                "points": 58.0,
                                "position": 12,
                                "positionText": "12",
                                "wins": 0
                            },
                            {
                                "points": 62.0,
                                "position": 11,
                                "positionText": "11",
                                "wins": 0
                            },
                            {
                                "points": 74.0,
                                "position": 10,
                                "positionText": "10",
                                "wins": 0
                            },
                            {
                                "points": 97.0,
                                "position": 9,
                                "positionText": "9",
                                "wins": 0
                            },
                            {
                                "points": 175.0,
                                "position": 8,
                                "positionText": "8",
                                "wins": 0
                            },
                            {
                                "points": 200.0,
                                "position": 7,
                                "positionText": "7",
                                "wins": 1
                            },
                            {
                                "points": 205.0,
                                "position": 6,
                                "positionText": "6",
                                "wins": 0
                            },
                            {
                                "points": 206.0,
                                "position": 4,
                                "positionText": "4",
                                "wins": 0
                            },
                            {
                                "points": 206.0,
                                "position": 5,
                                "positionText": "5",
                                "wins": 0
                            },
                            {
                                "points": 234.0,
                                "position": 3,
                                "positionText": "3",
                                "wins": 0
                            },
                            {
                                "points": 285.0,
                                "position": 2,
                                "positionText": "2",
                                "wins": 2
                            },
                            {
                                "points": 575.0,
                                "position": 1,
                                "positionText": "1",
                                "wins": 19
                            }
                        ]
                    }
                ]
    });

    Test::new(
        r#"{
            driversStandings(pagination: { limit: 30, page: 1 }, options: { year: 2023 }) {
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
