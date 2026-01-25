mod common;

#[tokio::test]
async fn test_get_circuit_by_ref() {
    common::Test::new(
        r#"query {
            circuits(filters: {circuitRef: {eq: "spa"}}) {
                nodes {
                    circuitRef
                    name
                    location
                    country
                    lat
                    lng
                    alt
                    url
                }
            }   
        }"#,
    )
    .await
    .test_map(|res| {
        let nodes = common::extract_nodes(res, "circuits");
        assert_eq!(nodes.len(), 1, "Invalid nodes len");

        assert_eq!(
            nodes[0],
            serde_json::json!({
                "circuitRef": "spa",
                "name": "Circuit de Spa-Francorchamps",
                "location": "Spa",
                "country": "Belgium",
                "lat": 50.437198638916016,
                "lng": 5.9713897705078125,
                "alt": 401,
                "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"
            })
        );
    })
    .await;
}

#[tokio::test]
async fn test_get_circuit_by_year_and_round() {
    common::Test::new(
        r#"query {
            circuits(having: {races: {year: {eq: 2023}, round: {eq: 22}}}) {
                nodes {
                    circuitRef
                    name
                    location
                    country
                    lat
                    lng
                    alt
                    url
                }
            }   
        }"#,
    )
    .await
    .test_map(|res| {
        let nodes = common::extract_nodes(res, "circuits");
        assert_eq!(nodes.len(), 1, "Invalid nodes len");

        assert_eq!(
            nodes[0],
            serde_json::json!({
                "circuitRef": "yas_marina",
                "name": "Yas Marina Circuit",
                "location": "Abu Dhabi",
                "country": "UAE",
                "lat": 24.467199325561523,
                "lng": 54.60309982299805,
                "alt": 3,
                "url": "http://en.wikipedia.org/wiki/Yas_Marina_Circuit"
            })
        );
    })
    .await;
}
