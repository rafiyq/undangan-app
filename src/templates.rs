use std::fmt::{self, Formatter, Display};
use askama::Template;
use chrono::{DateTime, Local, TimeDelta};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Guest {
	username: String,
	fullname: String,
    title: String,
	address: String,
    session: String
}

pub struct Remaining {
    remaining: TimeDelta,
    _end_date: String
}

impl Remaining {
    pub fn _new(remaining: TimeDelta) -> Self {
        Remaining { remaining, _end_date: String::new() }
    }
    pub fn from_rfc3339(datetime: &str) -> Self {
        let remaining = DateTime::parse_from_rfc3339(datetime).unwrap()
            .signed_duration_since(Local::now());
        Remaining { remaining, _end_date: datetime.to_string() }
    }
    pub fn _update(mut self) {
        self.remaining = DateTime::parse_from_rfc3339(self._end_date.as_str()).unwrap()
            .signed_duration_since(Local::now());
    }
    pub fn is_timeout(&self) -> bool {
        self.remaining.num_seconds() < 0
    }
    pub fn is_ongoing(&self) -> bool {
        !self.is_timeout()
    }
    pub fn days(&self) -> i64 {
        self.remaining.num_days()
    }
    pub fn hours(&self) -> String {
        format!("{:02}", self.remaining.num_hours() % 24)
    }
    pub fn minutes(&self) -> String {
        format!("{:02}", self.remaining.num_minutes() % 60)
    }
    pub fn seconds(&self) -> String {
        format!("{:02}", self.remaining.num_seconds() % 60)
    }
}

impl Display for Remaining {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} Day(s), {}:{}:{}",
            self.days(), self.hours(), self.minutes(), self.seconds())
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub guest: Option<Guest>,
    pub remaining: Remaining
}

#[derive(Template)]
#[template(path = "components/countdown.html")]
pub struct Countdown {
    pub remaining: Remaining
}