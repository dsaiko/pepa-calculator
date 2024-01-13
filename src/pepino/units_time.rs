use crate::utils::Pluralize;
use crate::{make_abbreviations, pluralize, string, Decimal, TemperatureUnit, Unit};
use rust_decimal_macros::dec;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Default)]
pub enum TimeUnit {
    #[default]
    Second,
    Minute,
    Hour,
    Day,
}

impl TimeUnit {
    pub fn abbreviations() -> HashMap<String, Unit> {
        let mut abbreviations = HashMap::new();

        for t in TimeUnit::iter() {
            abbreviations.extend(match t {
                TimeUnit::Second => {
                    make_abbreviations!(t.to_unit(), "s", "second", "seconds", "sec", "secs")
                }
                TimeUnit::Minute => {
                    make_abbreviations!(t.to_unit(), "m", "minute", "minutes", "min", "mins")
                }
                TimeUnit::Hour => make_abbreviations!(t.to_unit(), "h", "hours", "hour", "hrs"),
                TimeUnit::Day => make_abbreviations!(t.to_unit(), "d", "day", "days"),
            });
        }

        abbreviations
    }

    pub fn reference_unit_multiplier(self) -> Decimal {
        match self {
            TimeUnit::Second => dec!(1),
            TimeUnit::Minute => dec!(60.0),
            TimeUnit::Hour => dec!(60.0) * dec!(60.0),
            TimeUnit::Day => dec!(24.0) * dec!(60.0) * dec!(60.0),
        }
    }

    pub fn to_string_with_plural(self, _: &Decimal) -> String {
        match self {
            TimeUnit::Second => string!("s"),
            TimeUnit::Minute => string!("m"),
            TimeUnit::Hour => string!("h"),
            TimeUnit::Day => string!("d"),
        }
    }

    pub fn to_unit(self) -> Unit {
        Unit::Time(self)
    }
}
