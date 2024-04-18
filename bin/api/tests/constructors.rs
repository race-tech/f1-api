use shared::prelude::*;

pub mod common;

use common::models::StaticConstructor;

#[test]
fn test_get_constructors_by_ref() {
    common::Test::<StaticConstructor, Constructor>::new(
        "/api/f1/constructors/?constructor_ref=ferrari",
        Series::F1,
        StaticConstructor {
            constructor_ref: "ferrari",
            name: "Ferrari",
            nationality: Some("Italian"),
            url: "http://en.wikipedia.org/wiki/Scuderia_Ferrari",
        },
    )
    .test_ok();
}

#[test]
fn test_get_constructors() {
    common::Test::<&[StaticConstructor], Vec<Constructor>>::new(
        "/api/f1/constructors/",
        Series::F1,
        &ALL_CONSTRUCTORS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 8,
        total: 212,
    }))
    .test_ok();
}

#[test]
fn test_get_constructors_by_driver_ref() {
    common::Test::<&[StaticConstructor], Vec<Constructor>>::new(
        "/api/f1/constructors/?driver_ref=leclerc",
        Series::F1,
        &LECLERC_CONSTRUCTORS,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 1,
        total: 2,
    }))
    .test_ok();
}

#[test]
fn test_get_constructors_with_title() {
    common::Test::<&[StaticConstructor], Vec<Constructor>>::new(
        "/api/f1/constructors/?constructor_standing=1",
        Series::F1,
        &CONSTRUCTORS_WITH_TITLE,
    )
    .pagination(Some(Pagination {
        limit: 30,
        page: 1,
        max_page: 1,
        total: 17,
    }))
    .test_ok();
}

const ALL_CONSTRUCTORS: [StaticConstructor; 30] = constructors_from_json![
    {
        "constructor_ref": "mclaren",
        "name": "McLaren",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/McLaren"
    },
    {
        "constructor_ref": "bmw_sauber",
        "name": "BMW Sauber",
        "nationality": "German",
        "url": "http://en.wikipedia.org/wiki/BMW_Sauber"
    },
    {
        "constructor_ref": "williams",
        "name": "Williams",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Williams_Grand_Prix_Engineering"
    },
    {
        "constructor_ref": "renault",
        "name": "Renault",
        "nationality": "French",
        "url": "http://en.wikipedia.org/wiki/Renault_in_Formula_One"
    },
    {
        "constructor_ref": "toro_rosso",
        "name": "Toro Rosso",
        "nationality": "Italian",
        "url": "http://en.wikipedia.org/wiki/Scuderia_Toro_Rosso"
    },
    {
        "constructor_ref": "ferrari",
        "name": "Ferrari",
        "nationality": "Italian",
        "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
    },
    {
        "constructor_ref": "toyota",
        "name": "Toyota",
        "nationality": "Japanese",
        "url": "http://en.wikipedia.org/wiki/Toyota_Racing"
    },
    {
        "constructor_ref": "super_aguri",
        "name": "Super Aguri",
        "nationality": "Japanese",
        "url": "http://en.wikipedia.org/wiki/Super_Aguri_F1"
    },
    {
        "constructor_ref": "red_bull",
        "name": "Red Bull",
        "nationality": "Austrian",
        "url": "http://en.wikipedia.org/wiki/Red_Bull_Racing"
    },
    {
        "constructor_ref": "force_india",
        "name": "Force India",
        "nationality": "Indian",
        "url": "http://en.wikipedia.org/wiki/Racing_Point_Force_India"
    },
    {
        "constructor_ref": "honda",
        "name": "Honda",
        "nationality": "Japanese",
        "url": "http://en.wikipedia.org/wiki/Honda_Racing_F1"
    },
    {
        "constructor_ref": "spyker",
        "name": "Spyker",
        "nationality": "Dutch",
        "url": "http://en.wikipedia.org/wiki/Spyker_F1"
    },
    {
        "constructor_ref": "mf1",
        "name": "MF1",
        "nationality": "Russian",
        "url": "http://en.wikipedia.org/wiki/Midland_F1_Racing"
    },
    {
        "constructor_ref": "spyker_mf1",
        "name": "Spyker MF1",
        "nationality": "Dutch",
        "url": "http://en.wikipedia.org/wiki/Midland_F1_Racing"
    },
    {
        "constructor_ref": "sauber",
        "name": "Sauber",
        "nationality": "Swiss",
        "url": "http://en.wikipedia.org/wiki/Sauber_Motorsport"
    },
    {
        "constructor_ref": "bar",
        "name": "BAR",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/British_American_Racing"
    },
    {
        "constructor_ref": "jordan",
        "name": "Jordan",
        "nationality": "Irish",
        "url": "http://en.wikipedia.org/wiki/Jordan_Grand_Prix"
    },
    {
        "constructor_ref": "minardi",
        "name": "Minardi",
        "nationality": "Italian",
        "url": "http://en.wikipedia.org/wiki/Minardi"
    },
    {
        "constructor_ref": "jaguar",
        "name": "Jaguar",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Jaguar_Racing"
    },
    {
        "constructor_ref": "prost",
        "name": "Prost",
        "nationality": "French",
        "url": "http://en.wikipedia.org/wiki/Prost_Grand_Prix"
    },
    {
        "constructor_ref": "arrows",
        "name": "Arrows",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Arrows_Grand_Prix_International"
    },
    {
        "constructor_ref": "benetton",
        "name": "Benetton",
        "nationality": "Italian",
        "url": "http://en.wikipedia.org/wiki/Benetton_Formula"
    },
    {
        "constructor_ref": "brawn",
        "name": "Brawn",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Brawn_GP"
    },
    {
        "constructor_ref": "stewart",
        "name": "Stewart",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Stewart_Grand_Prix"
    },
    {
        "constructor_ref": "tyrrell",
        "name": "Tyrrell",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Tyrrell_Racing"
    },
    {
        "constructor_ref": "lola",
        "name": "Lola",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/MasterCard_Lola"
    },
    {
        "constructor_ref": "ligier",
        "name": "Ligier",
        "nationality": "French",
        "url": "http://en.wikipedia.org/wiki/Ligier"
    },
    {
        "constructor_ref": "forti",
        "name": "Forti",
        "nationality": "Italian",
        "url": "http://en.wikipedia.org/wiki/Forti"
    },
    {
        "constructor_ref": "footwork",
        "name": "Footwork",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Footwork_Arrows"
    },
    {
        "constructor_ref": "pacific",
        "name": "Pacific",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Pacific_Racing"
    }
];

