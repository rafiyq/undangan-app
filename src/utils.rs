use chrono::{DateTime, TimeDelta, Utc};

pub fn make_duration(dt: &str) -> TimeDelta {
    DateTime::parse_from_rfc3339(dt).unwrap()
        .with_timezone(&Utc)
        .signed_duration_since(Utc::now())
}

pub fn _make_duration_from_str(dt: &str) -> TimeDelta {
    let end_dt = DateTime::parse_from_str(dt, "%v %T %z").unwrap().to_utc();
    end_dt - Utc::now()
}

pub fn get_content_type(filename: &str) -> &'static str {
    let ext = filename.split('.').last().unwrap_or("");
    match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        _ => "application/octet-stream",
    }
}