use chrono::prelude::*;

#[derive(Default, Debug)]
pub struct Standing {
    year: usize,
    drivers: Vec<Driver>,
    constructors: Vec<Constructor>,
}

impl Standing {
    pub fn get_current() -> Standing {
        let current_year = Self::get_current_f1_year();
        let mut standing = Standing {
            year: current_year,
            drivers: Vec::new(),
            constructors: Vec::new(),
        };

        // Fetch the driver standing
        standing.get_driver_standing(current_year);
        // Fetch the constructor standing
        standing.get_constructor_standing(current_year);

        standing
    }

    fn get_current_f1_year() -> usize {
        let current_year = Utc::now().year() as usize;

        let response =
            reqwest::blocking::get(&format!("{}/{}/drivers.html", BASE_URL, current_year));
        let content = response.unwrap().text().unwrap();

        let document = scraper::Html::parse_document(&content);

        match document
            .select(&scraper::Selector::parse("table.resultsarchive-table").unwrap())
            .next()
        {
            Some(_) => current_year,
            None => current_year - 1,
        }
    }

    fn get_driver_standing(&mut self, year: usize) {
        let response = reqwest::blocking::get(&format!("{}/{}/drivers.html", BASE_URL, year));
        let content = response.unwrap().text().unwrap();

        let document = scraper::Html::parse_document(&content);
        let selector = scraper::Selector::parse("table.resultsarchive-table > tbody > tr").unwrap();
        let row_selector = scraper::Selector::parse("td").unwrap();

        for element in document.select(&selector) {
            let mut cells = element.select(&row_selector).skip(1);

            let position = cells
                .next()
                .map(|p| p.inner_html().parse::<u8>().expect("invalid position"))
                .unwrap();
            let names = cells
                .next()
                .expect("invalid row, no name found")
                .select(&scraper::Selector::parse("a > span").unwrap())
                .map(|e| e.inner_html())
                .collect::<Vec<_>>();
            let nationality = cells.next().expect("no nationality found").inner_html();
            let team = cells
                .next()
                .expect("no team found")
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .expect("no team name found")
                .inner_html();
            let points = cells
                .next()
                .map(|p| p.inner_html().parse::<u16>().expect("invalid points"))
                .unwrap();

            self.drivers.push(Driver::new(
                position,
                names[..2].join(" "),
                names[2].to_string(),
                nationality,
                team,
                points,
            ));
        }
    }

    fn get_constructor_standing(&mut self, year: usize) {
        let response = reqwest::blocking::get(&format!("{}/{}/team.html", BASE_URL, year));
        let content = response.unwrap().text().unwrap();

        let document = scraper::Html::parse_document(&content);
        let selector = scraper::Selector::parse("table.resultsarchive-table > tbody > tr").unwrap();
        let row_selector = scraper::Selector::parse("td").unwrap();

        for element in document.select(&selector) {
            let mut cells = element.select(&row_selector).skip(1);

            let position = cells
                .next()
                .map(|p| p.inner_html().parse::<u8>().expect("invalid position"))
                .unwrap();
            let name = cells
                .next()
                .expect("invalid row, no name found")
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .expect("no name found")
                .inner_html();
            let points = cells
                .next()
                .map(|p| p.inner_html().parse::<u16>().expect("invalid points"))
                .unwrap();

            self.constructors
                .push(Constructor::new(position, name, points));
        }
    }
}

#[derive(Default, Debug)]
struct Driver {
    position: u8,
    name: String,
    short: String,
    nationality: String,
    team: String,
    points: u16,
}

impl Driver {
    fn new(
        position: u8,
        name: String,
        short: String,
        nationality: String,
        team: String,
        points: u16,
    ) -> Driver {
        Driver {
            position,
            name,
            short,
            nationality,
            team,
            points,
        }
    }
}

#[derive(Default, Debug)]
struct Constructor {
    position: u8,
    name: String,
    points: u16,
}

impl Constructor {
    fn new(position: u8, name: String, points: u16) -> Constructor {
        Constructor {
            position,
            name,
            points,
        }
    }
}

const BASE_URL: &str = "https://www.formula1.com/en/results.html";
