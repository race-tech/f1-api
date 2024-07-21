use mysql::prelude::{FromRow, Queryable};
use sea_query::{IntoTableRef, MysqlQueryBuilder, SimpleExpr};

use shared::{
    error::Result,
    models::{graphql::PaginationOpts, response::Pagination},
};

use crate::pagination::Paginate;

pub trait SqlBuilder: Sized {
    type Output: FromRow;

    fn stmt(&mut self) -> &mut sea_query::SelectStatement;

    fn query(mut self, conn: &mut infrastructure::Connection) -> Result<Vec<Self::Output>> {
        let query = self.stmt().to_string(MysqlQueryBuilder);
        Ok(conn.query(query)?)
    }

    fn query_first(
        mut self,
        conn: &mut infrastructure::Connection,
    ) -> Result<Option<Self::Output>> {
        let query = self.stmt().to_string(MysqlQueryBuilder);
        Ok(conn.query_first(query)?)
    }

    fn query_pagination(
        mut self,
        pagination: PaginationOpts,
        conn: &mut infrastructure::Connection,
    ) -> Result<(Vec<Self::Output>, Pagination)> {
        self.stmt()
            .to_owned()
            .paginate(pagination.page.unwrap_or_default())
            .per_page(pagination.limit.unwrap_or_default())
            .query_and_count(conn)
    }

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
