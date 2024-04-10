use mysql::prelude::*;
use mysql::*;
use sea_query::{Alias, Asterisk, Expr, Func, MysqlQueryBuilder, Query, SelectStatement};

use infrastructure::Connection;
use shared::models::Pagination;

const DEFAULT_PER_PAGE: u64 = 20;

pub trait Paginate {
    fn paginate(self, page: u64) -> Paginated;
}

impl Paginate for SelectStatement {
    fn paginate(self, page: u64) -> Paginated {
        Paginated {
            query: self,
            page,
            per_page: DEFAULT_PER_PAGE,
            offset: offset(page, DEFAULT_PER_PAGE),
        }
    }
}

pub struct Paginated {
    query: SelectStatement,
    page: u64,
    per_page: u64,
    offset: u64,
}

impl Paginated {
    pub fn per_page(self, per_page: u64) -> Self {
        Paginated {
            per_page,
            offset: offset(self.page, per_page),
            ..self
        }
    }

    pub fn query_and_count<U>(self, conn: &mut Connection) -> (Vec<U>, Pagination)
    where
        U: FromRow,
    {
        let query = self
            .query
            .clone()
            .limit(self.per_page)
            .offset(self.offset)
            .to_string(MysqlQueryBuilder);

        let records: Vec<U> = conn.query(query).unwrap();
        let total = self.num_items(conn);
        let total_pages = (total as f64 / self.per_page as f64).ceil() as u64;

        let pagination = Pagination {
            total,
            limit: self.per_page,
            page: self.page,
            max_page: total_pages,
        };

        (records, pagination)
    }

    fn num_items(&self, conn: &mut Connection) -> u64 {
        let query = Query::select()
            .expr(Expr::cust("COUNT(*) AS num_items"))
            .from_subquery(
                self.query
                    .clone()
                    .reset_limit()
                    .reset_offset()
                    .clear_order_by()
                    .to_owned(),
                Alias::new("t"),
            )
            .to_string(MysqlQueryBuilder);

        *conn.query(query).unwrap().first().unwrap()
    }
}

fn offset(page: u64, per_page: u64) -> u64 {
    (page.saturating_sub(1)) * per_page
}
