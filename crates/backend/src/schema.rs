diesel::table! {
    circuits (circuit_id) {
        #[sql_name = "circuitId"]
        circuit_id -> Integer,
        #[sql_name = "circuitRef"]
        circuit_ref -> VarChar,
        name -> VarChar,
        location -> Nullable<VarChar>,
        country -> Nullable<VarChar>,
        lat -> Nullable<Float>,
        lng -> Nullable<Float>,
        alt -> Nullable<Integer>,
        url -> VarChar
    }
}

diesel::table! {
    constructor_results (constructor_results_id) {
        #[sql_name = "constructorResultsId"]
        constructor_results_id -> Integer,
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "constructorId"]
        constructor_id -> Integer,
        points -> Nullable<Float>,
        status -> Nullable<VarChar>
    }
}

diesel::table! {
    constructor_standings (constructor_standings_id) {
        #[sql_name = "constructorStandingsId"]
        constructor_standings_id -> Integer,
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "constructorId"]
        constructor_id -> Integer,
        points -> Float,
        position -> Nullable<Integer>,
        #[sql_name = "positionText"]
        position_text -> Nullable<VarChar>,
        wins -> Integer
    }
}

diesel::table! {
    constructors (constructor_id) {
        #[sql_name = "constructorId"]
        constructor_id -> Integer,
        #[sql_name = "constructorRef"]
        constructor_ref -> VarChar,
        name -> VarChar,
        nationality -> Nullable<VarChar>,
        url -> VarChar
    }
}

diesel::table! {
    driver_standings (driver_standings_id) {
        #[sql_name = "driverStandingsId"]
        driver_standings_id -> Integer,
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "driverId"]
        driver_id -> Integer,
        points -> Float,
        position -> Nullable<Integer>,
        #[sql_name = "positionText"]
        position_text -> Nullable<VarChar>,
        wins -> Integer
    }
}

diesel::table! {
    drivers (driver_id) {
        #[sql_name = "driverId"]
        driver_id -> Integer,
        #[sql_name = "driverRef"]
        driver_ref -> VarChar,
        number -> Nullable<Integer>,
        code -> Nullable<VarChar>,
        forename -> VarChar,
        surname -> VarChar,
        dob -> Nullable<Date>,
        nationality -> Nullable<VarChar>,
        url -> VarChar
    }
}

diesel::table! {
    lap_times (race_id, driver_id, lap) {
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "driverId"]
        driver_id -> Integer,
        lap -> Integer,
        position -> Nullable<Integer>,
        time -> Nullable<VarChar>,
        milliseconds -> Nullable<Integer>
    }
}

diesel::table! {
    pit_stops (race_id, driver_id, stop) {
        #[sql_name = "raceId"]
        race_id -> Integer,
        #[sql_name = "driverId"]
        driver_id -> Integer,
        stop -> Integer,
        lap -> Integer,
        time -> Time,
        duration -> Nullable<VarChar>,
        milliseconds -> Nullable<Integer>
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
        number -> Integer,
        position -> Nullable<Integer>,
        q1 -> Nullable<VarChar>,
        q2 -> Nullable<VarChar>,
        q3 -> Nullable<VarChar>
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
        name -> VarChar,
        date -> Date,
        time -> Nullable<Time>,
        url -> Nullable<VarChar>,
        fp1_date -> Nullable<Date>,
        fp1_time -> Nullable<Time>,
        fp2_date -> Nullable<Date>,
        fp2_time -> Nullable<Time>,
        fp3_date -> Nullable<Date>,
        fp3_time -> Nullable<Time>,
        quali_date -> Nullable<Date>,
        quali_time -> Nullable<Time>,
        sprint_date -> Nullable<Date>,
        sprint_time -> Nullable<Time>
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
        #[sql_name = "positionText"]
        position_text -> VarChar,
        #[sql_name = "positionOrder"]
        position_order -> Integer,
        points -> Float,
        laps -> Integer,
        time -> Nullable<VarChar>,
        milliseconds -> Nullable<Integer>,
        #[sql_name = "fastestLap"]
        fastest_lap -> Nullable<Integer>,
        rank -> Nullable<Integer>,
        #[sql_name = "fastestLapTime"]
        fastest_lap_time -> Nullable<VarChar>,
        #[sql_name = "fastestLapSpeed"]
        fastest_lap_speed -> Nullable<VarChar>,
        #[sql_name = "statusId"]
        status_id -> Integer
    }
}

diesel::table! {
    sprint_results (sprint_result_id) {
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
        #[sql_name = "positionText"]
        position_text -> VarChar,
        #[sql_name = "positionOrder"]
        position_order -> Integer,
        points -> Float,
        laps -> Integer,
        time -> Nullable<VarChar>,
        milliseconds -> Nullable<Integer>,
        #[sql_name = "fastestLap"]
        fastest_lap -> Nullable<Integer>,
        #[sql_name = "fastestLapTime"]
        fastest_lap_time -> Nullable<VarChar>,
        #[sql_name = "statusId"]
        status_id -> Integer
    }
}

diesel::table! {
    seasons (year) {
        year -> Integer,
        url -> VarChar
    }
}

diesel::table! {
    status (status_id) {
        #[sql_name = "statusId"]
        status_id -> Integer,
        #[sql_name = "status"]
        status_content -> VarChar
    }
}

diesel::joinable!(constructor_results -> races (race_id));
diesel::joinable!(constructor_results -> constructors (constructor_id));

diesel::joinable!(constructor_standings -> races (race_id));
diesel::joinable!(constructor_standings -> constructors (constructor_id));

diesel::joinable!(driver_standings -> races (race_id));
diesel::joinable!(driver_standings -> drivers (driver_id));

diesel::joinable!(lap_times -> drivers (driver_id));
diesel::joinable!(lap_times -> races (race_id));

diesel::joinable!(pit_stops -> drivers (driver_id));
diesel::joinable!(pit_stops -> races (race_id));

diesel::joinable!(qualifying -> races (race_id));
diesel::joinable!(qualifying -> drivers (driver_id));
diesel::joinable!(qualifying -> constructors (constructor_id));

diesel::joinable!(races -> seasons (year));
diesel::joinable!(races -> circuits (circuit_id));

diesel::joinable!(results -> races (race_id));
diesel::joinable!(results -> drivers (driver_id));
diesel::joinable!(results -> constructors (constructor_id));

diesel::joinable!(sprint_results -> races (race_id));
diesel::joinable!(sprint_results -> drivers (driver_id));
diesel::joinable!(sprint_results -> constructors (constructor_id));
