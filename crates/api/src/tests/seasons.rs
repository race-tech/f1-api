use serde_json::json;

use super::common::Test;

#[tokio::test]
async fn test_get_seasons() {
    let value = json!({
        "data": [
                    {
                        "year": 1950,
                        "url": "http://en.wikipedia.org/wiki/1950_Formula_One_season"
                    },
                    {
                        "year": 1951,
                        "url": "http://en.wikipedia.org/wiki/1951_Formula_One_season"
                    },
                    {
                        "year": 1952,
                        "url": "http://en.wikipedia.org/wiki/1952_Formula_One_season"
                    },
                    {
                        "year": 1953,
                        "url": "http://en.wikipedia.org/wiki/1953_Formula_One_season"
                    },
                    {
                        "year": 1954,
                        "url": "http://en.wikipedia.org/wiki/1954_Formula_One_season"
                    },
                    {
                        "year": 1955,
                        "url": "http://en.wikipedia.org/wiki/1955_Formula_One_season"
                    },
                    {
                        "year": 1956,
                        "url": "http://en.wikipedia.org/wiki/1956_Formula_One_season"
                    },
                    {
                        "year": 1957,
                        "url": "http://en.wikipedia.org/wiki/1957_Formula_One_season"
                    },
                    {
                        "year": 1958,
                        "url": "http://en.wikipedia.org/wiki/1958_Formula_One_season"
                    },
                    {
                        "year": 1959,
                        "url": "http://en.wikipedia.org/wiki/1959_Formula_One_season"
                    },
                    {
                        "year": 1960,
                        "url": "http://en.wikipedia.org/wiki/1960_Formula_One_season"
                    },
                    {
                        "year": 1961,
                        "url": "http://en.wikipedia.org/wiki/1961_Formula_One_season"
                    },
                    {
                        "year": 1962,
                        "url": "http://en.wikipedia.org/wiki/1962_Formula_One_season"
                    },
                    {
                        "year": 1963,
                        "url": "http://en.wikipedia.org/wiki/1963_Formula_One_season"
                    },
                    {
                        "year": 1964,
                        "url": "http://en.wikipedia.org/wiki/1964_Formula_One_season"
                    },
                    {
                        "year": 1965,
                        "url": "http://en.wikipedia.org/wiki/1965_Formula_One_season"
                    },
                    {
                        "year": 1966,
                        "url": "http://en.wikipedia.org/wiki/1966_Formula_One_season"
                    },
                    {
                        "year": 1967,
                        "url": "http://en.wikipedia.org/wiki/1967_Formula_One_season"
                    },
                    {
                        "year": 1968,
                        "url": "http://en.wikipedia.org/wiki/1968_Formula_One_season"
                    },
                    {
                        "year": 1969,
                        "url": "http://en.wikipedia.org/wiki/1969_Formula_One_season"
                    },
                    {
                        "year": 1970,
                        "url": "http://en.wikipedia.org/wiki/1970_Formula_One_season"
                    },
                    {
                        "year": 1971,
                        "url": "http://en.wikipedia.org/wiki/1971_Formula_One_season"
                    },
                    {
                        "year": 1972,
                        "url": "http://en.wikipedia.org/wiki/1972_Formula_One_season"
                    },
                    {
                        "year": 1973,
                        "url": "http://en.wikipedia.org/wiki/1973_Formula_One_season"
                    },
                    {
                        "year": 1974,
                        "url": "http://en.wikipedia.org/wiki/1974_Formula_One_season"
                    },
                    {
                        "year": 1975,
                        "url": "http://en.wikipedia.org/wiki/1975_Formula_One_season"
                    },
                    {
                        "year": 1976,
                        "url": "http://en.wikipedia.org/wiki/1976_Formula_One_season"
                    },
                    {
                        "year": 1977,
                        "url": "http://en.wikipedia.org/wiki/1977_Formula_One_season"
                    },
                    {
                        "year": 1978,
                        "url": "http://en.wikipedia.org/wiki/1978_Formula_One_season"
                    },
                    {
                        "year": 1979,
                        "url": "http://en.wikipedia.org/wiki/1979_Formula_One_season"
                    }
                ]
    });

    Test::new(
        r#"{
            seasons {
                data {
                    year
                    url
                }
            }
        }
        "#,
        value,
    )
    .specify_field("seasons")
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_seasons_by_year() {
    Test::new(
        r#"{
            season(year: 2023) {
                year
                url
            }
        }
        "#,
        json!({
            "season": {
                        "year": 2023,
                        "url": "http://en.wikipedia.org/wiki/2023_Formula_One_World_Championship"
                    }
        }),
    )
    .test_ok()
    .await
}
