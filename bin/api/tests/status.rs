use shared::prelude::*;

pub mod common;

use common::models::StaticStatus;

#[test]
fn test_get_status() {
    common::Test::<&[StaticStatus], Vec<Status>>::new("/api/f1/status", Series::F1, &ALL_STATUS)
        .pagination(Some(Pagination {
            limit: 30,
            page: 1,
            max_page: 5,
            total: 137,
        }))
        .test_ok();
}

#[test]
fn test_get_status_by_driver_ref() {
    common::Test::<&[StaticStatus], Vec<Status>>::new(
        "/api/f1/status?driver_ref=leclerc",
        Series::F1,
        &LECLERC_STATUS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 1,
        total: 16,
    }))
    .test_ok();
}

#[test]
fn test_get_status_by_constructor_ref() {
    common::Test::<&[StaticStatus], Vec<Status>>::new(
        "/api/f1/status?constructor_ref=ferrari",
        Series::F1,
        &FERRARI_STATUS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 3,
        total: 75,
    }))
    .test_ok();
}

const FERRARI_STATUS: [StaticStatus; 30] = status_from_json![
    {
        "status_id": 1,
        "status": "Finished",
        "count": 1287
    },
    {
        "status_id": 2,
        "status": "Disqualified",
        "count": 9
    },
    {
        "status_id": 3,
        "status": "Accident",
        "count": 95
    },
    {
        "status_id": 4,
        "status": "Collision",
        "count": 62
    },
    {
        "status_id": 5,
        "status": "Engine",
        "count": 165
    },
    {
        "status_id": 6,
        "status": "Gearbox",
        "count": 45
    },
    {
        "status_id": 7,
        "status": "Transmission",
        "count": 27
    },
    {
        "status_id": 8,
        "status": "Clutch",
        "count": 17
    },
    {
        "status_id": 9,
        "status": "Hydraulics",
        "count": 8
    },
    {
        "status_id": 10,
        "status": "Electrical",
        "count": 21
    },
    {
        "status_id": 11,
        "status": "+1 Lap",
        "count": 230
    },
    {
        "status_id": 12,
        "status": "+2 Laps",
        "count": 62
    },
    {
        "status_id": 13,
        "status": "+3 Laps",
        "count": 30
    },
    {
        "status_id": 14,
        "status": "+4 Laps",
        "count": 17
    },
    {
        "status_id": 15,
        "status": "+5 Laps",
        "count": 16
    },
    {
        "status_id": 16,
        "status": "+6 Laps",
        "count": 4
    },
    {
        "status_id": 17,
        "status": "+7 Laps",
        "count": 4
    },
    {
        "status_id": 18,
        "status": "+8 Laps",
        "count": 2
    },
    {
        "status_id": 19,
        "status": "+9 Laps",
        "count": 3
    },
    {
        "status_id": 20,
        "status": "Spun off",
        "count": 39
    },
    {
        "status_id": 21,
        "status": "Radiator",
        "count": 2
    },
    {
        "status_id": 22,
        "status": "Suspension",
        "count": 30
    },
    {
        "status_id": 23,
        "status": "Brakes",
        "count": 11
    },
    {
        "status_id": 24,
        "status": "Differential",
        "count": 4
    },
    {
        "status_id": 25,
        "status": "Overheating",
        "count": 2
    },
    {
        "status_id": 26,
        "status": "Mechanical",
        "count": 1
    },
    {
        "status_id": 27,
        "status": "Tyre",
        "count": 8
    },
    {
        "status_id": 29,
        "status": "Puncture",
        "count": 4
    },
    {
        "status_id": 30,
        "status": "Driveshaft",
        "count": 1
    },
    {
        "status_id": 31,
        "status": "Retired",
        "count": 3
    }
];

