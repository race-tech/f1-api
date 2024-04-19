use shared::prelude::*;

#[derive(Debug)]
pub struct StaticCircuit<'a> {
    pub circuit_ref: &'a str,
    pub name: &'a str,
    pub location: Option<&'a str>,
    pub country: Option<&'a str>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub alt: Option<i32>,
    pub url: &'a str,
}

#[derive(Debug)]
pub enum StaticStanding<'a> {
    Constructor {
        season: i32,
        round: i32,
        constructor_standings: &'a [StaticInnerStanding<'a>],
    },
    Driver {
        season: i32,
        round: i32,
        driver_standings: &'a [StaticInnerStanding<'a>],
    },
}

#[derive(Debug)]
pub enum StaticInnerStanding<'a> {
    Constructor {
        points: f32,
        position: Option<i32>,
        position_text: Option<&'a str>,
        wins: i32,
        constructor: StaticConstructor<'a>,
    },
    Driver {
        points: f32,
        position: Option<i32>,
        position_text: Option<&'a str>,
        wins: i32,
        driver: StaticDriver<'a>,
    },
}

#[derive(Debug)]
pub struct StaticConstructor<'a> {
    pub constructor_ref: &'a str,
    pub name: &'a str,
    pub nationality: Option<&'a str>,
    pub url: &'a str,
}

#[derive(Debug)]
pub struct StaticDriver<'a> {
    pub driver_ref: &'a str,
    pub number: Option<i32>,
    pub code: Option<&'a str>,
    pub forename: &'a str,
    pub surname: &'a str,
    pub dob: Option<&'a str>,
    pub nationality: Option<&'a str>,
    pub url: &'a str,
}

#[derive(Debug)]
pub struct StaticLaps<'a> {
    pub url: Option<&'a str>,
    pub race_name: &'a str,
    pub date: &'a str,
    pub time: Option<&'a str>,
    pub circuit: StaticCircuit<'a>,
    pub laps: &'a [StaticLap<'a>],
}

#[derive(Debug)]
pub struct StaticLap<'a> {
    pub number: i32,
    pub timings: &'a [StaticTiming<'a>],
}

#[derive(Debug)]
pub struct StaticTiming<'a> {
    pub driver_ref: &'a str,
    pub position: Option<i32>,
    pub time: Option<&'a str>,
}

#[derive(Debug)]
pub struct StaticPitStops<'a> {
    pub url: Option<&'a str>,
    pub race_name: &'a str,
    pub date: &'a str,
    pub time: Option<&'a str>,
    pub circuit: StaticCircuit<'a>,
    pub pit_stops: &'a [StaticPitStop<'a>],
}

#[derive(Debug)]
pub struct StaticPitStop<'a> {
    pub driver_ref: &'a str,
    pub lap: i32,
    pub stop: i32,
    pub time: &'a str,
    pub duration: Option<&'a str>,
}

#[derive(Debug)]
pub struct StaticRace<'a> {
    pub season: i32,
    pub round: i32,
    pub name: &'a str,
    pub date: &'a str,
    pub time: Option<&'a str>,
    pub url: Option<&'a str>,
    pub fp1: Option<StaticDateAndTime<'a>>,
    pub fp2: Option<StaticDateAndTime<'a>>,
    pub fp3: Option<StaticDateAndTime<'a>>,
    pub quali: Option<StaticDateAndTime<'a>>,
    pub sprint: Option<StaticDateAndTime<'a>>,

    pub circuit: StaticCircuit<'a>,
}

#[derive(Debug, Clone, Copy)]
pub struct StaticDateAndTime<'a> {
    pub date: &'a str,
    pub time: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct StaticSeason<'a> {
    pub year: i32,
    pub url: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct StaticStatus<'a> {
    pub status_id: i32,
    pub status: &'a str,
    pub count: i32,
}

fn compare<T, U>(t: Option<T>, u: Option<U>) -> bool
where
    T: PartialEq<U>,
{
    match (t, u) {
        (Some(t), Some(u)) => t == u,
        (None, None) => true,
        _ => false,
    }
}

impl PartialEq<Status> for StaticStatus<'_> {
    fn eq(&self, other: &Status) -> bool {
        self.status_id == other.status_id
            && self.status == other.status
            && self.count == other.count
    }
}

impl PartialEq<Season> for StaticSeason<'_> {
    fn eq(&self, other: &Season) -> bool {
        self.year == other.year && self.url == other.url
    }
}

impl PartialEq<RaceResponse> for StaticRace<'_> {
    fn eq(&self, other: &RaceResponse) -> bool {
        let circuit = &other.circuit;
        let other = &other.race;

        self.season == other.season
            && self.round == other.round
            && self.name == other.name
            && super::parse_date(self.date) == other.date
            && self.time.map(super::parse_time) == other.time
            && self.url == other.url.as_deref()
            && compare(self.fp1.as_ref(), other.fp1.as_ref())
            && compare(self.fp2.as_ref(), other.fp2.as_ref())
            && compare(self.fp3.as_ref(), other.fp3.as_ref())
            && compare(self.quali.as_ref(), other.quali.as_ref())
            && compare(self.sprint.as_ref(), other.sprint.as_ref())
            && self.circuit == *circuit
    }
}

impl PartialEq<DateAndTime> for StaticDateAndTime<'_> {
    fn eq(&self, other: &DateAndTime) -> bool {
        let date = super::parse_date(self.date);
        let time = super::parse_time(self.time);

        date == other.date && time == other.time
    }
}

