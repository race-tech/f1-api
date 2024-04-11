#![allow(dead_code)]

use sea_query::Iden;

#[derive(Iden)]
pub(crate) enum Circuits {
    Table,
    #[iden = "circuitId"]
    CircuitId,
    #[iden = "circuitRef"]
    CircuitRef,
    Name,
    Location,
    Country,
    Lat,
    Lng,
    Alt,
    Url,
}

#[derive(Iden)]
pub(crate) enum Races {
    Table,
    #[iden = "raceId"]
    RaceId,
    Year,
    Round,
    #[iden = "circuitId"]
    CircuitId,
    Name,
    Date,
    Time,
    Url,
    Fp1Date,
    Fp1Time,
    Fp2Date,
    Fp2Time,
    Fp3Date,
    Fp3Time,
    QualiDate,
    QualiTime,
    SprintDate,
    SprintTime,
}

#[derive(Iden)]
pub(crate) enum Results {
    Table,
    #[iden = "resultId"]
    ResultId,
    #[iden = "raceId"]
    RaceId,
    #[iden = "driverId"]
    DriverId,
    #[iden = "constructorId"]
    ConstructorId,
    Number,
    Grid,
    Position,
    #[iden = "positionText"]
    PositionText,
    PositionOrder,
    Points,
    Laps,
    Time,
    Milliseconds,
    FastestLap,
    Rank,
    FastestLapTime,
    FastestLapSpeed,
    StatusId,
}

#[derive(Iden)]
pub(crate) enum Drivers {
    Table,
    #[iden = "driverId"]
    DriverId,
    #[iden = "driverRef"]
    DriverRef,
    Number,
    Code,
    Forename,
    Surname,
    Dob,
    Nationality,
    Url,
}

#[derive(Iden)]
pub(crate) enum Constructors {
    Table,
    #[iden = "constructorId"]
    ConstructorId,
    #[iden = "constructorRef"]
    ConstructorRef,
    Name,
    Nationality,
    Url,
}

#[derive(Iden)]
pub(crate) enum DriverStandings {
    #[iden = "driverStandings"]
    Table,
    #[iden = "driverStandingsId"]
    Id,
    #[iden = "raceId"]
    RaceId,
    #[iden = "driverId"]
    DriverId,
    Points,
    Position,
    #[iden = "positionText"]
    PositionText,
    Wins,
}

#[derive(Iden)]
pub(crate) enum ConstructorStandings {
    #[iden = "constructorStandings"]
    Table,
    #[iden = "constructorStandingsId"]
    Id,
    #[iden = "raceId"]
    RaceId,
    #[iden = "constructorId"]
    ConstructorId,
    Points,
    Position,
    #[iden = "positionText"]
    PositionText,
    Wins,
}
