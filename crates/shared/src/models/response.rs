use async_graphql::*;

use crate::models::graphql;

type Races = Vec<graphql::Race>;
type Circuits = Vec<graphql::Circuit>;
type ConstructorStandings = Vec<graphql::ConstructorStanding>;
type Constructors = Vec<graphql::Constructor>;
type DriverStandings = Vec<graphql::DriverStanding>;
type Drivers = Vec<graphql::Driver>;
type Seasons = Vec<graphql::Season>;
type Statuses = Vec<graphql::Status>;

#[derive(SimpleObject)]
#[graphql(concrete(name = "Races", params(Races)))]
#[graphql(concrete(name = "Circuits", params(Circuits)))]
#[graphql(concrete(name = "ConstructorStandings", params(ConstructorStandings)))]
#[graphql(concrete(name = "Constructors", params(Constructors)))]
#[graphql(concrete(name = "DriverStandings", params(DriverStandings)))]
#[graphql(concrete(name = "Drivers", params(Drivers)))]
#[graphql(concrete(name = "Seasons", params(Seasons)))]
#[graphql(concrete(name = "Statuses", params(Statuses)))]
pub struct Response<T>
where
    T: async_graphql::OutputType,
{
    pub data: T,
    pub pagination: Pagination,
}

#[derive(SimpleObject)]
pub struct Pagination {
    pub limit: u64,
    pub page: u64,
    pub max_page: u64,
    pub total: u64,
}

impl<T> From<(T, Pagination)> for Response<T>
where
    T: async_graphql::OutputType,
{
    fn from(value: (T, Pagination)) -> Self {
        Self {
            data: value.0,
            pagination: value.1,
        }
    }
}
