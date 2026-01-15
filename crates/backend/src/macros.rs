#[macro_export]
macro_rules! one_of {
    ($($expr:expr),*) => {
        $(
            $expr.is_some() ||
        )* false
    };
}
