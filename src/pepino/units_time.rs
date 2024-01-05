use std::fmt::{Display, Formatter};

use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter)]
pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
}

impl TimeUnit {
    pub fn abbreviations(self) -> Vec<&'static str> {
        match self {
            TimeUnit::Second => vec!["second", "seconds", "sec", "secs"],
            TimeUnit::Minute => vec!["minutes", "minutes", "min", "mins"],
            TimeUnit::Hour => vec!["hours", "hour", "hrs"],
            TimeUnit::Day => vec!["day", "days"],
        }
    }

    pub fn to_reference_unit(self, v: f64) -> f64 {
        match self {
            TimeUnit::Second => v,
            TimeUnit::Minute => v * 60.0,
            TimeUnit::Hour => v * 60.0 * 60.0,
            TimeUnit::Day => v * 24.0 * 60.0 * 60.0,
        }
    }

    pub fn from_reference_unit(self, v: f64) -> f64 {
        match self {
            TimeUnit::Second => v,
            TimeUnit::Minute => v / 60.0,
            TimeUnit::Hour => v / 60.0 / 60.0,
            TimeUnit::Day => v / 60.0 / 60.0 / 24.0,
        }
    }
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TimeUnit::Second => "seconds",
                TimeUnit::Minute => "minutes",
                TimeUnit::Hour => "hours",
                TimeUnit::Day => "days",
            }
        )
    }
}