const LECLERC_CONSTRUCTORS: [StaticConstructor; 2] = constructors_from_json![
    {
        "constructor_ref": "sauber",
        "name": "Sauber",
        "nationality": "Swiss",
        "url": "http://en.wikipedia.org/wiki/Sauber_Motorsport"
    },
    {
        "constructor_ref": "ferrari",
        "name": "Ferrari",
        "nationality": "Italian",
        "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
    }
];

const CONSTRUCTORS_WITH_TITLE: [StaticConstructor; 17] = constructors_from_json![
    {
        "constructor_ref": "ferrari",
        "name": "Ferrari",
        "nationality": "Italian",
        "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari"
    },
    {
        "constructor_ref": "renault",
        "name": "Renault",
        "nationality": "French",
        "url": "http://en.wikipedia.org/wiki/Renault_in_Formula_One"
    },
    {
        "constructor_ref": "mclaren",
        "name": "McLaren",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/McLaren"
    },
    {
        "constructor_ref": "williams",
        "name": "Williams",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Williams_Grand_Prix_Engineering"
    },
    {
        "constructor_ref": "benetton",
        "name": "Benetton",
        "nationality": "Italian",
        "url": "http://en.wikipedia.org/wiki/Benetton_Formula"
    },
    {
        "constructor_ref": "team_lotus",
        "name": "Team Lotus",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Team_Lotus"
    },
    {
        "constructor_ref": "tyrrell",
        "name": "Tyrrell",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Tyrrell_Racing"
    },
    {
        "constructor_ref": "brawn",
        "name": "Brawn",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Brawn_GP"
    },
    {
        "constructor_ref": "matra-ford",
        "name": "Matra-Ford",
        "nationality": "French",
        "url": "http://en.wikipedia.org/wiki/Matra"
    },
    {
        "constructor_ref": "lotus-ford",
        "name": "Lotus-Ford",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Team_Lotus"
    },
    {
        "constructor_ref": "brabham-repco",
        "name": "Brabham-Repco",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Brabham"
    },
    {
        "constructor_ref": "cooper-climax",
        "name": "Cooper-Climax",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Cooper_Car_Company"
    },
    {
        "constructor_ref": "vanwall",
        "name": "Vanwall",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Vanwall"
    },
    {
        "constructor_ref": "lotus-climax",
        "name": "Lotus-Climax",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/Team_Lotus"
    },
    {
        "constructor_ref": "brm",
        "name": "BRM",
        "nationality": "British",
        "url": "http://en.wikipedia.org/wiki/BRM"
    },
    {
        "constructor_ref": "red_bull",
        "name": "Red Bull",
        "nationality": "Austrian",
        "url": "http://en.wikipedia.org/wiki/Red_Bull_Racing"
    },
    {
        "constructor_ref": "mercedes",
        "name": "Mercedes",
        "nationality": "German",
        "url": "http://en.wikipedia.org/wiki/Mercedes-Benz_in_Formula_One"
    }
];
