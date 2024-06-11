use serde_json::json;

use super::common::Test;

#[tokio::test]
async fn test_get_races_by_year() {
    let value: serde_json::Value = json!({
        "races": [
                    {
                        "season": 2024,
                        "round": 1,
                        "name": "Bahrain International Circuit",
                        "date": "2024-03-02",
                        "time": "15:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit",
                        "fp1": {
                            "date": "2024-02-29",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-02-29",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-03-01",
                            "time": "12:30:00.0"
                        },
                        "quali": {
                            "date": "2024-03-01",
                            "time": "16:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 2,
                        "name": "Jeddah Corniche Circuit",
                        "date": "2024-03-09",
                        "time": "17:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Jeddah_Street_Circuit",
                        "fp1": {
                            "date": "2024-03-07",
                            "time": "13:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-03-07",
                            "time": "17:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-03-08",
                            "time": "13:30:00.0"
                        },
                        "quali": {
                            "date": "2024-03-08",
                            "time": "17:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 3,
                        "name": "Albert Park Grand Prix Circuit",
                        "date": "2024-03-24",
                        "time": "4:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit",
                        "fp1": {
                            "date": "2024-03-22",
                            "time": "1:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-03-22",
                            "time": "5:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-03-23",
                            "time": "1:30:00.0"
                        },
                        "quali": {
                            "date": "2024-03-23",
                            "time": "5:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 4,
                        "name": "Suzuka Circuit",
                        "date": "2024-04-07",
                        "time": "5:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit",
                        "fp1": {
                            "date": "2024-04-05",
                            "time": "2:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-04-05",
                            "time": "6:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-04-06",
                            "time": "2:30:00.0"
                        },
                        "quali": {
                            "date": "2024-04-06",
                            "time": "6:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 5,
                        "name": "Shanghai International Circuit",
                        "date": "2024-04-21",
                        "time": "7:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Shanghai_International_Circuit",
                        "fp1": {
                            "date": "2024-04-19",
                            "time": "3:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-04-19",
                            "time": "7:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2024-04-20",
                            "time": "7:00:00.0"
                        },
                        "sprint": {
                            "date": "2024-04-20",
                            "time": "3:00:00.0"
                        }
                    },
                    {
                        "season": 2024,
                        "round": 6,
                        "name": "Miami International Autodrome",
                        "date": "2024-05-05",
                        "time": "20:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Miami_International_Autodrome",
                        "fp1": {
                            "date": "2024-05-03",
                            "time": "16:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-05-03",
                            "time": "20:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2024-05-04",
                            "time": "20:00:00.0"
                        },
                        "sprint": {
                            "date": "2024-05-04",
                            "time": "16:00:00.0"
                        }
                    },
                    {
                        "season": 2024,
                        "round": 7,
                        "name": "Autodromo Enzo e Dino Ferrari",
                        "date": "2024-05-19",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Enzo_e_Dino_Ferrari",
                        "fp1": {
                            "date": "2024-05-17",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-05-17",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-05-18",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2024-05-18",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 8,
                        "name": "Circuit de Monaco",
                        "date": "2024-05-26",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco",
                        "fp1": {
                            "date": "2024-05-24",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-05-24",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-05-25",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2024-05-25",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 9,
                        "name": "Circuit Gilles Villeneuve",
                        "date": "2024-06-09",
                        "time": "18:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_Gilles_Villeneuve",
                        "fp1": {
                            "date": "2024-06-07",
                            "time": "17:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-06-07",
                            "time": "21:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-06-08",
                            "time": "16:30:00.0"
                        },
                        "quali": {
                            "date": "2024-06-08",
                            "time": "20:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 10,
                        "name": "Circuit de Barcelona-Catalunya",
                        "date": "2024-06-23",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Barcelona-Catalunya",
                        "fp1": {
                            "date": "2024-06-21",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-06-21",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-06-22",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2024-06-22",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 11,
                        "name": "Red Bull Ring",
                        "date": "2024-06-30",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring",
                        "fp1": {
                            "date": "2024-06-28",
                            "time": "10:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-06-28",
                            "time": "14:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2024-06-29",
                            "time": "14:00:00.0"
                        },
                        "sprint": {
                            "date": "2024-06-29",
                            "time": "10:00:00.0"
                        }
                    },
                    {
                        "season": 2024,
                        "round": 12,
                        "name": "Silverstone Circuit",
                        "date": "2024-07-07",
                        "time": "14:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit",
                        "fp1": {
                            "date": "2024-07-05",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-07-05",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-07-06",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2024-07-06",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 13,
                        "name": "Hungaroring",
                        "date": "2024-07-21",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Hungaroring",
                        "fp1": {
                            "date": "2024-07-19",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-07-19",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-07-20",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2024-07-20",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 14,
                        "name": "Circuit de Spa-Francorchamps",
                        "date": "2024-07-28",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps",
                        "fp1": {
                            "date": "2024-07-26",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-07-26",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-07-27",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2024-07-27",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 15,
                        "name": "Circuit Park Zandvoort",
                        "date": "2024-08-25",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_Zandvoort",
                        "fp1": {
                            "date": "2024-08-23",
                            "time": "10:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-08-23",
                            "time": "14:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-08-24",
                            "time": "9:30:00.0"
                        },
                        "quali": {
                            "date": "2024-08-24",
                            "time": "13:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 16,
                        "name": "Autodromo Nazionale di Monza",
                        "date": "2024-09-01",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza",
                        "fp1": {
                            "date": "2024-08-30",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-08-30",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-08-31",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2024-08-31",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 17,
                        "name": "Baku City Circuit",
                        "date": "2024-09-15",
                        "time": "11:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit",
                        "fp1": {
                            "date": "2024-09-13",
                            "time": "9:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-09-13",
                            "time": "13:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-09-14",
                            "time": "8:30:00.0"
                        },
                        "quali": {
                            "date": "2024-09-14",
                            "time": "12:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 18,
                        "name": "Marina Bay Street Circuit",
                        "date": "2024-09-22",
                        "time": "12:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Marina_Bay_Street_Circuit",
                        "fp1": {
                            "date": "2024-09-20",
                            "time": "9:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-09-20",
                            "time": "13:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-09-21",
                            "time": "9:30:00.0"
                        },
                        "quali": {
                            "date": "2024-09-21",
                            "time": "13:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 19,
                        "name": "Circuit of the Americas",
                        "date": "2024-10-20",
                        "time": "19:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_of_the_Americas",
                        "fp1": {
                            "date": "2024-10-18",
                            "time": "17:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-10-18",
                            "time": "21:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2024-10-19",
                            "time": "22:00:00.0"
                        },
                        "sprint": {
                            "date": "2024-10-19",
                            "time": "18:00:00.0"
                        }
                    },
                    {
                        "season": 2024,
                        "round": 20,
                        "name": "Autódromo Hermanos Rodríguez",
                        "date": "2024-10-27",
                        "time": "20:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Hermanos_Rodr%C3%ADguez",
                        "fp1": {
                            "date": "2024-10-25",
                            "time": "18:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-10-25",
                            "time": "22:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-10-26",
                            "time": "17:30:00.0"
                        },
                        "quali": {
                            "date": "2024-10-26",
                            "time": "21:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 21,
                        "name": "Autódromo José Carlos Pace",
                        "date": "2024-11-03",
                        "time": "17:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Jos%C3%A9_Carlos_Pace",
                        "fp1": {
                            "date": "2024-11-01",
                            "time": "14:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-11-01",
                            "time": "18:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2024-11-02",
                            "time": "18:00:00.0"
                        },
                        "sprint": {
                            "date": "2024-11-02",
                            "time": "14:00:00.0"
                        }
                    },
                    {
                        "season": 2024,
                        "round": 22,
                        "name": "Las Vegas Strip Street Circuit",
                        "date": "2024-11-23",
                        "time": "6:00:00.0",
                        "url": "https://en.wikipedia.org/wiki/Las_Vegas_Grand_Prix#Circuit",
                        "fp1": {
                            "date": "2024-11-21",
                            "time": "2:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-11-21",
                            "time": "6:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-11-22",
                            "time": "2:30:00.0"
                        },
                        "quali": {
                            "date": "2024-11-22",
                            "time": "6:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 23,
                        "name": "Losail International Circuit",
                        "date": "2024-12-01",
                        "time": "17:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Losail_International_Circuit",
                        "fp1": {
                            "date": "2024-11-29",
                            "time": "13:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-11-29",
                            "time": "17:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2024-11-30",
                            "time": "17:00:00.0"
                        },
                        "sprint": {
                            "date": "2024-11-30",
                            "time": "13:00:00.0"
                        }
                    },
                    {
                        "season": 2024,
                        "round": 24,
                        "name": "Yas Marina Circuit",
                        "date": "2024-12-08",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit",
                        "fp1": {
                            "date": "2024-12-06",
                            "time": "9:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-12-06",
                            "time": "13:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-12-07",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2024-12-07",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    }
                ]
    });

    Test::new(
        r#"{
            races(options: { year: 2024 }, pagination: { limit: 30, page: 1 }) {
                season
                round
                name
                date
                time
                url
                fp1 {
                    date
                    time
                }
                fp2 {
                    date
                    time
                }
                fp3 {
                    date
                    time
                }
                quali {
                    date
                    time
                }
                sprint {
                    date
                    time
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_races_by_year_and_round() {
    Test::new(
        r#"{
            race(year: 2023, round: 1) {
                season
                round
                name
                date
                time
                url
            }
        }"#,
        json!({
            "race": {
                        "season": 2023,
                        "round": 1,
                        "name": "Bahrain International Circuit",
                        "date": "2023-03-05",
                        "time": "15:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit"
                    }
        }),
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_races_by_driver_ref() {
    let value: serde_json::Value = json!({
        "races": [
                    {
                        "season": 2018,
                        "round": 1,
                        "name": "Albert Park Grand Prix Circuit",
                        "date": "2018-03-25",
                        "time": "5:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 2,
                        "name": "Bahrain International Circuit",
                        "date": "2018-04-08",
                        "time": "15:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 3,
                        "name": "Shanghai International Circuit",
                        "date": "2018-04-15",
                        "time": "6:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Shanghai_International_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 4,
                        "name": "Baku City Circuit",
                        "date": "2018-04-29",
                        "time": "12:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 5,
                        "name": "Circuit de Barcelona-Catalunya",
                        "date": "2018-05-13",
                        "time": "13:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Barcelona-Catalunya",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 6,
                        "name": "Circuit de Monaco",
                        "date": "2018-05-27",
                        "time": "13:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 7,
                        "name": "Circuit Gilles Villeneuve",
                        "date": "2018-06-10",
                        "time": "18:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_Gilles_Villeneuve",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 8,
                        "name": "Circuit Paul Ricard",
                        "date": "2018-06-24",
                        "time": "14:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Paul_Ricard_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 9,
                        "name": "Red Bull Ring",
                        "date": "2018-07-01",
                        "time": "13:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 10,
                        "name": "Silverstone Circuit",
                        "date": "2018-07-08",
                        "time": "13:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 11,
                        "name": "Hockenheimring",
                        "date": "2018-07-22",
                        "time": "13:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Hockenheimring",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 12,
                        "name": "Hungaroring",
                        "date": "2018-07-29",
                        "time": "13:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Hungaroring",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 13,
                        "name": "Circuit de Spa-Francorchamps",
                        "date": "2018-08-26",
                        "time": "13:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 14,
                        "name": "Autodromo Nazionale di Monza",
                        "date": "2018-09-02",
                        "time": "13:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 15,
                        "name": "Marina Bay Street Circuit",
                        "date": "2018-09-16",
                        "time": "12:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Marina_Bay_Street_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 16,
                        "name": "Sochi Autodrom",
                        "date": "2018-09-30",
                        "time": "11:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Sochi_Autodrom",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 17,
                        "name": "Suzuka Circuit",
                        "date": "2018-10-07",
                        "time": "5:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 18,
                        "name": "Circuit of the Americas",
                        "date": "2018-10-21",
                        "time": "18:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_of_the_Americas",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 19,
                        "name": "Autódromo Hermanos Rodríguez",
                        "date": "2018-10-28",
                        "time": "19:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Hermanos_Rodr%C3%ADguez",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 20,
                        "name": "Autódromo José Carlos Pace",
                        "date": "2018-11-11",
                        "time": "17:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Jos%C3%A9_Carlos_Pace",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2018,
                        "round": 21,
                        "name": "Yas Marina Circuit",
                        "date": "2018-11-25",
                        "time": "13:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2019,
                        "round": 1,
                        "name": "Albert Park Grand Prix Circuit",
                        "date": "2019-03-17",
                        "time": "5:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2019,
                        "round": 2,
                        "name": "Bahrain International Circuit",
                        "date": "2019-03-31",
                        "time": "15:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2019,
                        "round": 3,
                        "name": "Shanghai International Circuit",
                        "date": "2019-04-14",
                        "time": "6:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Shanghai_International_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2019,
                        "round": 4,
                        "name": "Baku City Circuit",
                        "date": "2019-04-28",
                        "time": "12:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2019,
                        "round": 5,
                        "name": "Circuit de Barcelona-Catalunya",
                        "date": "2019-05-12",
                        "time": "13:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Barcelona-Catalunya",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2019,
                        "round": 6,
                        "name": "Circuit de Monaco",
                        "date": "2019-05-26",
                        "time": "13:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2019,
                        "round": 7,
                        "name": "Circuit Gilles Villeneuve",
                        "date": "2019-06-09",
                        "time": "18:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_Gilles_Villeneuve",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2019,
                        "round": 8,
                        "name": "Circuit Paul Ricard",
                        "date": "2019-06-23",
                        "time": "13:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Paul_Ricard_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 2019,
                        "round": 9,
                        "name": "Red Bull Ring",
                        "date": "2019-06-30",
                        "time": "13:10:00.0",
                        "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    }
                ]
    });

    Test::new(
        r#"{
            races(options: { driverRef: "leclerc" }) {
                season
                round
                name
                date
                time
                url
                fp1 {
                    date
                    time
                }
                fp2 {
                    date
                    time
                }
                fp3 {
                    date
                    time
                }
                quali {
                    date
                    time
                }
                sprint {
                    date
                    time
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_races_by_constructor_ref() {
    let value: serde_json::Value = json!({
        "races": [
                    {
                        "season": 1950,
                        "round": 2,
                        "name": "Circuit de Monaco",
                        "date": "1950-05-21",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1950,
                        "round": 4,
                        "name": "Circuit Bremgarten",
                        "date": "1950-06-04",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Circuit_Bremgarten",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1950,
                        "round": 5,
                        "name": "Circuit de Spa-Francorchamps",
                        "date": "1950-06-18",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1950,
                        "round": 6,
                        "name": "Reims-Gueux",
                        "date": "1950-07-02",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Reims-Gueux",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1950,
                        "round": 7,
                        "name": "Autodromo Nazionale di Monza",
                        "date": "1950-09-03",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1951,
                        "round": 1,
                        "name": "Circuit Bremgarten",
                        "date": "1951-05-27",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Circuit_Bremgarten",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1951,
                        "round": 3,
                        "name": "Circuit de Spa-Francorchamps",
                        "date": "1951-06-17",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1951,
                        "round": 4,
                        "name": "Reims-Gueux",
                        "date": "1951-07-01",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Reims-Gueux",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1951,
                        "round": 5,
                        "name": "Silverstone Circuit",
                        "date": "1951-07-14",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1951,
                        "round": 6,
                        "name": "Nürburgring",
                        "date": "1951-07-29",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/N%C3%BCrburgring",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1951,
                        "round": 7,
                        "name": "Autodromo Nazionale di Monza",
                        "date": "1951-09-16",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1951,
                        "round": 8,
                        "name": "Circuit de Pedralbes",
                        "date": "1951-10-28",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Pedralbes_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1952,
                        "round": 1,
                        "name": "Circuit Bremgarten",
                        "date": "1952-05-18",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Circuit_Bremgarten",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1952,
                        "round": 2,
                        "name": "Indianapolis Motor Speedway",
                        "date": "1952-05-30",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Indianapolis_Motor_Speedway",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1952,
                        "round": 3,
                        "name": "Circuit de Spa-Francorchamps",
                        "date": "1952-06-22",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1952,
                        "round": 4,
                        "name": "Rouen-Les-Essarts",
                        "date": "1952-07-06",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Rouen-Les-Essarts",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1952,
                        "round": 5,
                        "name": "Silverstone Circuit",
                        "date": "1952-07-19",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1952,
                        "round": 6,
                        "name": "Nürburgring",
                        "date": "1952-08-03",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/N%C3%BCrburgring",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1952,
                        "round": 7,
                        "name": "Circuit Park Zandvoort",
                        "date": "1952-08-17",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Circuit_Zandvoort",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1952,
                        "round": 8,
                        "name": "Autodromo Nazionale di Monza",
                        "date": "1952-09-07",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1953,
                        "round": 1,
                        "name": "Autódromo Juan y Oscar Gálvez",
                        "date": "1953-01-18",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Oscar_Alfredo_G%C3%A1lvez",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1953,
                        "round": 3,
                        "name": "Circuit Park Zandvoort",
                        "date": "1953-06-07",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Circuit_Zandvoort",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1953,
                        "round": 4,
                        "name": "Circuit de Spa-Francorchamps",
                        "date": "1953-06-21",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1953,
                        "round": 5,
                        "name": "Reims-Gueux",
                        "date": "1953-07-05",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Reims-Gueux",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1953,
                        "round": 6,
                        "name": "Silverstone Circuit",
                        "date": "1953-07-18",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1953,
                        "round": 7,
                        "name": "Nürburgring",
                        "date": "1953-08-02",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/N%C3%BCrburgring",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1953,
                        "round": 8,
                        "name": "Circuit Bremgarten",
                        "date": "1953-08-23",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Circuit_Bremgarten",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1953,
                        "round": 9,
                        "name": "Autodromo Nazionale di Monza",
                        "date": "1953-09-13",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1954,
                        "round": 1,
                        "name": "Autódromo Juan y Oscar Gálvez",
                        "date": "1954-01-17",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Oscar_Alfredo_G%C3%A1lvez",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    },
                    {
                        "season": 1954,
                        "round": 3,
                        "name": "Circuit de Spa-Francorchamps",
                        "date": "1954-06-20",
                        "time": null,
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps",
                        "fp1": null,
                        "fp2": null,
                        "fp3": null,
                        "quali": null,
                        "sprint": null
                    }
                ]
    });

    Test::new(
        r#"{
            races(options: { constructorRef: "ferrari" }) {
                season
                round
                name
                date
                time
                url
                fp1 {
                    date
                    time
                }
                fp2 {
                    date
                    time
                }
                fp3 {
                    date
                    time
                }
                quali {
                    date
                    time
                }
                sprint {
                    date
                    time
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_races_by_driver_ref_and_page() {
    let value: serde_json::Value = json!({
        "races": [
                    {
                        "season": 2022,
                        "round": 10,
                        "name": "Silverstone Circuit",
                        "date": "2022-07-03",
                        "time": "14:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit",
                        "fp1": {
                            "date": "2022-07-01",
                            "time": "12:00:00.0"
                        },
                        "fp2": {
                            "date": "2022-07-01",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-07-02",
                            "time": "11:00:00.0"
                        },
                        "quali": {
                            "date": "2022-07-02",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2022,
                        "round": 11,
                        "name": "Red Bull Ring",
                        "date": "2022-07-10",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring",
                        "fp1": {
                            "date": "2022-07-08",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2022-07-09",
                            "time": "10:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2022-07-08",
                            "time": "15:00:00.0"
                        },
                        "sprint": {
                            "date": "2022-07-09",
                            "time": "14:30:00.0"
                        }
                    },
                    {
                        "season": 2022,
                        "round": 12,
                        "name": "Circuit Paul Ricard",
                        "date": "2022-07-24",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Paul_Ricard_Circuit",
                        "fp1": {
                            "date": "2022-07-22",
                            "time": "12:00:00.0"
                        },
                        "fp2": {
                            "date": "2022-07-22",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-07-23",
                            "time": "11:00:00.0"
                        },
                        "quali": {
                            "date": "2022-07-23",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2022,
                        "round": 13,
                        "name": "Hungaroring",
                        "date": "2022-07-31",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Hungaroring",
                        "fp1": {
                            "date": "2022-07-29",
                            "time": "12:00:00.0"
                        },
                        "fp2": {
                            "date": "2022-07-29",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-07-30",
                            "time": "11:00:00.0"
                        },
                        "quali": {
                            "date": "2022-07-30",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2022,
                        "round": 14,
                        "name": "Circuit de Spa-Francorchamps",
                        "date": "2022-08-28",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps",
                        "fp1": {
                            "date": "2022-08-26",
                            "time": "12:00:00.0"
                        },
                        "fp2": {
                            "date": "2022-08-26",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-08-27",
                            "time": "11:00:00.0"
                        },
                        "quali": {
                            "date": "2022-08-27",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2022,
                        "round": 15,
                        "name": "Circuit Park Zandvoort",
                        "date": "2022-09-04",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_Zandvoort",
                        "fp1": {
                            "date": "2022-09-02",
                            "time": "10:30:00.0"
                        },
                        "fp2": {
                            "date": "2022-09-02",
                            "time": "14:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-09-03",
                            "time": "10:00:00.0"
                        },
                        "quali": {
                            "date": "2022-09-03",
                            "time": "13:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2022,
                        "round": 16,
                        "name": "Autodromo Nazionale di Monza",
                        "date": "2022-09-11",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza",
                        "fp1": {
                            "date": "2022-09-09",
                            "time": "12:00:00.0"
                        },
                        "fp2": {
                            "date": "2022-09-09",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-09-10",
                            "time": "11:00:00.0"
                        },
                        "quali": {
                            "date": "2022-09-10",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2022,
                        "round": 17,
                        "name": "Marina Bay Street Circuit",
                        "date": "2022-10-02",
                        "time": "12:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Marina_Bay_Street_Circuit",
                        "fp1": {
                            "date": "2022-09-30",
                            "time": "10:00:00.0"
                        },
                        "fp2": {
                            "date": "2022-09-30",
                            "time": "13:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-10-01",
                            "time": "10:00:00.0"
                        },
                        "quali": {
                            "date": "2022-10-01",
                            "time": "13:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2022,
                        "round": 18,
                        "name": "Suzuka Circuit",
                        "date": "2022-10-09",
                        "time": "5:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit",
                        "fp1": {
                            "date": "2022-10-07",
                            "time": "3:00:00.0"
                        },
                        "fp2": {
                            "date": "2022-10-07",
                            "time": "6:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-10-08",
                            "time": "3:00:00.0"
                        },
                        "quali": {
                            "date": "2022-10-08",
                            "time": "6:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2022,
                        "round": 19,
                        "name": "Circuit of the Americas",
                        "date": "2022-10-23",
                        "time": "19:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_of_the_Americas",
                        "fp1": {
                            "date": "2022-10-21",
                            "time": "19:00:00.0"
                        },
                        "fp2": {
                            "date": "2022-10-21",
                            "time": "22:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-10-22",
                            "time": "19:00:00.0"
                        },
                        "quali": {
                            "date": "2022-10-22",
                            "time": "22:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2022,
                        "round": 20,
                        "name": "Autódromo Hermanos Rodríguez",
                        "date": "2022-10-30",
                        "time": "20:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Hermanos_Rodr%C3%ADguez",
                        "fp1": {
                            "date": "2022-10-28",
                            "time": "18:00:00.0"
                        },
                        "fp2": {
                            "date": "2022-10-28",
                            "time": "21:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-10-29",
                            "time": "17:00:00.0"
                        },
                        "quali": {
                            "date": "2022-10-29",
                            "time": "20:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2022,
                        "round": 21,
                        "name": "Autódromo José Carlos Pace",
                        "date": "2022-11-13",
                        "time": "18:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Jos%C3%A9_Carlos_Pace",
                        "fp1": {
                            "date": "2022-11-11",
                            "time": "15:30:00.0"
                        },
                        "fp2": {
                            "date": "2022-11-12",
                            "time": "15:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2022-11-11",
                            "time": "19:00:00.0"
                        },
                        "sprint": {
                            "date": "2022-11-12",
                            "time": "19:30:00.0"
                        }
                    },
                    {
                        "season": 2022,
                        "round": 22,
                        "name": "Yas Marina Circuit",
                        "date": "2022-11-20",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit",
                        "fp1": {
                            "date": "2022-11-18",
                            "time": "10:00:00.0"
                        },
                        "fp2": {
                            "date": "2022-11-18",
                            "time": "13:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-11-19",
                            "time": "11:00:00.0"
                        },
                        "quali": {
                            "date": "2022-11-19",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 1,
                        "name": "Bahrain International Circuit",
                        "date": "2023-03-05",
                        "time": "15:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit",
                        "fp1": {
                            "date": "2023-03-03",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-03-03",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-03-04",
                            "time": "11:30:00.0"
                        },
                        "quali": {
                            "date": "2023-03-04",
                            "time": "15:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 2,
                        "name": "Jeddah Corniche Circuit",
                        "date": "2023-03-19",
                        "time": "17:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Jeddah_Street_Circuit",
                        "fp1": {
                            "date": "2023-03-17",
                            "time": "13:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-03-17",
                            "time": "17:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-03-18",
                            "time": "13:30:00.0"
                        },
                        "quali": {
                            "date": "2023-03-18",
                            "time": "17:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 3,
                        "name": "Albert Park Grand Prix Circuit",
                        "date": "2023-04-02",
                        "time": "5:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit",
                        "fp1": {
                            "date": "2023-03-31",
                            "time": "1:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-03-31",
                            "time": "5:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-04-01",
                            "time": "1:30:00.0"
                        },
                        "quali": {
                            "date": "2023-04-01",
                            "time": "5:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 4,
                        "name": "Baku City Circuit",
                        "date": "2023-04-30",
                        "time": "11:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit",
                        "fp1": {
                            "date": "2023-04-28",
                            "time": "9:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-04-29",
                            "time": "9:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2023-04-28",
                            "time": "13:00:00.0"
                        },
                        "sprint": {
                            "date": "2023-04-29",
                            "time": "13:30:00.0"
                        }
                    },
                    {
                        "season": 2023,
                        "round": 5,
                        "name": "Miami International Autodrome",
                        "date": "2023-05-07",
                        "time": "19:30:00.0",
                        "url": "http://en.wikipedia.org/wiki/Miami_International_Autodrome",
                        "fp1": {
                            "date": "2023-05-05",
                            "time": "18:00:00.0"
                        },
                        "fp2": {
                            "date": "2023-05-05",
                            "time": "21:30:00.0"
                        },
                        "fp3": {
                            "date": "2023-05-06",
                            "time": "16:30:00.0"
                        },
                        "quali": {
                            "date": "2023-05-06",
                            "time": "20:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 6,
                        "name": "Circuit de Monaco",
                        "date": "2023-05-28",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco",
                        "fp1": {
                            "date": "2023-05-26",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-05-26",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-05-27",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2023-05-27",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 7,
                        "name": "Circuit de Barcelona-Catalunya",
                        "date": "2023-06-04",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Barcelona-Catalunya",
                        "fp1": {
                            "date": "2023-06-02",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-06-02",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-06-03",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2023-06-03",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 8,
                        "name": "Circuit Gilles Villeneuve",
                        "date": "2023-06-18",
                        "time": "18:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_Gilles_Villeneuve",
                        "fp1": {
                            "date": "2023-06-16",
                            "time": "17:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-06-16",
                            "time": "21:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-06-17",
                            "time": "16:30:00.0"
                        },
                        "quali": {
                            "date": "2023-06-17",
                            "time": "20:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 9,
                        "name": "Red Bull Ring",
                        "date": "2023-07-02",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring",
                        "fp1": {
                            "date": "2023-06-30",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-07-01",
                            "time": "10:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2023-06-30",
                            "time": "15:00:00.0"
                        },
                        "sprint": {
                            "date": "2023-07-01",
                            "time": "14:30:00.0"
                        }
                    },
                    {
                        "season": 2023,
                        "round": 10,
                        "name": "Silverstone Circuit",
                        "date": "2023-07-09",
                        "time": "14:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit",
                        "fp1": {
                            "date": "2023-07-07",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-07-07",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-07-08",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2023-07-08",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 11,
                        "name": "Hungaroring",
                        "date": "2023-07-23",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Hungaroring",
                        "fp1": {
                            "date": "2023-07-21",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-07-21",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-07-22",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2023-07-22",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 12,
                        "name": "Circuit de Spa-Francorchamps",
                        "date": "2023-07-30",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps",
                        "fp1": {
                            "date": "2023-07-28",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-07-29",
                            "time": "10:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2023-07-28",
                            "time": "15:00:00.0"
                        },
                        "sprint": {
                            "date": "2023-07-29",
                            "time": "14:30:00.0"
                        }
                    },
                    {
                        "season": 2023,
                        "round": 13,
                        "name": "Circuit Park Zandvoort",
                        "date": "2023-08-27",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_Zandvoort",
                        "fp1": {
                            "date": "2023-08-25",
                            "time": "10:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-08-25",
                            "time": "14:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-08-26",
                            "time": "9:30:00.0"
                        },
                        "quali": {
                            "date": "2023-08-26",
                            "time": "13:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 14,
                        "name": "Autodromo Nazionale di Monza",
                        "date": "2023-09-03",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza",
                        "fp1": {
                            "date": "2023-09-01",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-09-01",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-09-02",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2023-09-02",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 15,
                        "name": "Marina Bay Street Circuit",
                        "date": "2023-09-17",
                        "time": "12:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Marina_Bay_Street_Circuit",
                        "fp1": {
                            "date": "2023-09-15",
                            "time": "9:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-09-15",
                            "time": "13:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-09-16",
                            "time": "9:30:00.0"
                        },
                        "quali": {
                            "date": "2023-09-16",
                            "time": "13:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 16,
                        "name": "Suzuka Circuit",
                        "date": "2023-09-24",
                        "time": "5:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit",
                        "fp1": {
                            "date": "2023-09-22",
                            "time": "2:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-09-22",
                            "time": "6:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-09-23",
                            "time": "2:30:00.0"
                        },
                        "quali": {
                            "date": "2023-09-23",
                            "time": "6:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 17,
                        "name": "Losail International Circuit",
                        "date": "2023-10-08",
                        "time": "17:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Losail_International_Circuit",
                        "fp1": {
                            "date": "2023-10-06",
                            "time": "13:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-10-07",
                            "time": "13:00:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2023-10-06",
                            "time": "17:00:00.0"
                        },
                        "sprint": {
                            "date": "2023-10-07",
                            "time": "17:30:00.0"
                        }
                    }
                ]
    });

    Test::new(
        r#"{
            races(options: { driverRef: "leclerc" }, pagination: { page: 4, limit: 30 }) {
                season
                round
                name
                date
                time
                url
                fp1 {
                    date
                    time
                }
                fp2 {
                    date
                    time
                }
                fp3 {
                    date
                    time
                }
                quali {
                    date
                    time
                }
                sprint {
                    date
                    time
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_races_by_constructor_ref_and_page() {
    let value: serde_json::Value = json!({
        "races": [
                    {
                        "season": 2022,
                        "round": 19,
                        "name": "Circuit of the Americas",
                        "date": "2022-10-23",
                        "time": "19:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_of_the_Americas",
                        "fp1": {
                            "date": "2022-10-21",
                            "time": "19:00:00.0"
                        },
                        "fp2": {
                            "date": "2022-10-21",
                            "time": "22:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-10-22",
                            "time": "19:00:00.0"
                        },
                        "quali": {
                            "date": "2022-10-22",
                            "time": "22:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2022,
                        "round": 20,
                        "name": "Autódromo Hermanos Rodríguez",
                        "date": "2022-10-30",
                        "time": "20:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Hermanos_Rodr%C3%ADguez",
                        "fp1": {
                            "date": "2022-10-28",
                            "time": "18:00:00.0"
                        },
                        "fp2": {
                            "date": "2022-10-28",
                            "time": "21:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-10-29",
                            "time": "17:00:00.0"
                        },
                        "quali": {
                            "date": "2022-10-29",
                            "time": "20:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2022,
                        "round": 21,
                        "name": "Autódromo José Carlos Pace",
                        "date": "2022-11-13",
                        "time": "18:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Jos%C3%A9_Carlos_Pace",
                        "fp1": {
                            "date": "2022-11-11",
                            "time": "15:30:00.0"
                        },
                        "fp2": {
                            "date": "2022-11-12",
                            "time": "15:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2022-11-11",
                            "time": "19:00:00.0"
                        },
                        "sprint": {
                            "date": "2022-11-12",
                            "time": "19:30:00.0"
                        }
                    },
                    {
                        "season": 2022,
                        "round": 22,
                        "name": "Yas Marina Circuit",
                        "date": "2022-11-20",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit",
                        "fp1": {
                            "date": "2022-11-18",
                            "time": "10:00:00.0"
                        },
                        "fp2": {
                            "date": "2022-11-18",
                            "time": "13:00:00.0"
                        },
                        "fp3": {
                            "date": "2022-11-19",
                            "time": "11:00:00.0"
                        },
                        "quali": {
                            "date": "2022-11-19",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 1,
                        "name": "Bahrain International Circuit",
                        "date": "2023-03-05",
                        "time": "15:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit",
                        "fp1": {
                            "date": "2023-03-03",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-03-03",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-03-04",
                            "time": "11:30:00.0"
                        },
                        "quali": {
                            "date": "2023-03-04",
                            "time": "15:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 2,
                        "name": "Jeddah Corniche Circuit",
                        "date": "2023-03-19",
                        "time": "17:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Jeddah_Street_Circuit",
                        "fp1": {
                            "date": "2023-03-17",
                            "time": "13:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-03-17",
                            "time": "17:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-03-18",
                            "time": "13:30:00.0"
                        },
                        "quali": {
                            "date": "2023-03-18",
                            "time": "17:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 3,
                        "name": "Albert Park Grand Prix Circuit",
                        "date": "2023-04-02",
                        "time": "5:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit",
                        "fp1": {
                            "date": "2023-03-31",
                            "time": "1:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-03-31",
                            "time": "5:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-04-01",
                            "time": "1:30:00.0"
                        },
                        "quali": {
                            "date": "2023-04-01",
                            "time": "5:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 4,
                        "name": "Baku City Circuit",
                        "date": "2023-04-30",
                        "time": "11:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit",
                        "fp1": {
                            "date": "2023-04-28",
                            "time": "9:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-04-29",
                            "time": "9:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2023-04-28",
                            "time": "13:00:00.0"
                        },
                        "sprint": {
                            "date": "2023-04-29",
                            "time": "13:30:00.0"
                        }
                    },
                    {
                        "season": 2023,
                        "round": 5,
                        "name": "Miami International Autodrome",
                        "date": "2023-05-07",
                        "time": "19:30:00.0",
                        "url": "http://en.wikipedia.org/wiki/Miami_International_Autodrome",
                        "fp1": {
                            "date": "2023-05-05",
                            "time": "18:00:00.0"
                        },
                        "fp2": {
                            "date": "2023-05-05",
                            "time": "21:30:00.0"
                        },
                        "fp3": {
                            "date": "2023-05-06",
                            "time": "16:30:00.0"
                        },
                        "quali": {
                            "date": "2023-05-06",
                            "time": "20:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 6,
                        "name": "Circuit de Monaco",
                        "date": "2023-05-28",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Monaco",
                        "fp1": {
                            "date": "2023-05-26",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-05-26",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-05-27",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2023-05-27",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 7,
                        "name": "Circuit de Barcelona-Catalunya",
                        "date": "2023-06-04",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Barcelona-Catalunya",
                        "fp1": {
                            "date": "2023-06-02",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-06-02",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-06-03",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2023-06-03",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 8,
                        "name": "Circuit Gilles Villeneuve",
                        "date": "2023-06-18",
                        "time": "18:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_Gilles_Villeneuve",
                        "fp1": {
                            "date": "2023-06-16",
                            "time": "17:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-06-16",
                            "time": "21:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-06-17",
                            "time": "16:30:00.0"
                        },
                        "quali": {
                            "date": "2023-06-17",
                            "time": "20:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 9,
                        "name": "Red Bull Ring",
                        "date": "2023-07-02",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Red_Bull_Ring",
                        "fp1": {
                            "date": "2023-06-30",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-07-01",
                            "time": "10:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2023-06-30",
                            "time": "15:00:00.0"
                        },
                        "sprint": {
                            "date": "2023-07-01",
                            "time": "14:30:00.0"
                        }
                    },
                    {
                        "season": 2023,
                        "round": 10,
                        "name": "Silverstone Circuit",
                        "date": "2023-07-09",
                        "time": "14:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit",
                        "fp1": {
                            "date": "2023-07-07",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-07-07",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-07-08",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2023-07-08",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 11,
                        "name": "Hungaroring",
                        "date": "2023-07-23",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Hungaroring",
                        "fp1": {
                            "date": "2023-07-21",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-07-21",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-07-22",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2023-07-22",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 12,
                        "name": "Circuit de Spa-Francorchamps",
                        "date": "2023-07-30",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps",
                        "fp1": {
                            "date": "2023-07-28",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-07-29",
                            "time": "10:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2023-07-28",
                            "time": "15:00:00.0"
                        },
                        "sprint": {
                            "date": "2023-07-29",
                            "time": "14:30:00.0"
                        }
                    },
                    {
                        "season": 2023,
                        "round": 13,
                        "name": "Circuit Park Zandvoort",
                        "date": "2023-08-27",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_Zandvoort",
                        "fp1": {
                            "date": "2023-08-25",
                            "time": "10:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-08-25",
                            "time": "14:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-08-26",
                            "time": "9:30:00.0"
                        },
                        "quali": {
                            "date": "2023-08-26",
                            "time": "13:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 14,
                        "name": "Autodromo Nazionale di Monza",
                        "date": "2023-09-03",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Autodromo_Nazionale_Monza",
                        "fp1": {
                            "date": "2023-09-01",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-09-01",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-09-02",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2023-09-02",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 15,
                        "name": "Marina Bay Street Circuit",
                        "date": "2023-09-17",
                        "time": "12:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Marina_Bay_Street_Circuit",
                        "fp1": {
                            "date": "2023-09-15",
                            "time": "9:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-09-15",
                            "time": "13:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-09-16",
                            "time": "9:30:00.0"
                        },
                        "quali": {
                            "date": "2023-09-16",
                            "time": "13:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 16,
                        "name": "Suzuka Circuit",
                        "date": "2023-09-24",
                        "time": "5:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit",
                        "fp1": {
                            "date": "2023-09-22",
                            "time": "2:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-09-22",
                            "time": "6:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-09-23",
                            "time": "2:30:00.0"
                        },
                        "quali": {
                            "date": "2023-09-23",
                            "time": "6:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 17,
                        "name": "Losail International Circuit",
                        "date": "2023-10-08",
                        "time": "17:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Losail_International_Circuit",
                        "fp1": {
                            "date": "2023-10-06",
                            "time": "13:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-10-07",
                            "time": "13:00:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2023-10-06",
                            "time": "17:00:00.0"
                        },
                        "sprint": {
                            "date": "2023-10-07",
                            "time": "17:30:00.0"
                        }
                    },
                    {
                        "season": 2023,
                        "round": 18,
                        "name": "Circuit of the Americas",
                        "date": "2023-10-22",
                        "time": "19:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Circuit_of_the_Americas",
                        "fp1": {
                            "date": "2023-10-20",
                            "time": "17:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-10-21",
                            "time": "18:00:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2023-10-20",
                            "time": "21:00:00.0"
                        },
                        "sprint": {
                            "date": "2023-10-21",
                            "time": "22:00:00.0"
                        }
                    },
                    {
                        "season": 2023,
                        "round": 19,
                        "name": "Autódromo Hermanos Rodríguez",
                        "date": "2023-10-29",
                        "time": "20:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Hermanos_Rodr%C3%ADguez",
                        "fp1": {
                            "date": "2023-10-27",
                            "time": "18:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-10-27",
                            "time": "22:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-10-28",
                            "time": "17:30:00.0"
                        },
                        "quali": {
                            "date": "2023-10-28",
                            "time": "21:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 20,
                        "name": "Autódromo José Carlos Pace",
                        "date": "2023-11-05",
                        "time": "17:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Aut%C3%B3dromo_Jos%C3%A9_Carlos_Pace",
                        "fp1": {
                            "date": "2023-11-03",
                            "time": "14:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-11-04",
                            "time": "14:30:00.0"
                        },
                        "fp3": null,
                        "quali": {
                            "date": "2023-11-03",
                            "time": "18:00:00.0"
                        },
                        "sprint": {
                            "date": "2023-11-04",
                            "time": "18:30:00.0"
                        }
                    },
                    {
                        "season": 2023,
                        "round": 21,
                        "name": "Las Vegas Strip Street Circuit",
                        "date": "2023-11-19",
                        "time": "6:00:00.0",
                        "url": "https://en.wikipedia.org/wiki/Las_Vegas_Grand_Prix#Circuit",
                        "fp1": {
                            "date": "2023-11-16",
                            "time": "4:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-11-16",
                            "time": "8:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-11-17",
                            "time": "4:30:00.0"
                        },
                        "quali": {
                            "date": "2023-11-17",
                            "time": "8:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2023,
                        "round": 22,
                        "name": "Yas Marina Circuit",
                        "date": "2023-11-26",
                        "time": "13:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit",
                        "fp1": {
                            "date": "2023-11-24",
                            "time": "9:30:00.0"
                        },
                        "fp2": {
                            "date": "2023-11-24",
                            "time": "13:00:00.0"
                        },
                        "fp3": {
                            "date": "2023-11-25",
                            "time": "10:30:00.0"
                        },
                        "quali": {
                            "date": "2023-11-25",
                            "time": "14:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 1,
                        "name": "Bahrain International Circuit",
                        "date": "2024-03-02",
                        "time": "15:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Bahrain_International_Circuit",
                        "fp1": {
                            "date": "2024-02-29",
                            "time": "11:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-02-29",
                            "time": "15:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-03-01",
                            "time": "12:30:00.0"
                        },
                        "quali": {
                            "date": "2024-03-01",
                            "time": "16:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 2,
                        "name": "Jeddah Corniche Circuit",
                        "date": "2024-03-09",
                        "time": "17:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Jeddah_Street_Circuit",
                        "fp1": {
                            "date": "2024-03-07",
                            "time": "13:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-03-07",
                            "time": "17:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-03-08",
                            "time": "13:30:00.0"
                        },
                        "quali": {
                            "date": "2024-03-08",
                            "time": "17:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 3,
                        "name": "Albert Park Grand Prix Circuit",
                        "date": "2024-03-24",
                        "time": "4:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Melbourne_Grand_Prix_Circuit",
                        "fp1": {
                            "date": "2024-03-22",
                            "time": "1:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-03-22",
                            "time": "5:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-03-23",
                            "time": "1:30:00.0"
                        },
                        "quali": {
                            "date": "2024-03-23",
                            "time": "5:00:00.0"
                        },
                        "sprint": null
                    },
                    {
                        "season": 2024,
                        "round": 4,
                        "name": "Suzuka Circuit",
                        "date": "2024-04-07",
                        "time": "5:00:00.0",
                        "url": "http://en.wikipedia.org/wiki/Suzuka_Circuit",
                        "fp1": {
                            "date": "2024-04-05",
                            "time": "2:30:00.0"
                        },
                        "fp2": {
                            "date": "2024-04-05",
                            "time": "6:00:00.0"
                        },
                        "fp3": {
                            "date": "2024-04-06",
                            "time": "2:30:00.0"
                        },
                        "quali": {
                            "date": "2024-04-06",
                            "time": "6:00:00.0"
                        },
                        "sprint": null
                    }
                ]
    });

    Test::new(
        r#"{
            races(
                options: { constructorRef: "ferrari" }
                pagination: { page: 36, limit: 30 }
            ) {
                season
                round
                name
                date
                time
                url
                fp1 {
                    date
                    time
                }
                fp2 {
                    date
                    time
                }
                fp3 {
                    date
                    time
                }
                quali {
                    date
                    time
                }
                sprint {
                    date
                    time
                }
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}
