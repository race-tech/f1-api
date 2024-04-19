#[macro_export]
macro_rules! __circuits_impl {
    (@circuit
        "circuit_ref": $ref:literal,
        "name": $name:literal,
        $("location": $location:literal,)?
        $("country": $country:literal,)?
        $("lat": $lat:expr,)?
        $("lng": $lng:expr,)?
        $("alt": $alt:expr,)?
        "url": $url:literal
    ) => {
        common::models::StaticCircuit {
            circuit_ref: $ref,
            name: $name,
            location: [$(Some($location), )? None][0],
            country: [$(Some($country), )? None][0],
            lat: [$(Some($lat), )? None][0],
            lng: [$(Some($lng), )? None][0],
            alt: [$(Some($alt), )? None][0],
            url: $url,
        }
    };
    (@internal [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@internal [$($expr:expr),*]; $(,)?{ $($fields:tt)* } $($tt:tt)*) => {
        __circuits_impl!(@internal [$($expr,)* __circuits_impl!(@circuit $($fields)*)]; $($tt)*)
    };
}

#[macro_export]
macro_rules! circuits_from_json {
    ($($tt:tt)*) => {
        __circuits_impl!(@internal []; $($tt)*)
    };
}

#[macro_export]
macro_rules! __constructor_standings_impl {
    (@standings [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@standings [$($expr:expr),*]; $(,)?{
        "points": $points:expr,
        "position": $position:expr,
        "position_text": $position_text:literal,
        "wins": $wins:expr,
        "constructor": { $($fields:tt)* }
    } $($tt:tt)*) => {
        __constructor_standings_impl!(@standings [$($expr,)* common::models::StaticInnerStanding::Constructor {
            points: $points,
            position: Some($position),
            position_text: Some($position_text),
            wins: $wins,
            constructor: __constructors_impl!(@constructor $($fields)*)
        }]; $($tt)*)
    };
    (@internal [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@internal [$($expr:expr),*]; $(,)?{
        "season": $season:expr,
        "round": $round:expr,
        "constructor_standings": [$($st:tt)*]
    } $($tt:tt)*) => {
        __constructor_standings_impl!(@internal [$($expr,)* common::models::StaticStanding::Constructor {
            season: $season,
            round: $round,
            constructor_standings: &__constructor_standings_impl![@standings []; $($st)*],
        }]; $($tt)*)
    };
}

#[macro_export]
macro_rules! constructor_standings_from_json {
    ($($tt:tt)*) => {
        __constructor_standings_impl!(@internal []; $($tt)*)
    };
}

#[macro_export]
macro_rules! __constructors_impl {
    (@constructor
        "constructor_ref": $ref:literal,
        "name": $name:literal,
        $("nationality": $nationality:literal,)?
        "url": $url:literal
    ) => {
        common::models::StaticConstructor {
            constructor_ref: $ref,
            name: $name,
            nationality: [$(Some($nationality), )? None][0],
            url: $url
        }
    };
    (@internal [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@internal [$($expr:expr),*]; $(,)?{ $($fields:tt)* } $($tt:tt)*) => {
        __constructors_impl!(@internal [$($expr,)* __constructors_impl!(@constructor $($fields)*)]; $($tt)*)
    }
}

#[macro_export]
macro_rules! constructors_from_json {
    ($($tt:tt)*) => {
        __constructors_impl!(@internal []; $($tt)*)
    };
}

#[macro_export]
macro_rules! __drivers_impl {
    (@driver
        "driver_ref": $ref:literal,
        $("number": $number:expr,)?
        $("code": $code:literal,)?
        "forename": $forename:literal,
        "surname": $surname:literal,
        $("dob": $dob:literal,)?
        $("nationality": $nationality:literal,)?
        "url": $url:literal
    ) => {
        common::models::StaticDriver {
            driver_ref: $ref,
            number: [$(Some($number), )? None][0],
            code: [$(Some($code), )? None][0],
            forename: $forename,
            surname: $surname,
            dob: [$(Some($dob), )? None][0],
            nationality: [$(Some($nationality), )? None][0],
            url: $url,
        }
    };
    (@internal [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@internal [$($expr:expr),*]; $(,)?{ $($fields:tt)* } $($tt:tt)*) => {
        __drivers_impl!(@internal [$($expr,)* __drivers_impl!(@driver $($fields)*)]; $($tt)*)
    };
}

#[macro_export]
macro_rules! drivers_from_json {
    ($($tt:tt)*) => {
        __drivers_impl!(@internal []; $($tt)*)
    };
}

#[macro_export]
macro_rules! __driver_standings_impl {
    (@standings [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@standings [$($expr:expr),*]; $(,)?{
        "points": $points:expr,
        "position": $position:expr,
        "position_text": $position_text:literal,
        "wins": $wins:expr,
        "driver": { $($fields:tt)* }
    } $($tt:tt)*) => {
        __driver_standings_impl!(@standings [$($expr,)* common::models::StaticInnerStanding::Driver {
            points: $points,
            position: Some($position),
            position_text: Some($position_text),
            wins: $wins,
            driver: __drivers_impl!(@driver $($fields)*)
        }]; $($tt)*)
    };
    (@internal [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@internal [$($expr:expr),*]; $(,)?{
        "season": $season:expr,
        "round": $round:expr,
        "driver_standings": [$($st:tt)*]
    } $($tt:tt)*) => {
        __driver_standings_impl!(@internal [$($expr,)* common::models::StaticStanding::Driver {
            season: $season,
            round: $round,
            driver_standings: &__driver_standings_impl![@standings []; $($st)*],
        }]; $($tt)*)
    };
}

#[macro_export]
macro_rules! driver_standings_from_json {
    ($($tt:tt)*) => {
        __driver_standings_impl!(@internal []; $($tt)*)
    };
}

#[macro_export]
macro_rules! __laps_impl {
    (@timings [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@timings [$($expr:expr),*]; $(,)?{
        "driver_ref": $ref:literal,
        $("position": $position:expr,)?
        $("time": $time:literal)?
    } $($tt:tt)*) => {
        __laps_impl!(@timings [$($expr,)* common::models::StaticTiming {
            driver_ref: $ref,
            position: [$(Some($position), )? None][0],
            time: [$(Some($time), )? None][0],
        }]; $($tt)*)
    };
    (@laps [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@laps [$($expr:expr),*]; $(,)?{
        "number": $number:expr,
        "timings": [$($tt:tt)*]
    } $($lt:tt)*) => {
        __laps_impl!(@laps [$($expr,)* common::models::StaticLap {
            number: $number,
            timings: &__laps_impl![@timings []; $($tt)*]
        }]; $($lt)*)
    };
    (@internal {
        $("url": $url:literal,)?
        "race_name": $name:literal,
        "date": $date:literal,
        $("time": $time:literal,)?
        "circuit": { $($ct:tt)* },
        "laps": [$($lt:tt)*]
    }) => {
        common::models::StaticLaps {
            url: [$(Some($url), )? None][0],
            race_name: $name,
            date: $date,
            time: [$(Some($time), )? None][0],
            circuit: __circuits_impl!(@circuit $($ct)*),
            laps: &__laps_impl!(@laps []; $($lt)*),
        }
    };
}

#[macro_export]
macro_rules! laps_from_json {
    ($($tt:tt)*) => {
        __laps_impl!(@internal $($tt)*)
    };
}

#[macro_export]
macro_rules! __stops_impl {
    (@stops [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@stops [$($expr:expr),*]; $(,)?{
        "driver_ref": $ref:literal,
        "lap": $lap:expr,
        "stop": $stop:expr,
        "time": $time:literal,
        $("duration": $duration:literal)?
    } $($tt:tt)*) => {
        __stops_impl!(@stops [$($expr,)* common::models::StaticPitStop {
            driver_ref: $ref,
            lap: $lap,
            stop: $stop,
            time: $time,
            duration: [$(Some($duration), )? None][0]
        }]; $($tt)*)
    };
    (@internal {
        $("url": $url:literal,)?
        "race_name": $name:literal,
        "date": $date:literal,
        $("time": $time:literal,)?
        "circuit": { $($ct:tt)* },
        "pit_stops": [$($tt:tt)*]
    }) => {
        common::models::StaticPitStops {
            url: [$(Some($url), )? None][0],
            race_name: $name,
            date: $date,
            time: [$(Some($time), )? None][0],
            circuit: __circuits_impl!(@circuit $($ct)*),
            pit_stops: &__stops_impl!(@stops []; $($tt)*),
        }
    };
}

#[macro_export]
macro_rules! stops_from_json {
    ($($tt:tt)*) => {
        __stops_impl!(@internal $($tt)*)
    };
}

#[macro_export]
macro_rules! __races_impl {
    (@dateandtime
        "date": $date:literal,
        "time": $time:literal
    ) => {
        common::models::StaticDateAndTime {
            date: $date,
            time: $time
        }
    };
    (@race
        "season": $season:expr,
        "round": $round:expr,
        "name": $name:literal,
        "date": $date:literal,
        $("time": $time:literal,)?
        $("url": $url:literal,)?
        $("fp1": { $($fp1:tt)* },)?
        $("fp2": { $($fp2:tt)* },)?
        $("fp3": { $($fp3:tt)* },)?
        $("quali": { $($quali:tt)* },)?
        $("sprint": { $($sprint:tt)* },)?
        "circuit": { $($circuit:tt)* }
    ) => {
        common::models::StaticRace {
            season: $season,
            round: $round,
            name: $name,
            date: $date,
            time: [$(Some($time), )? None][0],
            url: [$(Some($url), )? None][0],
            fp1: [$(Some( __races_impl!(@dateandtime $($fp1)*) ), )? None][0],
            fp2: [$(Some( __races_impl!(@dateandtime $($fp2)*) ), )? None][0],
            fp3: [$(Some( __races_impl!(@dateandtime $($fp3)*) ), )? None][0],
            quali: [$(Some( __races_impl!(@dateandtime $($quali)*) ), )? None][0],
            sprint: [$(Some( __races_impl!(@dateandtime $($sprint)*) ), )? None][0],
            circuit: __circuits_impl!(@circuit $($circuit)*)
        }
    };
    (@internal [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@internal [$($expr:expr),*]; $(,)?{
        $($fields:tt)*
    } $($tt:tt)*) => {
        __races_impl!(@internal [$($expr,)* __races_impl!(@race $($fields)*)]; $($tt)*)
    }
}

#[macro_export]
macro_rules! races_from_json {
    ($($tt:tt)*) => {
        __races_impl!(@internal []; $($tt)*)
    };
}

#[macro_export]
macro_rules! __seasons_impl {
    (@season
        "year": $year:expr,
        "url": $url:literal
    ) => {
        common::models::StaticSeason {
            year: $year,
            url: $url
        }
    };
    (@internal [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@internal [$($expr:expr),*]; $(,)?{
        $($fields:tt)*
    } $($tt:tt)*) => {
        __seasons_impl!(@internal [$($expr,)* __seasons_impl!(@season $($fields)*)]; $($tt)*)
    }
}

#[macro_export]
macro_rules! seasons_from_json {
    ($($tt:tt)*) => {
        __seasons_impl!(@internal []; $($tt)*)
    };
}

#[macro_export]
macro_rules! __status_impl {
    (@status
        "status_id": $status_id:expr,
        "status": $status:literal,
        "count": $count:expr
    ) => {
        common::models::StaticStatus {
            status_id: $status_id,
            status: $status,
            count: $count
        }
    };
    (@internal [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@internal [$($expr:expr),*]; $(,)?{
        $($fields:tt)*
    } $($tt:tt)*) => {
        __status_impl!(@internal [$($expr,)* __status_impl!(@status $($fields)*)]; $($tt)*)
    }
}

#[macro_export]
macro_rules! status_from_json {
    ($($tt:tt)*) => {
        __status_impl!(@internal []; $($tt)*)
    };
}
