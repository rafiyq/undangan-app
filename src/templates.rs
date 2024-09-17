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

pub struct DayAndTime {
    days: u32,
    hours: u32,
    minutes: u32,
    seconds: u32
}

impl DayAndTime {
    pub fn from_timedelta(timedelta: TimeDelta) -> Self {
        DayAndTime {
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

impl Display for DayAndTime {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}-{}:{}:{}",
            self.days, self.hours, self.minutes, self.seconds)
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub countdown_ongoing: bool,
    pub countdown_remaining: DayAndTime
}

#[derive(Template)]
#[template(path = "countdown.html")]
pub struct Countdown {
    pub countdown_remaining: DayAndTime
}