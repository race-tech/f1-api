use derives::FilterValidation;

use crate::parameters::*;

pub trait FilterValidation {
    fn validate(&self) -> Result<(), crate::error::Error>;
}

#[derive(Debug, Default, FilterValidation)]
pub struct DriverFilter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    #[validation(unique)]
    pub driver_number: Option<DriverNumber>,
    #[validation(unique)]
    pub driver_ref: Option<DriverRef>,
    pub constructor_ref: Option<ConstructorRef>,
    pub circuit_ref: Option<Circuit>,
    pub grid: Option<Grid>,
    pub result: Option<RaceResult>,
    #[validation(skip)]
    pub year: Option<Year>,
    #[validation(skip)]
    pub round: Option<Round>,
}

#[derive(Debug, Default, FilterValidation)]
pub struct ConstructorFilter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    #[validation(unique)]
    pub driver_number: Option<DriverNumber>,
    #[validation(unique)]
    pub driver_ref: Option<DriverRef>,
    pub constructor_ref: Option<ConstructorRef>,
    pub circuit_ref: Option<Circuit>,
    pub grid: Option<Grid>,
    pub result: Option<RaceResult>,
    #[validation(skip)]
    pub year: Option<Year>,
    #[validation(skip)]
    pub round: Option<Round>,
}

#[derive(Debug, Default, FilterValidation)]
pub struct DriverStandingFilter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub driver_ref: Option<DriverRef>,
    pub result: Option<ChampionshipResult>,
    #[validation(skip)]
    pub year: Option<Year>,
    #[validation(skip)]
    pub round: Option<Round>,
}

#[derive(Debug, Default, FilterValidation)]
pub struct ConstructorStandingFilter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub constructor_ref: Option<ConstructorRef>,
    pub result: Option<ChampionshipResult>,
    #[validation(skip)]
    pub year: Option<Year>,
    #[validation(skip)]
    pub round: Option<Round>,
}

#[derive(Debug, Default, FilterValidation)]
pub struct SeasonFilter {
    #[validation(skip)]
    pub limit: Option<Limit>,
    #[validation(skip)]
    pub page: Option<Page>,
    pub circuit_ref: Option<Circuit>,
    pub constructor_ref: Option<ConstructorRef>,
    pub driver_ref: Option<DriverRef>,
    pub grid: Option<Grid>,
}
