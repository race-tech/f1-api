use diesel::prelude::*;

mod database;
mod livetiming;
mod standing;

fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = diesel::MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    use database::constructor_standing::ConstructorStanding;
    use database::race::Race;

    let races: Vec<Race> = Race::by_year(2023).load(&mut conn).unwrap();

    let standings = ConstructorStanding::by_race_id(races.last().unwrap().race_id)
        .load(&mut conn)
        .unwrap();

    for s in standings {
        println!("{:?}", s.points);
    }
}
