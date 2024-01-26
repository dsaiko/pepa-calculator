use std::collections::HashMap;

use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    make_abbreviations, make_abbreviations_with_prefixes, string, Decimal, Unit, UnitPrefix,
};

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Hash)]
pub enum TimeUnit {
    Second(Option<UnitPrefix>),
    Minute,
    Hour,
    Day,
}

impl TimeUnit {
    pub fn abbreviations() -> HashMap<String, Unit> {
        let mut abbreviations = HashMap::new();

        for t in TimeUnit::iter() {
            abbreviations.extend(match t {
                TimeUnit::Second(_) => {
                    make_abbreviations_with_prefixes!(
                        TimeUnit::Second,
                        "s",
                        "second",
                        "seconds",
                        "sec",
                        "secs"
                    )
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
            TimeUnit::Second(None) => dec!(1),
            TimeUnit::Second(Some(p)) => p.multiplier(),
            TimeUnit::Minute => dec!(60.0),
            TimeUnit::Hour => dec!(60.0) * dec!(60.0),
            TimeUnit::Day => dec!(24.0) * dec!(60.0) * dec!(60.0),
        }
    }

    pub fn to_string_with_plural(self, _: &Decimal) -> String {
        match self {
            TimeUnit::Second(None) => string!("s"),
            TimeUnit::Second(Some(p)) => string!(p) + "s",
            TimeUnit::Minute => string!("m"),
            TimeUnit::Hour => string!("h"),
            TimeUnit::Day => string!("d"),
        }
    }

    pub fn to_unit(self) -> Unit {
        Unit::Time(self)
    }
}

impl Default for TimeUnit {
    fn default() -> Self {
        TimeUnit::Second(None)
    }
}
