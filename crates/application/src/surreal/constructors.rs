use surql::expression::Expression;
use surql::function::array::ArrayFunction;
use surql::graph::Graph;
use surql::idiom::Idiom;
use surql::order::Order;
use surql::query::SelectStatement;

use shared::models::graphql::GetConstructorsOpts;
use surrealdb::sql::Fields;

pub fn get(constructor_ref: String) -> SelectStatement {
    SelectStatement::new()
        .all()
        .what("constructors")
        .cond(Expression::eq("ref", constructor_ref))
        .to_owned()
}

pub fn get_with_params(params: GetConstructorsOpts) -> SelectStatement {
    let mut query = SelectStatement::new();
    query
        .value()
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
                        .what("constructor_standing.constructors.*")
                        .to_owned(),
                )
                .to_owned(),
        )
        .what("result");

    if let Some(circuit_ref) = params.circuit_ref {
        query.cond(Expression::eq("circuit.ref", circuit_ref));
    }

    SelectStatement::new()
        .all()
        .what(ArrayFunction::group(query.to_owned().into_subquery()))
        .order(Order::asc("ref"))
        .to_owned()
}