impl PartialEq<PitStopsResponse> for StaticPitStops<'_> {
    fn eq(&self, other: &PitStopsResponse) -> bool {
        let date = chrono::NaiveDate::parse_from_str(self.date, "%Y-%m-%d").unwrap();
        let time = self
            .time
            .map(|t| chrono::NaiveTime::parse_from_str(t, "%H:%M:%S").unwrap());

        self.url == other.url.as_deref()
            && self.race_name == other.race_name
            && date == other.date
            && time == other.time
            && self.circuit == other.circuit
            && self.pit_stops == other.pit_stops
    }
}

impl PartialEq<PitStop> for StaticPitStop<'_> {
    fn eq(&self, other: &PitStop) -> bool {
        let time = chrono::NaiveTime::parse_from_str(self.time, "%H:%M:%S").unwrap();

        self.driver_ref == other.driver_ref
            && self.lap == other.lap
            && self.stop == other.stop
            && time == other.time
            && self.duration == other.duration.as_deref()
    }
}

impl PartialEq<LapsResponse> for StaticLaps<'_> {
    fn eq(&self, other: &LapsResponse) -> bool {
        let date = chrono::NaiveDate::parse_from_str(self.date, "%Y-%m-%d").unwrap();
        let time = self
            .time
            .map(|t| chrono::NaiveTime::parse_from_str(t, "%H:%M:%S").unwrap());

        self.url == other.url.as_deref()
            && self.race_name == other.race_name
            && date == other.date
            && time == other.time
            && self.circuit == other.circuit
            && self.laps == other.laps
    }
}

impl PartialEq<Lap> for StaticLap<'_> {
    fn eq(&self, other: &Lap) -> bool {
        self.number == other.number && self.timings == other.timings
    }
}

impl PartialEq<LapTiming> for StaticTiming<'_> {
    fn eq(&self, other: &LapTiming) -> bool {
        self.driver_ref == other.driver_ref
            && self.position == other.position
            && self.time == other.time.as_deref()
    }
}

impl PartialEq<Driver> for StaticDriver<'_> {
    fn eq(&self, other: &Driver) -> bool {
        self.driver_ref.eq(&other.driver_ref)
            && self.number.eq(&other.number)
            && self.code.eq(&other.code.as_deref())
            && self.forename.eq(&other.forename)
            && self.surname.eq(&other.surname)
            && self
                .dob
                .map(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").expect("invalid date"))
                .eq(&other.dob)
            && self.nationality.eq(&other.nationality.as_deref())
            && self.url.eq(&other.url)
    }
}

impl PartialEq<InnerStandingResponse> for StaticStanding<'_> {
    fn eq(&self, other: &InnerStandingResponse) -> bool {
        match (self, other) {
            (
                StaticStanding::Constructor {
                    season: self_season,
                    round: self_round,
                    constructor_standings: self_standings,
                },
                InnerStandingResponse::Constructor {
                    season,
                    round,
                    constructor_standings,
                },
            ) => {
                self_season == season
                    && self_round == round
                    && self_standings == constructor_standings
            }
            (
                StaticStanding::Driver {
                    season: self_season,
                    round: self_round,
                    driver_standings: self_standings,
                },
                InnerStandingResponse::Driver {
                    season,
                    round,
                    driver_standings,
                },
            ) => self_season == season && self_round == round && self_standings == driver_standings,
            _ => false,
        }
    }
}

impl PartialEq<Standings> for StaticInnerStanding<'_> {
    fn eq(&self, other: &Standings) -> bool {
        match (self, other) {
            (
                Self::Constructor {
                    points,
                    position,
                    position_text,
                    wins,
                    constructor: self_constructor,
                },
                Standings::Constructor {
                    standing,
                    constructor,
                },
            ) => {
                *points == standing.points
                    && position == &standing.position
                    && *position_text == standing.position_text.as_deref()
                    && *wins == standing.wins
                    && self_constructor == constructor
            }
            (
                Self::Driver {
                    points,
                    position,
                    position_text,
                    wins,
                    driver: self_driver,
                },
                Standings::Driver { standing, driver },
            ) => {
                *points == standing.points
                    && position == &standing.position
                    && *position_text == standing.position_text.as_deref()
                    && *wins == standing.wins
                    && self_driver == driver
            }
            _ => false,
        }
    }
}

impl PartialEq<Constructor> for StaticConstructor<'_> {
    fn eq(&self, other: &Constructor) -> bool {
        self.constructor_ref == other.constructor_ref
            && self.name == other.name
            && self.nationality == other.nationality.as_deref()
            && self.url == other.url
    }
}

impl PartialEq<Circuit> for StaticCircuit<'_> {
    fn eq(&self, other: &Circuit) -> bool {
        self.circuit_ref.eq(&other.circuit_ref)
            && self.name.eq(&other.name)
            && self.location.eq(&other.location.as_deref())
            && self.country.eq(&other.country.as_deref())
            && self.alt.eq(&other.alt)
            && self.lat.eq(&other.lat)
            && self.lng.eq(&other.lng)
            && self.url.eq(&other.url)
    }
}

impl PartialEq<&Circuit> for StaticCircuit<'_> {
    fn eq(&self, other: &&Circuit) -> bool {
        self.circuit_ref.eq(&other.circuit_ref)
            && self.name.eq(&other.name)
            && self.location.eq(&other.location.as_deref())
            && self.country.eq(&other.country.as_deref())
            && self.alt.eq(&other.alt)
            && self.lat.eq(&other.lat)
            && self.lng.eq(&other.lng)
            && self.url.eq(&other.url)
    }
}
