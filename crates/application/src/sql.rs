use sea_query::{IntoTableRef, SimpleExpr};

pub(crate) trait SqlBuilder: Sized {
    fn stmt(&mut self) -> &mut sea_query::SelectStatement;

    fn from<F, R>(mut self, f: F, table: R) -> Self
    where
        F: FnOnce(&Self) -> bool,
        R: IntoTableRef,
    {
        if f(&self) {
            self.stmt().from(table);
        }

        self
    }

    fn and_where<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&Self) -> Option<SimpleExpr>,
    {
        if let Some(expr) = f(&self) {
            self.stmt().and_where(expr);
        }

        self
    }
}

#[macro_export]
macro_rules! one_of {
    ($($expr:expr),*) => {
        $(
            $expr.is_some() ||
        )* false
    };
}
