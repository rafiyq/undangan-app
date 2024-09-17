use chrono::{DateTime, TimeDelta, Utc};

pub fn make_duration(dt: &str) -> TimeDelta {
    DateTime::parse_from_rfc3339(dt).unwrap()
        .with_timezone(&Utc)
        .signed_duration_since(Utc::now())
}

pub fn make_duration_from_str(dt: &str) -> TimeDelta {
    let end_dt = DateTime::parse_from_str(dt, "%v %T %z").unwrap().to_utc();
    end_dt - Utc::now()
}