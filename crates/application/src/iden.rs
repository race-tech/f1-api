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
    ConstructorId,
    Number,
    Grid,
    Position,
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
    ConstructorId,
    ConstructorRef,
    Name,
    Nationality,
    Url,
}
