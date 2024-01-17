// @generated automatically by Diesel CLI.
#![allow(non_snake_case)]

diesel::table! {
    circuits (circuit_id) {
        #[sql_name = "circuitId"]
        circuit_id -> Integer,
        #[max_length = 255]
        #[sql_name = "circuitRef"]
        circuit_ref -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        location -> Nullable<Varchar>,
        #[max_length = 255]
        country -> Nullable<Varchar>,
        lat -> Nullable<Float>,
        lng -> Nullable<Float>,
        alt -> Nullable<Integer>,
        #[max_length = 255]
        url -> Varchar,
    }
}

diesel::table! {
    constructorResults (constructor_results_id) {
        #[sql_name = "constructorResultsId"]
        constructor_results_id -> Integer,
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "constructorId"]
        constructor_id -> Integer,
        points -> Nullable<Float>,
        #[max_length = 255]
        status -> Nullable<Varchar>,
    }
}

diesel::table! {
    constructorStandings (constructor_standing_id) {
        #[sql_name = "constructorStandingsId"]
        constructor_standing_id -> Integer,
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "constructorId"]
        constructor_id -> Integer,
        points -> Float,
        position -> Nullable<Integer>,
        #[max_length = 255]
        #[sql_name = "positionText"]
        position_text -> Nullable<Varchar>,
        wins -> Integer,
    }
}

diesel::table! {
    constructors (constructor_id) {
        #[sql_name = "constructorId"]
        constructor_id -> Integer,
        #[max_length = 255]
        #[sql_name = "constructorRef"]
        constructor_ref -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        nationality -> Nullable<Varchar>,
        #[max_length = 255]
        url -> Varchar,
    }
}

diesel::table! {
    driverStandings (driver_standing_id) {
        #[sql_name = "driverStandingsId"]
        driver_standing_id -> Integer,
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "driverId"]
        driver_id -> Integer,
        points -> Float,
        position -> Nullable<Integer>,
        #[max_length = 255]
        #[sql_name = "positionText"]
        position_text -> Nullable<Varchar>,
        wins -> Integer,
    }
}

diesel::table! {
    drivers (driver_id) {
        #[sql_name = "driverId"]
        driver_id -> Integer,
        #[max_length = 255]
        #[sql_name = "driverRef"]
        driver_ref -> Varchar,
        number -> Nullable<Integer>,
        #[max_length = 3]
        code -> Nullable<Varchar>,
        #[max_length = 255]
        forename -> Varchar,
        #[max_length = 255]
        surname -> Varchar,
        dob -> Nullable<Date>,
        #[max_length = 255]
        nationality -> Nullable<Varchar>,
        #[max_length = 255]
        url -> Varchar,
    }
}

diesel::table! {
    lapTimes (race_id, driver_id, lap) {
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "driverId"]
        driver_id -> Integer,
        lap -> Integer,
        position -> Nullable<Integer>,
        #[max_length = 255]
        time -> Nullable<Varchar>,
        milliseconds -> Nullable<Integer>,
    }
}

diesel::table! {
    pitStops (race_id, driver_id, stop) {
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "driverId"]
        driver_id -> Integer,
        stop -> Integer,
        lap -> Integer,
        time -> Time,
        #[max_length = 255]
        duration -> Nullable<Varchar>,
        milliseconds -> Nullable<Integer>,
    }
}

diesel::table! {
    qualifying (qualifying_id) {
        #[sql_name = "qualifyId"]
        qualifying_id -> Integer,
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "driverId"]
        driver_id -> Integer,
        #[sql_name = "constructorId"]
        constructor_id -> Integer,
        number -> Integer,
        position -> Nullable<Integer>,
        #[max_length = 255]
        q1 -> Nullable<Varchar>,
        #[max_length = 255]
        q2 -> Nullable<Varchar>,
        #[max_length = 255]
        q3 -> Nullable<Varchar>,
    }
}

diesel::table! {
    races (race_id) {
        #[sql_name = "raceId"]
        race_id -> Integer,
        year -> Integer,
        round -> Integer,
        #[sql_name = "circuitId"]
        circuit_id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        date -> Date,
        time -> Nullable<Time>,
        #[max_length = 255]
        url -> Nullable<Varchar>,
        fp1_date -> Nullable<Date>,
        fp1_time -> Nullable<Time>,
        fp2_date -> Nullable<Date>,
        fp2_time -> Nullable<Time>,
        fp3_date -> Nullable<Date>,
        fp3_time -> Nullable<Time>,
        #[sql_name = "quali_date"]
        qualifying_date -> Nullable<Date>,
        #[sql_name = "quali_time"]
        qualifying_time -> Nullable<Time>,
        sprint_date -> Nullable<Date>,
        sprint_time -> Nullable<Time>,
    }
}

diesel::table! {
    results (result_id) {
        #[sql_name = "resultId"]
        result_id -> Integer,
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "driverId"]
        driver_id -> Integer,
        #[sql_name = "constructorId"]
        constructor_id -> Integer,
        number -> Nullable<Integer>,
        grid -> Integer,
        position -> Nullable<Integer>,
        #[max_length = 255]
        #[sql_name = "positionText"]
        position_text -> Varchar,
        #[sql_name = "positionOrder"]
        position_order -> Integer,
        points -> Float,
        laps -> Integer,
        #[max_length = 255]
        time -> Nullable<Varchar>,
        milliseconds -> Nullable<Integer>,
        #[sql_name = "fastestLap"]
        fastest_lap -> Nullable<Integer>,
        rank -> Nullable<Integer>,
        #[max_length = 255]
        #[sql_name = "fastestLapTime"]
        fastest_lap_time -> Nullable<Varchar>,
        #[max_length = 255]
        #[sql_name = "fastestLapSpeed"]
        fastest_lap_speed -> Nullable<Varchar>,
        #[sql_name = "statusId"]
        status_id -> Integer,
    }
}

diesel::table! {
    seasons (year) {
        year -> Integer,
        #[max_length = 255]
        url -> Varchar,
    }
}

diesel::table! {
    sprintResults (sprint_result_id) {
        #[sql_name = "sprintResultId"]
        sprint_result_id -> Integer,
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "driverId"]
        driver_id -> Integer,
        #[sql_name = "constructorId"]
        constructor_id -> Integer,
        number -> Nullable<Integer>,
        grid -> Integer,
        position -> Nullable<Integer>,
        #[max_length = 255]
        #[sql_name = "positionText"]
        position_text -> Varchar,
        #[sql_name = "positionOrder"]
        position_order -> Integer,
        points -> Float,
        laps -> Integer,
        #[max_length = 255]
        time -> Nullable<Varchar>,
        milliseconds -> Nullable<Integer>,
        #[sql_name = "fastestLap"]
        fastest_lap -> Nullable<Integer>,
        rank -> Nullable<Integer>,
        #[max_length = 255]
        #[sql_name = "fastestLapTime"]
        fastest_lap_time -> Nullable<Varchar>,
        #[max_length = 255]
        #[sql_name = "fastestLapSpeed"]
        fastest_lap_speed -> Nullable<Varchar>,
        #[sql_name = "statusId"]
        status_id -> Integer,
    }
}

diesel::table! {
    status (status_id) {
        #[sql_name = "statusId"]
        status_id -> Integer,
        #[max_length = 255]
        #[sql_name = "status"]
        status_content -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    circuits,
    constructorResults,
    constructorStandings,
    constructors,
    driverStandings,
    drivers,
    lapTimes,
    pitStops,
    qualifying,
    races,
    results,
    seasons,
    sprintResults,
    status,
);
