use surql::expression::Expression;
use surql::idiom::Idiom;
use surql::order::Order;
use surql::query::SelectStatement;

use shared::models::graphql::GetCircuitsOpts;

pub fn get(circuit_ref: String) -> SelectStatement {
    SelectStatement::new()
        .all()
        .what(Idiom::parse_unchecked("circuit"))
        .cond(Expression::eq(Idiom::parse_unchecked("ref"), circuit_ref))
        .to_owned()
}

pub fn get_with_params(params: GetCircuitsOpts) -> SelectStatement {
    let mut query = SelectStatement::new();
    query
        .value()
        .expr(Idiom::parse_unchecked("race.circuit"))
        .what(Idiom::parse_unchecked("result"))
        .order(Order::asc(Idiom::parse_unchecked("race.circuit")))
        .fetch(Idiom::parse_unchecked("circuit"));

    if let Some(driver_ref) = params.driver_ref {
        query.cond(Expression::eq(
            Idiom::parse_unchecked("driver.ref"),
            driver_ref,
        ));
    }
    if let Some(constructor_ref) = params.constructor_ref {
        query.cond(Expression::eq(
            Idiom::parse_unchecked("constructors.ref"),
            constructor_ref,
        ));
    }
    if let Some(grid) = params.grid {
        query.cond(Expression::eq(Idiom::parse_unchecked("grid"), grid));
    }
    if let Some(fastest) = params.fastest {
        query.cond(Expression::eq(Idiom::parse_unchecked("fastest"), fastest));
    }
    if let Some(result) = params.result {
        query.cond(Expression::eq(Idiom::parse_unchecked("result"), result));
    }
    if let Some(year) = params.year {
        query.cond(Expression::eq(Idiom::parse_unchecked("race.year"), year));
    }
    if let Some(round) = params.round {
        query.cond(Expression::eq(Idiom::parse_unchecked("race.round"), round));
    }
    if let Some(status) = params.status {
        query.cond(Expression::eq(
            Idiom::parse_unchecked("status.status"),
            status,
        ));
    }

    query.to_owned()
}
