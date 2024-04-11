use derives::FilterValidation;

use crate::parameters::*;

pub trait FilterValidation {
    fn validate(&self) -> Result<(), crate::error::Error>;
}

#[derive(Debug, Default, FilterValidation)]
pub struct GetCircuitsFilter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub driver_ref: Option<DriverRef>,
    pub constructor_ref: Option<ConstructorRef>,
    pub status: Option<Status>,
    pub grid: Option<Grid>,
    pub fastest: Option<Fastest>,
    pub result: Option<RaceResult>,
    pub year: Option<Year>,
    pub round: Option<Round>,
}

#[derive(Debug, Default, FilterValidation)]
pub struct GetDriversFilter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub circuit_ref: Option<CircuitRef>,
    pub constructor_ref: Option<ConstructorRef>,
    pub driver_standing: Option<DriverStanding>,
    pub status: Option<Status>,
    pub grid: Option<Grid>,
    pub fastest: Option<Fastest>,
    pub result: Option<RaceResult>,
    pub year: Option<Year>,
    pub round: Option<Round>,
}

#[derive(Debug, Default, FilterValidation)]
pub struct GetConstructorsFilter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub circuit_ref: Option<CircuitRef>,
    pub driver_ref: Option<DriverRef>,
    pub constructor_standing: Option<ConstructorStanding>,
    pub status: Option<Status>,
    pub grid: Option<Grid>,
    pub fastest: Option<Fastest>,
    pub result: Option<RaceResult>,
    pub year: Option<Year>,
    pub round: Option<Round>,
}
