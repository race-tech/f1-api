// @generated automatically by Diesel CLI.
#![allow(non_snake_case)]

diesel::table! {
    circuits (circuitId) {
        circuitId -> Integer,
        #[max_length = 255]
        circuitRef -> Varchar,
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
    constructorResults (constructorResultsId) {
        constructorResultsId -> Integer,
        raceId -> Integer,
        constructorId -> Integer,
        points -> Nullable<Float>,
        #[max_length = 255]
        status -> Nullable<Varchar>,
    }
}

diesel::table! {
    constructorStandings (constructorStandingsId) {
        constructorStandingsId -> Integer,
        raceId -> Integer,
        constructorId -> Integer,
        points -> Float,
        position -> Nullable<Integer>,
        #[max_length = 255]
        positionText -> Nullable<Varchar>,
        wins -> Integer,
    }
}

diesel::table! {
    constructors (constructorId) {
        constructorId -> Integer,
        #[max_length = 255]
        constructorRef -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        nationality -> Nullable<Varchar>,
        #[max_length = 255]
        url -> Varchar,
    }
}

diesel::table! {
    driverStandings (driverStandingsId) {
        driverStandingsId -> Integer,
        raceId -> Integer,
        driverId -> Integer,
        points -> Float,
        position -> Nullable<Integer>,
        #[max_length = 255]
        positionText -> Nullable<Varchar>,
        wins -> Integer,
    }
}

diesel::table! {
    drivers (driverId) {
        driverId -> Integer,
        #[max_length = 255]
        driverRef -> Varchar,
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
    lapTimes (raceId, driverId, lap) {
        raceId -> Integer,
        driverId -> Integer,
        lap -> Integer,
        position -> Nullable<Integer>,
        #[max_length = 255]
        time -> Nullable<Varchar>,
        milliseconds -> Nullable<Integer>,
    }
}

diesel::table! {
    pitStops (raceId, driverId, stop) {
        raceId -> Integer,
        driverId -> Integer,
        stop -> Integer,
        lap -> Integer,
        time -> Time,
        #[max_length = 255]
        duration -> Nullable<Varchar>,
        milliseconds -> Nullable<Integer>,
    }
}

diesel::table! {
    qualifying (qualifyId) {
        qualifyId -> Integer,
        raceId -> Integer,
        driverId -> Integer,
        constructorId -> Integer,
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
    races (raceId) {
        raceId -> Integer,
        year -> Integer,
        round -> Integer,
        circuitId -> Integer,
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
        quali_date -> Nullable<Date>,
        quali_time -> Nullable<Time>,
        sprint_date -> Nullable<Date>,
        sprint_time -> Nullable<Time>,
    }
}

diesel::table! {
    results (resultId) {
        resultId -> Integer,
        raceId -> Integer,
        driverId -> Integer,
        constructorId -> Integer,
        number -> Nullable<Integer>,
        grid -> Integer,
        position -> Nullable<Integer>,
        #[max_length = 255]
        positionText -> Varchar,
        positionOrder -> Integer,
        points -> Float,
        laps -> Integer,
        #[max_length = 255]
        time -> Nullable<Varchar>,
        milliseconds -> Nullable<Integer>,
        fastestLap -> Nullable<Integer>,
        rank -> Nullable<Integer>,
        #[max_length = 255]
        fastestLapTime -> Nullable<Varchar>,
        #[max_length = 255]
        fastestLapSpeed -> Nullable<Varchar>,
        statusId -> Integer,
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
    sprintResults (sprintResultId) {
        sprintResultId -> Integer,
        raceId -> Integer,
        driverId -> Integer,
        constructorId -> Integer,
        number -> Integer,
        grid -> Integer,
        position -> Nullable<Integer>,
        #[max_length = 255]
        positionText -> Varchar,
        positionOrder -> Integer,
        points -> Float,
        laps -> Integer,
        #[max_length = 255]
        time -> Nullable<Varchar>,
        milliseconds -> Nullable<Integer>,
        fastestLap -> Nullable<Integer>,
        #[max_length = 255]
        fastestLapTime -> Nullable<Varchar>,
        statusId -> Integer,
    }
}

diesel::table! {
    status (statusId) {
        statusId -> Integer,
        #[max_length = 255]
        #[sql_name = "status"]
        status_type -> Varchar,
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
