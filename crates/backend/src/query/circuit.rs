use diesel::dsl::AsSelect;
use diesel::prelude::*;

use crate::models::Circuit;
use crate::one_of;
use crate::query::Query;
use crate::schema::circuits::BoxedQuery;

pub struct CircuitQuery;

impl<'a> Query<'a> for CircuitQuery {
    type Params = shared::models::graphql::GetCircuitsOpts;
    type BoxedQuery = BoxedQuery<'a, crate::DieselBackend, AsSelect<Circuit, crate::DieselBackend>>;

    fn by_ref(ref_: &'a str) -> Self::BoxedQuery {
        use crate::schema::circuits::dsl::*;

        circuits
            .filter(circuit_ref.eq(ref_))
            .limit(1)
            .select(Circuit::as_select())
            .into_boxed()
    }

    fn by_params(params: Self::Params) -> Self::BoxedQuery {
        use crate::schema::circuits::dsl::*;
        use crate::schema::races::dsl::races;

        let mut query = circuits
            .order_by(circuit_ref.asc())
            .distinct()
            .select(Circuit::as_select())
            .into_boxed();

        let mut query = if one_of!(
            params.year,
            params.driver_ref,
            params.constructor_ref,
            params.status,
            params.grid,
            params.fastest,
            params.result
        ) {
            query.inner_join(races)
        } else {
            query
        };

        query
    }
}