const LECLERC_STATUS: [StaticStatus; 16] = status_from_json![
    {
        "status_id": 1,
        "status": "Finished",
        "count": 90
    },
    {
        "status_id": 2,
        "status": "Disqualified",
        "count": 1
    },
    {
        "status_id": 3,
        "status": "Accident",
        "count": 4
    },
    {
        "status_id": 4,
        "status": "Collision",
        "count": 6
    },
    {
        "status_id": 5,
        "status": "Engine",
        "count": 2
    },
    {
        "status_id": 11,
        "status": "+1 Lap",
        "count": 14
    },
    {
        "status_id": 12,
        "status": "+2 Laps",
        "count": 2
    },
    {
        "status_id": 22,
        "status": "Suspension",
        "count": 1
    },
    {
        "status_id": 26,
        "status": "Mechanical",
        "count": 1
    },
    {
        "status_id": 27,
        "status": "Tyre",
        "count": 1
    },
    {
        "status_id": 30,
        "status": "Driveshaft",
        "count": 1
    },
    {
        "status_id": 40,
        "status": "Electronics",
        "count": 1
    },
    {
        "status_id": 101,
        "status": "Turbo",
        "count": 1
    },
    {
        "status_id": 130,
        "status": "Collision damage",
        "count": 2
    },
    {
        "status_id": 131,
        "status": "Power Unit",
        "count": 1
    },
    {
        "status_id": 140,
        "status": "Undertray",
        "count": 1
    }
];

const ALL_STATUS: [StaticStatus; 30] = status_from_json![
    {
        "status_id": 1,
        "status": "Finished",
        "count": 7427
    },
    {
        "status_id": 2,
        "status": "Disqualified",
        "count": 145
    },
    {
        "status_id": 3,
        "status": "Accident",
        "count": 1057
    },
    {
        "status_id": 4,
        "status": "Collision",
        "count": 842
    },
    {
        "status_id": 5,
        "status": "Engine",
        "count": 2022
    },
    {
        "status_id": 6,
        "status": "Gearbox",
        "count": 809
    },
    {
        "status_id": 7,
        "status": "Transmission",
        "count": 321
    },
    {
        "status_id": 8,
        "status": "Clutch",
        "count": 214
    },
    {
        "status_id": 9,
        "status": "Hydraulics",
        "count": 138
    },
    {
        "status_id": 10,
        "status": "Electrical",
        "count": 316
    },
    {
        "status_id": 11,
        "status": "+1 Lap",
        "count": 3944
    },
    {
        "status_id": 12,
        "status": "+2 Laps",
        "count": 1600
    },
    {
        "status_id": 13,
        "status": "+3 Laps",
        "count": 731
    },
    {
        "status_id": 14,
        "status": "+4 Laps",
        "count": 405
    },
    {
        "status_id": 15,
        "status": "+5 Laps",
        "count": 221
    },
    {
        "status_id": 16,
        "status": "+6 Laps",
        "count": 153
    },
    {
        "status_id": 17,
        "status": "+7 Laps",
        "count": 99
    },
    {
        "status_id": 18,
        "status": "+8 Laps",
        "count": 52
    },
    {
        "status_id": 19,
        "status": "+9 Laps",
        "count": 38
    },
    {
        "status_id": 20,
        "status": "Spun off",
        "count": 792
    },
    {
        "status_id": 21,
        "status": "Radiator",
        "count": 43
    },
    {
        "status_id": 22,
        "status": "Suspension",
        "count": 431
    },
    {
        "status_id": 23,
        "status": "Brakes",
        "count": 253
    },
    {
        "status_id": 24,
        "status": "Differential",
        "count": 61
    },
    {
        "status_id": 25,
        "status": "Overheating",
        "count": 132
    },
    {
        "status_id": 26,
        "status": "Mechanical",
        "count": 29
    },
    {
        "status_id": 27,
        "status": "Tyre",
        "count": 55
    },
    {
        "status_id": 28,
        "status": "Driver Seat",
        "count": 1
    },
    {
        "status_id": 29,
        "status": "Puncture",
        "count": 41
    },
    {
        "status_id": 30,
        "status": "Driveshaft",
        "count": 25
    }
];
