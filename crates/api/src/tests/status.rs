use serde_json::json;

use super::common::Test;

#[tokio::test]
async fn test_get_status() {
    let value: serde_json::Value = json!({
        "status": [
                    {
                        "statusId": 1,
                        "status": "Finished",
                        "count": 7427
                    },
                    {
                        "statusId": 2,
                        "status": "Disqualified",
                        "count": 145
                    },
                    {
                        "statusId": 3,
                        "status": "Accident",
                        "count": 1057
                    },
                    {
                        "statusId": 4,
                        "status": "Collision",
                        "count": 842
                    },
                    {
                        "statusId": 5,
                        "status": "Engine",
                        "count": 2022
                    },
                    {
                        "statusId": 6,
                        "status": "Gearbox",
                        "count": 809
                    },
                    {
                        "statusId": 7,
                        "status": "Transmission",
                        "count": 321
                    },
                    {
                        "statusId": 8,
                        "status": "Clutch",
                        "count": 214
                    },
                    {
                        "statusId": 9,
                        "status": "Hydraulics",
                        "count": 138
                    },
                    {
                        "statusId": 10,
                        "status": "Electrical",
                        "count": 316
                    },
                    {
                        "statusId": 11,
                        "status": "+1 Lap",
                        "count": 3944
                    },
                    {
                        "statusId": 12,
                        "status": "+2 Laps",
                        "count": 1600
                    },
                    {
                        "statusId": 13,
                        "status": "+3 Laps",
                        "count": 731
                    },
                    {
                        "statusId": 14,
                        "status": "+4 Laps",
                        "count": 405
                    },
                    {
                        "statusId": 15,
                        "status": "+5 Laps",
                        "count": 221
                    },
                    {
                        "statusId": 16,
                        "status": "+6 Laps",
                        "count": 153
                    },
                    {
                        "statusId": 17,
                        "status": "+7 Laps",
                        "count": 99
                    },
                    {
                        "statusId": 18,
                        "status": "+8 Laps",
                        "count": 52
                    },
                    {
                        "statusId": 19,
                        "status": "+9 Laps",
                        "count": 38
                    },
                    {
                        "statusId": 20,
                        "status": "Spun off",
                        "count": 792
                    },
                    {
                        "statusId": 21,
                        "status": "Radiator",
                        "count": 43
                    },
                    {
                        "statusId": 22,
                        "status": "Suspension",
                        "count": 431
                    },
                    {
                        "statusId": 23,
                        "status": "Brakes",
                        "count": 253
                    },
                    {
                        "statusId": 24,
                        "status": "Differential",
                        "count": 61
                    },
                    {
                        "statusId": 25,
                        "status": "Overheating",
                        "count": 132
                    },
                    {
                        "statusId": 26,
                        "status": "Mechanical",
                        "count": 29
                    },
                    {
                        "statusId": 27,
                        "status": "Tyre",
                        "count": 55
                    },
                    {
                        "statusId": 28,
                        "status": "Driver Seat",
                        "count": 1
                    },
                    {
                        "statusId": 29,
                        "status": "Puncture",
                        "count": 41
                    },
                    {
                        "statusId": 30,
                        "status": "Driveshaft",
                        "count": 25
                    }
                ]
    });

    Test::new(
        r#"{
            status(options: null) {
                statusId
                status
                count
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_status_by_driver_ref() {
    let value: serde_json::Value = json!({
        "status": [
                    {
                        "statusId": 1,
                        "status": "Finished",
                        "count": 90
                    },
                    {
                        "statusId": 2,
                        "status": "Disqualified",
                        "count": 1
                    },
                    {
                        "statusId": 3,
                        "status": "Accident",
                        "count": 4
                    },
                    {
                        "statusId": 4,
                        "status": "Collision",
                        "count": 6
                    },
                    {
                        "statusId": 5,
                        "status": "Engine",
                        "count": 2
                    },
                    {
                        "statusId": 11,
                        "status": "+1 Lap",
                        "count": 14
                    },
                    {
                        "statusId": 12,
                        "status": "+2 Laps",
                        "count": 2
                    },
                    {
                        "statusId": 22,
                        "status": "Suspension",
                        "count": 1
                    },
                    {
                        "statusId": 26,
                        "status": "Mechanical",
                        "count": 1
                    },
                    {
                        "statusId": 27,
                        "status": "Tyre",
                        "count": 1
                    },
                    {
                        "statusId": 30,
                        "status": "Driveshaft",
                        "count": 1
                    },
                    {
                        "statusId": 40,
                        "status": "Electronics",
                        "count": 1
                    },
                    {
                        "statusId": 101,
                        "status": "Turbo",
                        "count": 1
                    },
                    {
                        "statusId": 130,
                        "status": "Collision damage",
                        "count": 2
                    },
                    {
                        "statusId": 131,
                        "status": "Power Unit",
                        "count": 1
                    },
                    {
                        "statusId": 140,
                        "status": "Undertray",
                        "count": 1
                    }
                ]
    });

    Test::new(
        r#"{
            status(options: { driverRef: "leclerc" }) {
                statusId
                status
                count
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_status_by_constructor_ref() {
    let value: serde_json::Value = json!({
        "status": [
                    {
                        "statusId": 1,
                        "status": "Finished",
                        "count": 1287
                    },
                    {
                        "statusId": 2,
                        "status": "Disqualified",
                        "count": 9
                    },
                    {
                        "statusId": 3,
                        "status": "Accident",
                        "count": 95
                    },
                    {
                        "statusId": 4,
                        "status": "Collision",
                        "count": 62
                    },
                    {
                        "statusId": 5,
                        "status": "Engine",
                        "count": 165
                    },
                    {
                        "statusId": 6,
                        "status": "Gearbox",
                        "count": 45
                    },
                    {
                        "statusId": 7,
                        "status": "Transmission",
                        "count": 27
                    },
                    {
                        "statusId": 8,
                        "status": "Clutch",
                        "count": 17
                    },
                    {
                        "statusId": 9,
                        "status": "Hydraulics",
                        "count": 8
                    },
                    {
                        "statusId": 10,
                        "status": "Electrical",
                        "count": 21
                    },
                    {
                        "statusId": 11,
                        "status": "+1 Lap",
                        "count": 230
                    },
                    {
                        "statusId": 12,
                        "status": "+2 Laps",
                        "count": 62
                    },
                    {
                        "statusId": 13,
                        "status": "+3 Laps",
                        "count": 30
                    },
                    {
                        "statusId": 14,
                        "status": "+4 Laps",
                        "count": 17
                    },
                    {
                        "statusId": 15,
                        "status": "+5 Laps",
                        "count": 16
                    },
                    {
                        "statusId": 16,
                        "status": "+6 Laps",
                        "count": 4
                    },
                    {
                        "statusId": 17,
                        "status": "+7 Laps",
                        "count": 4
                    },
                    {
                        "statusId": 18,
                        "status": "+8 Laps",
                        "count": 2
                    },
                    {
                        "statusId": 19,
                        "status": "+9 Laps",
                        "count": 3
                    },
                    {
                        "statusId": 20,
                        "status": "Spun off",
                        "count": 39
                    },
                    {
                        "statusId": 21,
                        "status": "Radiator",
                        "count": 2
                    },
                    {
                        "statusId": 22,
                        "status": "Suspension",
                        "count": 30
                    },
                    {
                        "statusId": 23,
                        "status": "Brakes",
                        "count": 11
                    },
                    {
                        "statusId": 24,
                        "status": "Differential",
                        "count": 4
                    },
                    {
                        "statusId": 25,
                        "status": "Overheating",
                        "count": 2
                    },
                    {
                        "statusId": 26,
                        "status": "Mechanical",
                        "count": 1
                    },
                    {
                        "statusId": 27,
                        "status": "Tyre",
                        "count": 8
                    },
                    {
                        "statusId": 29,
                        "status": "Puncture",
                        "count": 4
                    },
                    {
                        "statusId": 30,
                        "status": "Driveshaft",
                        "count": 1
                    },
                    {
                        "statusId": 31,
                        "status": "Retired",
                        "count": 3
                    }
                ]
    });

    Test::new(
        r#"{
            status(options: { constructorRef: "ferrari" }) {
                statusId
                status
                count
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}
