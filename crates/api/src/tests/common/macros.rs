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
        StaticCircuit {
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
        $crate::__circuits_impl!(@internal [$($expr,)* $crate::__circuits_impl!(@circuit $($fields)*)]; $($tt)*)
    };
}

#[macro_export]
macro_rules! circuits_from_json {
    ($($tt:tt)*) => {
        $crate::__circuits_impl!(@internal []; $($tt)*)
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
        $crate::__constructor_standings_impl!(@standings [$($expr,)* StaticInnerStanding::Constructor {
            points: $points,
            position: Some($position),
            position_text: Some($position_text),
            wins: $wins,
            constructor: $crate::__constructors_impl!(@constructor $($fields)*)
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
        $crate::__constructor_standings_impl!(@internal [$($expr,)* StaticStanding::Constructor {
            season: $season,
            round: $round,
            constructor_standings: &$crate::__constructor_standings_impl![@standings []; $($st)*],
        }]; $($tt)*)
    };
}

#[macro_export]
macro_rules! constructor_standings_from_json {
    ($($tt:tt)*) => {
        $crate::__constructor_standings_impl!(@internal []; $($tt)*)
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
        StaticConstructor {
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
        $crate::__constructors_impl!(@internal [$($expr,)* $crate::__constructors_impl!(@constructor $($fields)*)]; $($tt)*)
    }
}

#[macro_export]
macro_rules! constructors_from_json {
    ($($tt:tt)*) => {
        $crate::__constructors_impl!(@internal []; $($tt)*)
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
        StaticDriver {
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
        $crate::__drivers_impl!(@internal [$($expr,)* $crate::__drivers_impl!(@driver $($fields)*)]; $($tt)*)
    };
}

#[macro_export]
macro_rules! drivers_from_json {
    ($($tt:tt)*) => {
        $crate::__drivers_impl!(@internal []; $($tt)*)
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
        $crate::__driver_standings_impl!(@standings [$($expr,)* StaticInnerStanding::Driver {
            points: $points,
            position: Some($position),
            position_text: Some($position_text),
            wins: $wins,
            driver: $crate::__drivers_impl!(@driver $($fields)*)
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
        $crate::__driver_standings_impl!(@internal [$($expr,)* StaticStanding::Driver {
            season: $season,
            round: $round,
            driver_standings: &$crate::__driver_standings_impl![@standings []; $($st)*],
        }]; $($tt)*)
    };
}

#[macro_export]
macro_rules! driver_standings_from_json {
    ($($tt:tt)*) => {
        $crate::__driver_standings_impl!(@internal []; $($tt)*)
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
        $crate::__laps_impl!(@timings [$($expr,)* StaticTiming {
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
        $crate::__laps_impl!(@laps [$($expr,)* StaticLap {
            number: $number,
            timings: &$crate::__laps_impl![@timings []; $($tt)*]
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
        StaticLaps {
            url: [$(Some($url), )? None][0],
            race_name: $name,
            date: $date,
            time: [$(Some($time), )? None][0],
            circuit: $crate::__circuits_impl!(@circuit $($ct)*),
            laps: &$crate::__laps_impl!(@laps []; $($lt)*),
        }
    };
}

#[macro_export]
macro_rules! laps_from_json {
    ($($tt:tt)*) => {
        $crate::__laps_impl!(@internal $($tt)*)
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
        $crate::__stops_impl!(@stops [$($expr,)* StaticPitStop {
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
        StaticPitStops {
            url: [$(Some($url), )? None][0],
            race_name: $name,
            date: $date,
            time: [$(Some($time), )? None][0],
            circuit: $crate::__circuits_impl!(@circuit $($ct)*),
            pit_stops: &$crate::__stops_impl!(@stops []; $($tt)*),
        }
    };
}

#[macro_export]
macro_rules! stops_from_json {
    ($($tt:tt)*) => {
        $crate::__stops_impl!(@internal $($tt)*)
    };
}

#[macro_export]
macro_rules! __races_impl {
    (@dateandtime
        "date": $date:literal,
        "time": $time:literal
    ) => {
        StaticDateAndTime {
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
        StaticRace {
            season: $season,
            round: $round,
            name: $name,
            date: $date,
            time: [$(Some($time), )? None][0],
            url: [$(Some($url), )? None][0],
            fp1: [$(Some( $crate::__races_impl!(@dateandtime $($fp1)*) ), )? None][0],
            fp2: [$(Some( $crate::__races_impl!(@dateandtime $($fp2)*) ), )? None][0],
            fp3: [$(Some( $crate::__races_impl!(@dateandtime $($fp3)*) ), )? None][0],
            quali: [$(Some( $crate::__races_impl!(@dateandtime $($quali)*) ), )? None][0],
            sprint: [$(Some( $crate::__races_impl!(@dateandtime $($sprint)*) ), )? None][0],
            circuit: $crate::__circuits_impl!(@circuit $($circuit)*)
        }
    };
    (@internal [$($expr:expr),*];) => {
        [$($expr),*]
    };
    (@internal [$($expr:expr),*]; $(,)?{
        $($fields:tt)*
    } $($tt:tt)*) => {
        $crate::__races_impl!(@internal [$($expr,)* $crate::__races_impl!(@race $($fields)*)]; $($tt)*)
    }
}

#[macro_export]
macro_rules! races_from_json {
    ($($tt:tt)*) => {
        $crate::__races_impl!(@internal []; $($tt)*)
    };
}

#[macro_export]
macro_rules! __seasons_impl {
    (@season
        "year": $year:expr,
        "url": $url:literal
    ) => {
        StaticSeason {
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
        $crate::__seasons_impl!(@internal [$($expr,)* $crate::__seasons_impl!(@season $($fields)*)]; $($tt)*)
    }
}

#[macro_export]
macro_rules! seasons_from_json {
    ($($tt:tt)*) => {
        $crate::__seasons_impl!(@internal []; $($tt)*)
    };
}

#[macro_export]
macro_rules! __status_impl {
    (@status
        "status_id": $status_id:expr,
        "status": $status:literal,
        "count": $count:expr
    ) => {
        StaticStatus {
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
        $crate::__status_impl!(@internal [$($expr,)* $crate::__status_impl!(@status $($fields)*)]; $($tt)*)
    }
}

#[macro_export]
macro_rules! status_from_json {
    ($($tt:tt)*) => {
        $crate::__status_impl!(@internal []; $($tt)*)
    };
}
