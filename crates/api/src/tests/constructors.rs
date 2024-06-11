use serde_json::json;

use super::common::Test;

#[tokio::test]
async fn test_get_constructors_by_ref() {
    Test::new(
        r#"{
            constructor(constructorRef: "ferrari") {
                constructorRef
                name
                nationality
                url
            }
        }"#,
        json!({
            "constructor": {
                        "constructorRef": "ferrari",
                        "name": "Ferrari",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                    }
        }),
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_constructors() {
    let value: serde_json::Value = json!({
        "constructors": [
                    {
                        "constructorRef": "mclaren",
                        "name": "McLaren",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/McLaren"
                    },
                    {
                        "constructorRef": "bmw_sauber",
                        "name": "BMW Sauber",
                        "nationality": "German",
                        "url": "http://en.wikipedia.org/wiki/BMW_Sauber"
                    },
                    {
                        "constructorRef": "williams",
                        "name": "Williams",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Williams_Grand_Prix_Engineering"
                    },
                    {
                        "constructorRef": "renault",
                        "name": "Renault",
                        "nationality": "French",
                        "url": "http://en.wikipedia.org/wiki/Renault_in_Formula_One"
                    },
                    {
                        "constructorRef": "toro_rosso",
                        "name": "Toro Rosso",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Scuderia_Toro_Rosso"
                    },
                    {
                        "constructorRef": "ferrari",
                        "name": "Ferrari",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                    },
                    {
                        "constructorRef": "toyota",
                        "name": "Toyota",
                        "nationality": "Japanese",
                        "url": "http://en.wikipedia.org/wiki/Toyota_Racing"
                    },
                    {
                        "constructorRef": "super_aguri",
                        "name": "Super Aguri",
                        "nationality": "Japanese",
                        "url": "http://en.wikipedia.org/wiki/Super_Aguri_F1"
                    },
                    {
                        "constructorRef": "red_bull",
                        "name": "Red Bull",
                        "nationality": "Austrian",
                        "url": "http://en.wikipedia.org/wiki/Red_Bull_Racing"
                    },
                    {
                        "constructorRef": "force_india",
                        "name": "Force India",
                        "nationality": "Indian",
                        "url": "http://en.wikipedia.org/wiki/Racing_Point_Force_India"
                    },
                    {
                        "constructorRef": "honda",
                        "name": "Honda",
                        "nationality": "Japanese",
                        "url": "http://en.wikipedia.org/wiki/Honda_Racing_F1"
                    },
                    {
                        "constructorRef": "spyker",
                        "name": "Spyker",
                        "nationality": "Dutch",
                        "url": "http://en.wikipedia.org/wiki/Spyker_F1"
                    },
                    {
                        "constructorRef": "mf1",
                        "name": "MF1",
                        "nationality": "Russian",
                        "url": "http://en.wikipedia.org/wiki/Midland_F1_Racing"
                    },
                    {
                        "constructorRef": "spyker_mf1",
                        "name": "Spyker MF1",
                        "nationality": "Dutch",
                        "url": "http://en.wikipedia.org/wiki/Midland_F1_Racing"
                    },
                    {
                        "constructorRef": "sauber",
                        "name": "Sauber",
                        "nationality": "Swiss",
                        "url": "http://en.wikipedia.org/wiki/Sauber_Motorsport"
                    },
                    {
                        "constructorRef": "bar",
                        "name": "BAR",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/British_American_Racing"
                    },
                    {
                        "constructorRef": "jordan",
                        "name": "Jordan",
                        "nationality": "Irish",
                        "url": "http://en.wikipedia.org/wiki/Jordan_Grand_Prix"
                    },
                    {
                        "constructorRef": "minardi",
                        "name": "Minardi",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Minardi"
                    },
                    {
                        "constructorRef": "jaguar",
                        "name": "Jaguar",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Jaguar_Racing"
                    },
                    {
                        "constructorRef": "prost",
                        "name": "Prost",
                        "nationality": "French",
                        "url": "http://en.wikipedia.org/wiki/Prost_Grand_Prix"
                    },
                    {
                        "constructorRef": "arrows",
                        "name": "Arrows",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Arrows_Grand_Prix_International"
                    },
                    {
                        "constructorRef": "benetton",
                        "name": "Benetton",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Benetton_Formula"
                    },
                    {
                        "constructorRef": "brawn",
                        "name": "Brawn",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Brawn_GP"
                    },
                    {
                        "constructorRef": "stewart",
                        "name": "Stewart",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Stewart_Grand_Prix"
                    },
                    {
                        "constructorRef": "tyrrell",
                        "name": "Tyrrell",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Tyrrell_Racing"
                    },
                    {
                        "constructorRef": "lola",
                        "name": "Lola",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/MasterCard_Lola"
                    },
                    {
                        "constructorRef": "ligier",
                        "name": "Ligier",
                        "nationality": "French",
                        "url": "http://en.wikipedia.org/wiki/Ligier"
                    },
                    {
                        "constructorRef": "forti",
                        "name": "Forti",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Forti"
                    },
                    {
                        "constructorRef": "footwork",
                        "name": "Footwork",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Footwork_Arrows"
                    },
                    {
                        "constructorRef": "pacific",
                        "name": "Pacific",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Pacific_Racing"
                    }
                ]
    });

    Test::new(
        r#"{
            constructors(pagination: { limit: 30, page: 1 }) {
                constructorRef
                name
                nationality
                url
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_constructors_by_driver_ref() {
    let value: serde_json::Value = json!({
        "constructors": [
                    {
                        "constructorRef": "sauber",
                        "name": "Sauber",
                        "nationality": "Swiss",
                        "url": "http://en.wikipedia.org/wiki/Sauber_Motorsport"
                    },
                    {
                        "constructorRef": "ferrari",
                        "name": "Ferrari",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                    }
                ]
    });

    Test::new(
        r#"{
            constructors(
                pagination: { limit: 30, page: 1 }
                options: { driverRef: "leclerc" }
            ) {
                constructorRef
                name
                nationality
                url
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}

#[tokio::test]
async fn test_get_value() {
    let value: serde_json::Value = json!({
        "constructors": [
                    {
                        "constructorRef": "ferrari",
                        "name": "Ferrari",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
                    },
                    {
                        "constructorRef": "renault",
                        "name": "Renault",
                        "nationality": "French",
                        "url": "http://en.wikipedia.org/wiki/Renault_in_Formula_One"
                    },
                    {
                        "constructorRef": "mclaren",
                        "name": "McLaren",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/McLaren"
                    },
                    {
                        "constructorRef": "williams",
                        "name": "Williams",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Williams_Grand_Prix_Engineering"
                    },
                    {
                        "constructorRef": "benetton",
                        "name": "Benetton",
                        "nationality": "Italian",
                        "url": "http://en.wikipedia.org/wiki/Benetton_Formula"
                    },
                    {
                        "constructorRef": "team_lotus",
                        "name": "Team Lotus",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Team_Lotus"
                    },
                    {
                        "constructorRef": "tyrrell",
                        "name": "Tyrrell",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Tyrrell_Racing"
                    },
                    {
                        "constructorRef": "brawn",
                        "name": "Brawn",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Brawn_GP"
                    },
                    {
                        "constructorRef": "matra-ford",
                        "name": "Matra-Ford",
                        "nationality": "French",
                        "url": "http://en.wikipedia.org/wiki/Matra"
                    },
                    {
                        "constructorRef": "lotus-ford",
                        "name": "Lotus-Ford",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Team_Lotus"
                    },
                    {
                        "constructorRef": "brabham-repco",
                        "name": "Brabham-Repco",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Brabham"
                    },
                    {
                        "constructorRef": "cooper-climax",
                        "name": "Cooper-Climax",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Cooper_Car_Company"
                    },
                    {
                        "constructorRef": "vanwall",
                        "name": "Vanwall",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Vanwall"
                    },
                    {
                        "constructorRef": "lotus-climax",
                        "name": "Lotus-Climax",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/Team_Lotus"
                    },
                    {
                        "constructorRef": "brm",
                        "name": "BRM",
                        "nationality": "British",
                        "url": "http://en.wikipedia.org/wiki/BRM"
                    },
                    {
                        "constructorRef": "red_bull",
                        "name": "Red Bull",
                        "nationality": "Austrian",
                        "url": "http://en.wikipedia.org/wiki/Red_Bull_Racing"
                    },
                    {
                        "constructorRef": "mercedes",
                        "name": "Mercedes",
                        "nationality": "German",
                        "url": "http://en.wikipedia.org/wiki/Mercedes-Benz_in_Formula_One"
                    }
                ]
    });

    Test::new(
        r#"{
            constructors(
                pagination: { limit: 30, page: 1 }
                options: { constructorStanding: 1 }
            ) {
                constructorRef
                name
                nationality
                url
            }
        }"#,
        value,
    )
    .test_ok()
    .await
}
