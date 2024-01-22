pub enum NumberFilter<T> {
    Equal(T),
    NotEqual(T),
    GreaterThen(T),
    LowerThen(T),
    IsNull,
    IsNotNull,
}

pub enum StringFilter {
    Equal(String),
    NotEqual(String),
    Like(String),
    In(Vec<String>),
}

pub enum BooleanFilter {
    True,
    False,
    IsNull,
    IsNotNull,
}

pub enum Filter {
    Number(NumberFilter<i32>),
    String(StringFilter),
    Boolean(BooleanFilter),
    And(Vec<Filter>),
    Or(Vec<Filter>),
}
