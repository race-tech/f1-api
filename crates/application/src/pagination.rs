use std::collections::HashMap;

use serde::de::DeserializeOwned;
use surql::function::count::Count;
use surql::query::SelectStatement;

use infrastructure::Conn;
use shared::error;
use shared::models::graphql::PaginationOpts;
use shared::models::response::Pagination;
use shared::prelude::Result;

const DEFAULT_PER_PAGE: u64 = 20;

pub struct Paginated<Q, U> {
    query: Q,
    params: HashMap<String, serde_json::Value>,
    page: u64,
    per_page: u64,
    offset: u64,
    data: std::marker::PhantomData<U>,
}

pub trait Paginate {
    type Query;

    fn paginate<U>(self, pagination: PaginationOpts) -> Paginated<Self::Query, U>;
}

impl Paginate for SelectStatement {
    type Query = SelectStatement;

    fn paginate<U>(self, pagination: PaginationOpts) -> Paginated<Self::Query, U> {
        let page = pagination.page.unwrap_or_default();
        let per_page = pagination.limit.unwrap_or(DEFAULT_PER_PAGE);

        Paginated {
            query: self,
            params: HashMap::new(),
            page,
            per_page,
            offset: offset(page, per_page),
            data: std::marker::PhantomData,
        }
    }
}

impl<U: DeserializeOwned> Paginated<SelectStatement, U> {
    pub async fn query_and_count<'a>(
        mut self,
        conn: &mut Conn<'a>,
    ) -> Result<(Vec<U>, Pagination)> {
        let subquery = self.query.clone().into_subquery();
        let count = Count::new(Some(subquery));
        let query = self
            .query
            .limit(self.per_page)
            .start(self.offset)
            .to_owned();

        let mut res = conn
            .query(vec![count.into(), query.into()])
            .bind(self.params)
            .await?;

        let total: Option<u64> = res.take(0)?;

        if total.is_none() {
            return Err(error!(InternalServer => "error while querying pagination count"));
        }

        let records: Vec<U> = res.take(1)?;
        let total_pages = (total.unwrap() as f64 / self.per_page as f64).ceil() as u64;

        let pagination = Pagination {
            total: total.unwrap(),
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
