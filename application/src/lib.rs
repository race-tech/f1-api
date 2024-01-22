mod constructor_standing;
mod driver;
mod driver_standing;
mod pagination;
mod race;
mod schema;

type Backend = diesel::mysql::Mysql;

pub mod models {
    pub use super::constructor_standing::ConstructorStanding;
    pub use super::driver::Driver;
    pub use super::driver_standing::DriverStanding;
    pub use super::race::Race;
}

pub mod filters {
    // Filters for "numbers"
    pub enum NumberFilter<T> {
        Equal(T),
        NotEqual(T),
        GreaterThen(T),
        LowerThen(T),
        IsNull,
        IsNotNull,
    }

    macro_rules! number_filter {
        ($filter:ident, $dsl_field:expr) => {{
            match $filter {
                NumberFilter::Equal(value) => Box::new($dsl_field.eq(value).nullable()),
                NumberFilter::NotEqual(value) => Box::new($dsl_field.ne(value).nullable()),
                NumberFilter::GreaterThen(value) => Box::new($dsl_field.gt(value).nullable()),
                NumberFilter::LowerThen(value) => Box::new($dsl_field.lt(value).nullable()),
                NumberFilter::IsNull => Box::new($dsl_field.is_null().nullable()),
                NumberFilter::IsNotNull => Box::new($dsl_field.is_not_null().nullable()),
            }
        }};
    }

    pub enum StringFilter {
        Equal(String),
        NotEqual(String),
        Like(String),
        In(Vec<String>),
    }

    macro_rules! string_filter {
        ($filter:ident, $dsl_field:expr ) => {{
            match $filter {
                StringFilter::Equal(value) => Box::new($dsl_field.eq(value).nullable()),
                StringFilter::NotEqual(value) => Box::new($dsl_field.ne(value).nullable()),
                StringFilter::Like(value) => Box::new($dsl_field.like(value).nullable()),
                StringFilter::In(value) => Box::new($dsl_field.eq_any(value).nullable()),
            }
        }};
    }

    pub enum BooleanFilter {
        True,
        False,
        IsNull,
        IsNotNull,
    }

    macro_rules! boolean_filter {
        ($filter:ident, $dsl_field:expr ) => {{
            match $filter {
                BooleanFilter::True => Box::new($dsl_field.eq(true).nullable()),
                BooleanFilter::False => Box::new($dsl_field.eq(false).nullable()),
                BooleanFilter::IsNull => Box::new($dsl_field.is_null().nullable()),
                BooleanFilter::IsNotNull => Box::new($dsl_field.is_not_null().nullable()),
            }
        }};
    }

    pub enum AndOr {
        And,
        Or,
    }

    macro_rules! fields_to_filter {
        ($struct:expr, $(
            $field:ident => ($condition:ident, $filter:path)
        ),*) => {
            {
                let filter = $struct;
                let mut conditions = vec![];
                $(
                    if let Some(inner) = filter.$field.map(|f| f.inner()) {
                        conditions.push(Condition::$condition($filter(inner)));
                    }
                )*
                conditions
            }
        };
    }

    macro_rules! create_filter {
        ($conditions:expr, $and_or:expr) => {
            $conditions
                .into_iter()
                // Map into array of boxed conditions
                .filter_map::<BoxedCondition, _>(Condition::into_boxed_condition)
                // Reduce to a boxed_condition1.and(boxed_condition2).and(boxed_condition3)...
                .fold(None, |boxed_conditions: Option<BoxedCondition>, boxed_condition| {
                    Some(match boxed_conditions {
                        Some(bc) => match $and_or {
                            AndOr::And => Box::new(bc.and(boxed_condition)),
                            AndOr::Or => Box::new(bc.or(boxed_condition)),
                        },
                        None => boxed_condition,
                    })
                })
        };
    }

    pub(crate) use create_filter;
    pub(crate) use boolean_filter;
    pub(crate) use fields_to_filter;
    pub(crate) use number_filter;
    pub(crate) use string_filter;
}
