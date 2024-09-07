use chrono::{DateTime, TimeDelta, Utc};

pub fn make_duration(dt: &str) -> TimeDelta {
    DateTime::parse_from_rfc3339(dt).unwrap()
        .with_timezone(&Utc)
        .signed_duration_since(Utc::now())
}