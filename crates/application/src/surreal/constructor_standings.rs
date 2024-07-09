use surql::expression::Expression;
use surql::function::array::ArrayFunction;
use surql::order::Order;
use surql::query::SelectStatement;

use shared::models::graphql::GetConstructorStandingsOpts;

pub fn get_with_params(parameters: GetConstructorStandingsOpts) -> SelectStatement {
    let mut query = SelectStatement::new();
    query
        .all()
        .expr("constructors.*")
        .expr("race.*")
        .expr_as("race.season.*", "season")
        .order(Order::asc("season.year"))
        .order(Order::desc("points"))
        .what("constructor_standing");

    if let Some(round) = parameters.round {
        query.cond(Expression::eq("race.round", round));
    } else {
        query.cond(Expression::eq(
            ArrayFunction::at("race.season->last_race->race", 0),
            "race",
        ));
    }

    if let Some(year) = parameters.year {
        query.cond(Expression::eq("race.season.year", year));
    }

    if let Some(constructor_ref) = parameters.constructor_ref {
        query.cond(Expression::eq("constructors.ref", constructor_ref));
    }

    if let Some(position) = parameters.position {
        query.cond(Expression::eq("position_text", position));
    }

    query.to_owned()
}
