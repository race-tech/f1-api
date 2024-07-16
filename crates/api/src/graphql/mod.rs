use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

mod circuit;
mod constructor;
mod constructor_standing;
mod driver;
mod driver_standing;
mod lap_time;
mod pit_stop;
mod race;
mod season;
mod status;

pub type ServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct Query(
    circuit::CircuitQuery,
    constructor_standing::ConstructorStandingQuery,
    constructor::ConstructorQuery,
    driver_standing::DriverStandingQuery,
    driver::DriverQuery,
    lap_time::LapTimeQuery,
    pit_stop::PitStopQuery,
    race::RaceQuery,
    season::SeasonQuery,
    status::StatusQuery,
);
