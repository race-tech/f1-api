use surql::expression::Expression;
use surql::function::array::ArrayFunction;
use surql::graph::Graph;
use surql::idiom::Idiom;
use surql::order::Order;
use surql::query::SelectStatement;

use shared::models::graphql::GetConstructorsOpts;
use surrealdb::sql::Fields;

use super::constructor_standings;

pub fn get(constructor_ref: String) -> SelectStatement {
    SelectStatement::new()
        .all()
        .what(Idiom::parse_unchecked("constructors"))
        .cond(Expression::eq(
            Idiom::parse_unchecked("ref"),
            constructor_ref,
        ))
        .to_owned()
}

pub fn get_with_params(params: GetConstructorsOpts) -> SelectStatement {
    let mut query = SelectStatement::new();
    query
        .all()
        .expr(
            Idiom::new()
                .push("race")
                .push(
                    Graph::graph_out()
                        .exprs(Fields::all())
                        .what("race_to_constructor_standing")
                        .to_owned(),
                )
                .push(
                    Graph::graph_out()
                        .exprs(Fields::all())
                        .what("constructor_standing.constructors")
                        .to_owned(),
                )
                .to_owned(),
        )
        .fetch(Idiom::parse_unchecked("constructors"))
        .what(Idiom::parse_unchecked("result"));

    if let Some(circuit_ref) = params.circuit_ref {
        query.cond(Expression::eq(
            Idiom::parse_unchecked("race.circuit.ref"),
            circuit_ref,
        ));
    }

    if let Some(driver_ref) = params.driver_ref {
        query.cond(Expression::eq(
            Idiom::parse_unchecked("driver.ref"),
            driver_ref,
        ));
    }

    if let Some(constructor_standings) = params.constructor_standings {
        query.cond(Expression::eq(
            Idiom::parse_unchecked(""),
            constructor_standings,
        ));
    }

    SelectStatement::new()
        .all()
        .what(ArrayFunction::group(query.to_owned().into_subquery()))
        .order(Order::asc(Idiom::parse_unchecked("ref")))
        .to_owned()
}
