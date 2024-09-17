use std::fmt::{self, Formatter, Display};
use askama::Template;
use chrono::TimeDelta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Guest {
	id: u32,
	username: String,
	fullname: String,
	information: String,
}

pub struct Remaining {
    remaining: TimeDelta
}

impl Remaining {
    pub fn from_timedelta(timedelta: TimeDelta) -> Self {
        Remaining {
            remaining: timedelta
        }
    }
    pub fn is_timeout(&self) -> bool {
        self.remaining.is_zero()
    }
    pub fn is_ongoing(&self) -> bool {
        !self.is_timeout()
    }
    pub fn days(&self) -> String {
        format!("{:02}", self.remaining.num_days())
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
    pub remaining: Remaining
}

#[derive(Template)]
#[template(path = "countdown.html")]
pub struct Countdown {
    pub remaining: Remaining
}