// @generated automatically by Diesel CLI.
#![allow(non_snake_case)]

diesel::table! {
    circuits (circuit_id) {
        #[sql_name = "circuitId"]
        circuit_id -> Integer,
        #[sql_name = "circuitRef"]
        #[max_length = 255]
        circuit_ref -> Varchar,
        #[sql_name = "circuitName"]
        #[max_length = 255]
        circuit_name -> Varchar,
        #[sql_name = "circuitLocation"]
        #[max_length = 255]
        circuit_location -> Nullable<Varchar>,
        #[sql_name = "circuitCountry"]
        #[max_length = 255]
        country -> Nullable<Varchar>,
        #[sql_name = "circuitLat"]
        lat -> Nullable<Float>,
        #[sql_name = "circuitLng"]
        lng -> Nullable<Float>,
        #[sql_name = "circuitAlt"]
        alt -> Nullable<Integer>,
        #[sql_name = "circuitUrl"]
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
        #[sql_name = "constructorResultsPoints"]
        points -> Nullable<Float>,
        #[sql_name = "constructorResultsStatus"]
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
        #[sql_name = "constructorStandingsPoints"]
        points -> Float,
        #[sql_name = "constructorStandingsPosition"]
        position -> Nullable<Integer>,
        #[sql_name = "constructorStandingsPositionText"]
        #[max_length = 255]
        position_text -> Nullable<Varchar>,
        #[sql_name = "constructorStandingsWins"]
        wins -> Integer,
    }
}

diesel::table! {
    constructors (constructor_id) {
        #[sql_name = "constructorId"]
        constructor_id -> Integer,
        #[sql_name = "constructorRef"]
        #[max_length = 255]
        constructor_ref -> Varchar,
        #[sql_name = "constructorName"]
        #[max_length = 255]
        name -> Varchar,
        #[sql_name = "constructorNationality"]
        #[max_length = 255]
        nationality -> Nullable<Varchar>,
        #[sql_name = "constructorUrl"]
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
        #[sql_name = "driverStandingsPoints"]
        points -> Float,
        #[sql_name = "driverStandingsPosition"]
        position -> Nullable<Integer>,
        #[sql_name = "driverStandingsPositionText"]
        #[max_length = 255]
        position_text -> Nullable<Varchar>,
        #[sql_name = "driverStandingsWins"]
        wins -> Integer,
    }
}

diesel::table! {
    drivers (driver_id) {
        #[sql_name = "driverId"]
        driver_id -> Integer,
        #[sql_name = "driverRef"]
        #[max_length = 255]
        driver_ref -> Varchar,
        #[sql_name = "driverNumber"]
        number -> Nullable<Integer>,
        #[sql_name = "driverCode"]
        #[max_length = 3]
        code -> Nullable<Varchar>,
        #[sql_name = "driverForename"]
        #[max_length = 255]
        forename -> Varchar,
        #[sql_name = "driverSurname"]
        #[max_length = 255]
        surname -> Varchar,
        #[sql_name = "driverDob"]
        dob -> Nullable<Date>,
        #[sql_name = "driverNationality"]
        #[max_length = 255]
        nationality -> Nullable<Varchar>,
        #[sql_name = "driverUrl"]
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
        #[sql_name = "lapTimesLap"]
        lap -> Integer,
        #[sql_name = "lapTimesPosition"]
        position -> Nullable<Integer>,
        #[sql_name = "lapTimesTime"]
        #[max_length = 255]
        time -> Nullable<Varchar>,
        #[sql_name = "lapTimesMilliseconds"]
        milliseconds -> Nullable<Integer>,
    }
}

diesel::table! {
    pitStops (race_id, driver_id, lap) {
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "driverId"]
        driver_id -> Integer,
        stop -> Integer,
        #[sql_name = "pitStopsLap"]
        lap -> Integer,
        #[sql_name = "pitStopsTime"]
        time -> Time,
        #[sql_name = "pitStopsDuration"]
        #[max_length = 255]
        duration -> Nullable<Varchar>,
        #[sql_name = "pitStopsMilliseconds"]
        milliseconds -> Nullable<Integer>,
    }
}

