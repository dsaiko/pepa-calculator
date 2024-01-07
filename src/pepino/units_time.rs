use std::fmt::{Display, Formatter};

use rust_decimal_macros::dec;
use strum_macros::EnumIter;

use crate::Decimal;

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

    pub fn to_reference_unit(self, v: Decimal) -> Decimal {
        match self {
            TimeUnit::Second => v,
            TimeUnit::Minute => v * dec!(60.0),
            TimeUnit::Hour => v * dec!(60.0) * dec!(60.0),
            TimeUnit::Day => v * dec!(24.0) * dec!(60.0) * dec!(60.0),
        }
    }

    pub fn from_reference_unit(self, v: Decimal) -> Decimal {
        match self {
            TimeUnit::Second => v,
            TimeUnit::Minute => v / dec!(60.0),
            TimeUnit::Hour => v / dec!(60.0) / dec!(60.0),
            TimeUnit::Day => v / dec!(60.0) / dec!(60.0) / dec!(24.0),
        }
    }
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TimeUnit::Second => "second",
                TimeUnit::Minute => "minute",
                TimeUnit::Hour => "hour",
                TimeUnit::Day => "day",
            }
        )
    }
}
