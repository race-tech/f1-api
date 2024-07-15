use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

mod circuit;
mod constructor_standing;

pub type ServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct Query(
    circuit::CircuitQuery,
    constructor_standing::ConstructorStandingQuery,
);
