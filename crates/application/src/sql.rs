pub struct GenericParams {
    year: Option<i32>,
    round: Option<i32>,
    driver_ref: Option<String>,
    constructor_ref: Option<String>,
    status: Option<i32>,
    grid: Option<i32>,
    position: Option<i32>,
    fastest: Option<i32>,
    result: Option<i32>,
    driver_standings: Option<i32>,
    constructor_standings: Option<i32>,
}

impl GenericParams {
    fn one_of(&self) -> bool {
        self.year.is_some()
            || self.round.is_some()
            || self.driver_ref.is_some()
            || self.constructor_ref.is_some()
            || self.status.is_some()
            || self.grid.is_some()
            || self.position.is_some()
            || self.fastest.is_some()
            || self.result.is_some()
            || self.driver_standings.is_some()
            || self.constructor_standings.is_some()
    }
}

pub fn append_race_table(params: &GenericParams) -> &str {
    if params.one_of() {
        ", races"
    } else {
        ""
    }
}

pub fn append_results_table(params: &GenericParams) -> &str {
    if params.one_of() {
        ", results"
    } else {
        ""
    }
}

pub fn append_drivers_table(params: &GenericParams) -> &str {
    if params.one_of() {
        ", drivers"
    } else {
        ""
    }
}

pub fn append_constructors_table(params: &GenericParams) -> &str {
    if params.one_of() {
        ", constructors"
    } else {
        ""
    }
}

pub fn and_circuits(params: &GenericParams) -> &str {
    if params.one_of() {
        " AND races.circuitId=circuits.circuitId"
    } else {
        ""
    }
}

pub fn and_races(params: &GenericParams) -> &str {
    if params.one_of() {
        " AND results.raceId=races.raceId"
    } else {
        ""
    }
}

pub fn and_constructors(params: &GenericParams) -> String {
    if params.constructor_ref.is_some() {
        format!(" AND results.constructorId=constructors.constructorId AND constructors.constructorRef={}", params.constructor_ref.as_ref().unwrap())
    } else {
        "".to_string()
    }
}

pub fn and_drivers(params: &GenericParams) -> String {
    if params.driver_ref.is_some() {
        format!(
            " AND results.driverId=drivers.driverId AND drivers.driverRef='{}'",
            params.driver_ref.as_ref().unwrap()
        )
    } else {
        "".to_string()
    }
}

impl From<shared::parameters::GetCircuitsQueryParams> for GenericParams {
    fn from(value: shared::parameters::GetCircuitsQueryParams) -> Self {
        Self {
            year: value.year,
            round: value.round,
            driver_ref: value.driver_ref,
            constructor_ref: value.constructor_ref,
            status: value.status,
            grid: value.grid,
            position: value.result,
            fastest: value.fastest,
            result: value.result,
            driver_standings: None,
            constructor_standings: None,
        }
    }
}