diesel::table! {
    qualifying (qualify_id) {
        #[sql_name = "qualifyId"]
        qualify_id -> Integer,
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "driverId"]
        driver_id -> Integer,
        #[sql_name = "constructorId"]
        constructor_id -> Integer,
        #[sql_name = "qualifyingNumber"]
        number -> Integer,
        #[sql_name = "qualifyingPosition"]
        position -> Nullable<Integer>,
        #[sql_name = "qualifyingQ1"]
        #[max_length = 255]
        q1 -> Nullable<Varchar>,
        #[sql_name = "qualifyingQ2"]
        #[max_length = 255]
        q2 -> Nullable<Varchar>,
        #[sql_name = "qualifyingQ3"]
        #[max_length = 255]
        q3 -> Nullable<Varchar>,
    }
}

diesel::table! {
    races (race_id) {
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "raceYear"]
        year -> Integer,
        #[sql_name = "raceRound"]
        round -> Integer,
        #[sql_name = "circuitId"]
        circuit_id -> Integer,
        #[sql_name = "raceName"]
        #[max_length = 255]
        name -> Varchar,
        #[sql_name = "raceDate"]
        date -> Date,
        #[sql_name = "raceTime"]
        time -> Nullable<Time>,
        #[sql_name = "raceUrl"]
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
        #[sql_name = "resultNumber"]
        number -> Integer,
        #[sql_name = "resultGrid"]
        grid -> Integer,
        #[sql_name = "resultPosition"]
        position -> Nullable<Integer>,
        #[sql_name = "resultPositionText"]
        #[max_length = 255]
        position_text -> Varchar,
        #[sql_name = "resultPositionOrder"]
        position_order -> Integer,
        #[sql_name = "resultPoints"]
        points -> Float,
        #[sql_name = "resultLaps"]
        laps -> Integer,
        #[sql_name = "resultTime"]
        #[max_length = 255]
        time -> Nullable<Varchar>,
        #[sql_name = "resultMilliseconds"]
        milliseconds -> Nullable<Integer>,
        #[sql_name = "fastestLap"]
        fastest_lap -> Nullable<Integer>,
        rank -> Nullable<Integer>,
        #[sql_name = "resultFastestLapTime"]
        #[max_length = 255]
        fastest_lap_time -> Nullable<Varchar>,
        #[sql_name = "resultFastestLapSpeed"]
        #[max_length = 255]
        fastest_lap_speed -> Nullable<Varchar>,
        #[sql_name = "resultStatusId"]
        status_id -> Integer,
    }
}

diesel::table! {
    seasons (year) {
        #[sql_name = "seasonYear"]
        year -> Integer,
        #[sql_name = "seasonUrl"]
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
        #[sql_name = "sprintResultNumber"]
        number -> Integer,
        #[sql_name = "sprintResultGrid"]
        grid -> Integer,
        #[sql_name = "sprintResultPosition"]
        position -> Nullable<Integer>,
        #[sql_name = "sprintResultPositionText"]
        #[max_length = 255]
        position_text -> Varchar,
        #[sql_name = "sprintResultPositionOrder"]
        position_order -> Integer,
        #[sql_name = "sprintResultPoints"]
        points -> Float,
        #[sql_name = "sprintResultLaps"]
        laps -> Integer,
        #[sql_name = "sprintResultTime"]
        #[max_length = 255]
        time -> Nullable<Varchar>,
        #[sql_name = "sprintResultMilliseconds"]
        milliseconds -> Nullable<Integer>,
        #[sql_name = "sprintResultFastestLap"]
        fastest_lap -> Nullable<Integer>,
        #[sql_name = "sprintResultFastestLapTime"]
        #[max_length = 255]
        fastest_lap_time -> Nullable<Varchar>,
        #[sql_name = "sprintResultStatusId"]
        status_id -> Integer,
    }
}

diesel::table! {
    status (status_id) {
        #[sql_name = "statusId"]
        status_id -> Integer,
        #[sql_name = "status"]
        #[max_length = 255]
        status_content -> Varchar,
    }
}

use diesel::joinable;

joinable!(results -> drivers (driver_id));
joinable!(results -> constructors (constructor_id));
joinable!(results -> races (race_id));
joinable!(races -> circuits (circuit_id));
joinable!(driverStandings -> drivers (driver_id));
joinable!(driverStandings -> races (race_id));

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
