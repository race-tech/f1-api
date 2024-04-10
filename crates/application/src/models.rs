diesel::table! {
    circuits (circuit_id) {
        #[sql_name = "circuitId"]
        circuit_id -> Integer,
        #[sql_name = "circuitRef"]
        #[max_length = 255]
        circuit_ref -> Varchar,
        #[sql_name = "circuitName"]
        #[max_length = 255]
        circuit_name -> Varchar,
        #[sql_name = "circuitLocation"]
        #[max_length = 255]
        circuit_location -> Nullable<Varchar>,
        #[sql_name = "circuitCountry"]
        #[max_length = 255]
        country -> Nullable<Varchar>,
        #[sql_name = "circuitLat"]
        lat -> Nullable<Float>,
        #[sql_name = "circuitLng"]
        lng -> Nullable<Float>,
        #[sql_name = "circuitAlt"]
        alt -> Nullable<Integer>,
        #[sql_name = "circuitUrl"]
        #[max_length = 255]
        url -> Varchar,
    }
}
