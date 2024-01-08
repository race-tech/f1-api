mod signalr;

pub use signalr::SignalrConn;

const BASE_URL: &str = "https://livetiming.formula1.com";

static PAGES: &[(&str, &str)] = &[
    ("session_data", "SessionData"),
    ("session_info", "SessionInfo"),
    ("archive_status", "ArchiveStatus"),
    ("heartbeats", "Heartbeats"),
    ("audio_streams", "AudioStreams"),
    ("driver_list", "DriverList"),
    ("extrapolated_clock", "ExtrapolatedClock"), // Boolean
    ("race_control_messages", "RaceControlMessages"), // Flags etc
    ("session_status", "SessionStatus"),         // Start and finish times
    ("team_radio", "TeamRadio"),                 // Links to team radios
    ("timing_app_data", "TimingAppData"),        // Tyres and laps (juicy)
    ("timing_stats", "TimingStats"),             // "Best times/speed" useless
    ("track_status", "TrackStatus"),             // SC, VSC and Yellow
    ("weather_data", "WeatherData"),             // Temp, wind and rain
    ("position", "Position.z"),                  // Coordinates, not GPS? (.z)
    ("car_data", "CarData.z"),                   // Telemetry channels (.z)
    ("content_streams", "ContentStreams"),       // Lap by lap feeds
    ("timing_data", "TimingData"),               // Gap to car ahead
    ("lap_count", "LapCount"),                   // Lap counter
    ("championship_prediction", "ChampionshipPrediction"), // Points
    ("index", "Index"),
];

// Use binary search to access the map:
fn get_page(key: &str) -> &str {
    PAGES
        .binary_search_by(|(k, _)| k.cmp(&key))
        .map(|x| PAGES[x].1)
        .expect("page not found")
}
