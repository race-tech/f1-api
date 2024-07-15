use surql::expression::Expression;
use surql::function::array::ArrayFunction;
use surql::graph::Graph;
use surql::idiom::Idiom;
use surql::order::Order;
use surql::query::SelectStatement;

use shared::models::graphql::GetConstructorStandingsOpts;

pub fn get_with_params(parameters: GetConstructorStandingsOpts) -> SelectStatement {
    let mut query = SelectStatement::new();
    query
        .all()
        .fetch(Idiom::parse_unchecked("constructors"))
        .fetch(Idiom::parse_unchecked("race"))
        .fetch(Idiom::parse_unchecked("race.season"))
        .fetch(Idiom::parse_unchecked("race.circuit"))
        .order(Order::asc(Idiom::parse_unchecked("race.season.year")))
        .order(Order::desc("points"))
        .what(Idiom::parse_unchecked("constructor_standing"));

    if let Some(round) = parameters.round {
        query.cond(Expression::eq(Idiom::parse_unchecked("race.round"), round));
    } else {
        query.cond(Expression::eq(
            // race.season->last_race->race
            ArrayFunction::at(
                Idiom::new()
                    .push("race")
                    .push("season")
                    .push(Graph::graph_out().what("last_race").to_owned())
                    .push(Graph::graph_out().what("race").to_owned())
                    .to_owned(),
                0,
            ),
            Idiom::parse_unchecked("race"),
        ));
    }

    if let Some(year) = parameters.year {
        query.cond(Expression::eq(
            Idiom::parse_unchecked("race.season.year"),
            year,
        ));
    }

    if let Some(constructor_ref) = parameters.constructor_ref {
        query.cond(Expression::eq(
            Idiom::parse_unchecked("constructor.ref"),
            constructor_ref,
        ));
    }

    if let Some(position) = parameters.position {
        query.cond(Expression::eq(
            Idiom::parse_unchecked("position_text"),
            position,
        ));
    }

    query.to_owned()
}
