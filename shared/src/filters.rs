use crate::parameters::*;

#[derive(Debug, Default)]
pub struct DriverFilter {
    pub limit: Option<Limit>,
    pub page: Option<Page>,
    pub driver_number: Option<DriverNumber>,
    pub driver_ref: Option<DriverRef>,
    pub constructor: Option<ConstructorName>,
    pub circuit: Option<Circuit>,
    pub grid: Option<Grid>,
    pub result: Option<RaceResult>,
    pub year: Option<Year>,
    pub round: Option<Round>,
}

#[derive(Debug, Clone)]
pub struct ConstructorFilter {
    pub limit: Option<Limit>,
    pub page: Option<Page>,
    pub driver_number: Option<DriverNumber>,
    pub driver_ref: Option<DriverRef>,
    pub constructor: Option<ConstructorName>,
    pub circuit: Option<Circuit>,
    pub grid: Option<Grid>,
    pub result: Option<RaceResult>,
    pub year: Option<Year>,
    pub round: Option<Round>,
}

#[derive(Debug, Default)]
pub struct DriverStandingFilter {
    pub limit: Option<Limit>,
    pub page: Option<Page>,
    pub name: Option<DriverRef>,
    pub result: Option<RaceResult>,
    pub race_id: Option<RaceId>,
}
