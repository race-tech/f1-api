use sea_query::MysqlQueryBuilder;
use shared::models::graphql::GetCircuitsOpts;

use crate::{circuit::CircuitQueryBuilder, SqlBuilder};

macro_rules! test_with_builder {
    ($builder:expr, $query:expr) => {{
        let mut builder = $builder;
        let query = builder.stmt().to_string(MysqlQueryBuilder);
        assert_eq!(query, $query);
    }};
}

const BASE_SELECT: &str = "SELECT DISTINCT `circuits`.`circuitId`, `circuits`.`circuitRef`, `circuits`.`name`, `circuits`.`location`, `circuits`.`country`, `circuits`.`lat`, `circuits`.`lng`, `circuits`.`alt`, `circuits`.`url` FROM `circuits`";
const ORDER_BY: &str = "ORDER BY `circuits`.`circuitRef` ASC";

#[test]
fn test_circuit() {
    test_with_builder!(
        CircuitQueryBuilder::circuit("monza"),
        format!("{} WHERE `circuits`.`circuitRef` = 'monza'", BASE_SELECT)
    )
}

#[test]
fn test_circuits_driver_ref() {
    let params = GetCircuitsOpts {
        driver_ref: Some("leclerc".to_string()),
        ..Default::default()
    };
    test_with_builder!(
        CircuitQueryBuilder::circuits(params),
        format!("{}, `races`, `results`, `drivers` WHERE `circuits`.`circuitId` = `races`.`circuitId` AND `results`.`raceId` = `races`.`raceId` AND (`drivers`.`driverId` = `results`.`driverId` AND `drivers`.`driverRef` = 'leclerc') {}", BASE_SELECT, ORDER_BY)
    );
}

#[test]
fn test_circuits_constructor_ref() {
    let params = GetCircuitsOpts {
        constructor_ref: Some("ferrari".to_string()),
        ..Default::default()
    };
    test_with_builder!(
        CircuitQueryBuilder::circuits(params),
        format!("{}, `races`, `results`, `constructors` WHERE `circuits`.`circuitId` = `races`.`circuitId` AND `results`.`raceId` = `races`.`raceId` AND (`constructors`.`constructorId` = `results`.`constructorId` AND `constructors`.`constructorRef` = 'ferrari') {}", BASE_SELECT, ORDER_BY)
        );
}

#[test]
fn test_circuits_driver_and_constructor_ref() {
    let params = GetCircuitsOpts {
        driver_ref: Some("leclerc".into()),
        constructor_ref: Some("ferrari".into()),
        ..Default::default()
    };
    test_with_builder!(
        CircuitQueryBuilder::circuits(params),
        format!("{}, `races`, `results`, `drivers`, `constructors` WHERE `circuits`.`circuitId` = `races`.`circuitId` AND `results`.`raceId` = `races`.`raceId` AND (`constructors`.`constructorId` = `results`.`constructorId` AND `constructors`.`constructorRef` = 'ferrari') AND (`drivers`.`driverId` = `results`.`driverId` AND `drivers`.`driverRef` = 'leclerc') {}", BASE_SELECT, ORDER_BY)
    );
}

#[test]
fn test_circuits_grid() {
    let params = GetCircuitsOpts {
        grid: Some(1),
        ..Default::default()
    };
    test_with_builder!(
        CircuitQueryBuilder::circuits(params),
        format!("{}, `races`, `results` WHERE `circuits`.`circuitId` = `races`.`circuitId` AND `results`.`raceId` = `races`.`raceId` AND `results`.`grid` = 1 {}", BASE_SELECT, ORDER_BY)
    )
}

#[test]
fn test_driver_ref_and_result() {
    let params = GetCircuitsOpts {
        driver_ref: Some("leclerc".into()),
        result: Some(1),
        ..Default::default()
    };
    test_with_builder!(
        CircuitQueryBuilder::circuits(params),
        format!("{}, `races`, `results`, `drivers` WHERE `circuits`.`circuitId` = `races`.`circuitId` AND `results`.`raceId` = `races`.`raceId` AND (`drivers`.`driverId` = `results`.`driverId` AND `drivers`.`driverRef` = 'leclerc') AND `results`.`positionText` = 1 {}", BASE_SELECT, ORDER_BY)
    )
}

#[test]
fn test_circuits_status() {
    let params = GetCircuitsOpts {
        status: Some(1),
        ..Default::default()
    };
    test_with_builder!(
        CircuitQueryBuilder::circuits(params),
        format!("{}, `races`, `results` WHERE `circuits`.`circuitId` = `races`.`circuitId` AND `results`.`raceId` = `races`.`raceId` AND `results`.`statusId` = 1 {}", BASE_SELECT, ORDER_BY)
    )
}
