use async_graphql::*;

#[derive(Debug, SimpleObject)]
pub struct DateAndTime {
    pub date: time::Date,
    pub time: String,
}

#[derive(Debug, SimpleObject)]
pub struct Race {
    pub season: i32,
    pub round: i32,
    pub name: String,
    pub date: time::Date,
    pub time: Option<String>,
    pub url: String,
    pub fp1: Option<DateAndTime>,
    pub fp2: Option<DateAndTime>,
    pub fp3: Option<DateAndTime>,
    pub quali: Option<DateAndTime>,
    pub sprint: Option<DateAndTime>,
}

impl From<super::Race> for Race {
    fn from(v: super::Race) -> Self {
        Race {
            season: v.year,
            round: v.round,
            name: v.name,
            date: v.race_date,
            time: v.race_time.map(|t| format!("{}", t)),
            url: v.url,
            fp1: v.fp1_date.map(|date| DateAndTime {
                date,
                time: format!("{}", v.fp1_time.unwrap()),
            }),
            fp2: v.fp2_date.map(|date| DateAndTime {
                date,
                time: format!("{}", v.fp2_time.unwrap()),
            }),
            fp3: v.fp3_date.map(|date| DateAndTime {
                date,
                time: format!("{}", v.fp3_time.unwrap()),
            }),
            quali: v.quali_date.map(|date| DateAndTime {
                date,
                time: format!("{}", v.quali_time.unwrap()),
            }),
            sprint: v.sprint_date.map(|date| DateAndTime {
                date,
                time: format!("{}", v.sprint_time.unwrap()),
            }),
        }
    }
}
