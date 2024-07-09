use surql::expression::Expression;
use surql::order::Order;
use surql::query::SelectStatement;

use shared::models::graphql::GetCircuitsOpts;

pub fn get(circuit_ref: String) -> SelectStatement {
    SelectStatement::new()
        .all()
        .what("circuit")
        .cond(Expression::eq("ref", circuit_ref))
        .to_owned()
}

pub fn get_with_params(params: GetCircuitsOpts) -> SelectStatement {
    let mut query = SelectStatement::new();
    query
        .value()
        .expr("race.circuit.*")
        .what("result")
        .order(Order::asc("ref"));

    if let Some(driver_ref) = params.driver_ref {
        query.cond(Expression::eq("driver.ref", driver_ref));
    }
    if let Some(constructor_ref) = params.constructor_ref {
        query.cond(Expression::eq("constructors.ref", constructor_ref));
    }
    if let Some(grid) = params.grid {
        query.cond(Expression::eq("grid", grid));
    }
    if let Some(fastest) = params.fastest {
        query.cond(Expression::eq("fastest", fastest));
    }
    if let Some(result) = params.result {
        query.cond(Expression::eq("result", result));
    }
    if let Some(year) = params.year {
        query.cond(Expression::eq("race.year", year));
    }
    if let Some(round) = params.round {
        query.cond(Expression::eq("race.round", round));
    }
    if let Some(status) = params.status {
        query.cond(Expression::eq("status.status", status));
    }

    query.to_owned()
}
