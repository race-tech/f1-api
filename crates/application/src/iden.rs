use sea_query::Iden;

#[derive(Iden)]
pub(crate) enum Circuits {
    Table,
    CircuitId,
    CircuitRef,
    Name,
    Location,
    Country,
    Latitude,
    Longitude,
    Altitude,
    Url,
}

#[derive(Iden)]
pub(crate) enum Races {
    Table,
    RaceId,
    Year,
    Round,
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
    ResultId,
    RaceId,
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
