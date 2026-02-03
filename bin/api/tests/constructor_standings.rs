mod common;

#[tokio::test]
async fn test_get_constructor_standings() {
    common::Test::new(
        r#"query {
            constructorStandings(pagination: {page: {limit: 10, page: 1}}) {
                nodes {
                    points
                    position
                    positionText
                    wins

                    races {
                        year
                        round
                    }
                }
            }
        }
        "#,
    )
    .await
    .test_map(|res| {
        let nodes = common::extract_nodes(res, "constructorStandings");
        assert_eq!(nodes.len(), 10, "Invalid nodes len");

        assert_eq!(
            serde_json::Value::Array(nodes),
            serde_json::json!([
              {
                "points": 2.0,
                "position": 8,
                "positionText": "8",
                "wins": 0,
                "races": {
                  "year": 2008,
                  "round": 2
                }
              },
              {
                "points": 11.0,
                "position": 3,
                "positionText": "3",
                "wins": 1,
                "races": {
                  "year": 2008,
                  "round": 2
                }
              },
              {
                "points": 5.0,
                "position": 6,
                "positionText": "6",
                "wins": 0,
                "races": {
                  "year": 2008,
                  "round": 2
                }
              },
              {
                "points": 2.0,
                "position": 7,
                "positionText": "7",
                "wins": 0,
                "races": {
                  "year": 2008,
                  "round": 2
                }
              },
              {
                "points": 0.0,
                "position": 9,
                "positionText": "9",
                "wins": 0,
                "races": {
                  "year": 2008,
                  "round": 2
                }
              },
              {
                "points": 0.0,
                "position": 10,
                "positionText": "10",
                "wins": 0,
                "races": {
                  "year": 2008,
                  "round": 2
                }
              },
              {
                "points": 0.0,
                "position": 11,
                "positionText": "11",
                "wins": 0,
                "races": {
                  "year": 2008,
                  "round": 2
                }
              },
              {
                "points": 28.0,
                "position": 3,
                "positionText": "3",
                "wins": 1,
                "races": {
                  "year": 2008,
                  "round": 3
                }
              },
              {
                "points": 30.0,
                "position": 1,
                "positionText": "1",
                "wins": 0,
                "races": {
                  "year": 2008,
                  "round": 3
                }
              },
              {
                "points": 10.0,
                "position": 4,
                "positionText": "4",
                "wins": 0,
                "races": {
                  "year": 2008,
                  "round": 3
                }
              }
            ])
        );
    })
    .await;
}
