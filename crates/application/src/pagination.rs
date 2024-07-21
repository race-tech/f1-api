use mysql::prelude::{FromRow, Queryable};
use mysql::{FromRowError, Row};
use sea_query::{
    Alias, Asterisk, Expr, MysqlQueryBuilder, Query, SelectStatement, WindowStatement,
};

use infrastructure::Connection;
use shared::error::Result;
use shared::models::response::Pagination;

const DEFAULT_PER_PAGE: u64 = 20;

pub trait Paginate {
    fn paginate<U>(self, page: u64) -> Paginated<U>;
}

impl Paginate for SelectStatement {
    fn paginate<U>(self, page: u64) -> Paginated<U> {
        Paginated {
            query: self,
            page,
            per_page: DEFAULT_PER_PAGE,
            offset: offset(page, DEFAULT_PER_PAGE),
            data: std::marker::PhantomData,
        }
    }
}

pub struct Paginated<U> {
    query: SelectStatement,
    page: u64,
    per_page: u64,
    offset: u64,
    data: std::marker::PhantomData<U>,
}

impl<U: FromRow> Paginated<U> {
    pub fn per_page(self, per_page: u64) -> Self {
        Paginated {
            per_page,
            offset: offset(self.page, per_page),
            ..self
        }
    }

    pub fn query_and_count(self, conn: &mut Connection) -> Result<(Vec<U>, Pagination)> {
        let query = Query::select()
            .column(Asterisk)
            .expr_window_as(
                Expr::cust("COUNT(*)"),
                WindowStatement::new(),
                Alias::new("pagination_count_total"),
            )
            .from_subquery(self.query, Alias::new("t"))
            .limit(self.per_page)
            .offset(self.offset)
            .to_string(MysqlQueryBuilder);

        let res: Vec<PaginationResult<U>> = conn.query(query)?;
        let total = res.first().map(|r| r.total).unwrap_or(0);
        let records = res.into_iter().map(|r| r.data).collect();
        let total_pages = (total as f64 / self.per_page as f64).ceil() as u64;

        let pagination = Pagination {
            total,
            limit: self.per_page,
            page: self.page,
            max_page: total_pages,
        };

        Ok((records, pagination))
    }
}

fn offset(page: u64, per_page: u64) -> u64 {
    (page.saturating_sub(1)) * per_page
}

struct PaginationResult<U> {
    data: U,
    total: u64,
}

impl<U> FromRow for PaginationResult<U>
where
    U: FromRow,
{
    fn from_row(mut row: Row) -> Self
    where
        Self: Sized,
    {
        let total = row.take("pagination_count_total");
        assert!(total.is_some(), "error while querying pagination count");

        let data = U::from_row(row);

        PaginationResult {
            data,
            total: total.unwrap(),
        }
    }

    fn from_row_opt(mut row: Row) -> std::prelude::v1::Result<Self, FromRowError>
    where
        Self: Sized,
    {
        let total = row.take_opt("pagination_count_total");
        assert!(total.is_some(), "error while querying pagination count");
        assert!(
            total.as_ref().unwrap().is_ok(),
            "error while parsing pagination count"
        );

        let data = U::from_row_opt(row)?;

        Ok(PaginationResult {
            data,
            total: total.unwrap().unwrap(),
        })
    }
}
