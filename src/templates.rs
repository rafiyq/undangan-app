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
    days: u32,
    hours: u32,
    minutes: u32,
    seconds: u32
}

impl Remaining {
    pub fn from_timedelta(timedelta: TimeDelta) -> Self {
        Remaining {
            days: timedelta.num_days() as u32,
            hours: (timedelta.num_hours() % 24) as u32,
            minutes: (timedelta.num_minutes() % 60) as u32,
            seconds: (timedelta.num_seconds() % 60) as u32
        }

    }
    pub fn is_timeout(&self) -> bool {
        self.days == 0 && 
        self.hours == 0 && 
        self.minutes == 0 && 
        self.seconds == 0
    }
}

impl Display for Remaining {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} Day(s), {}:{}:{}",
            self.days, self.hours, self.minutes, self.seconds)
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub countdown_ongoing: bool,
    pub remaining: Remaining
}

#[derive(Template)]
#[template(path = "countdown.html")]
pub struct Countdown {
    pub remaining: Remaining
}