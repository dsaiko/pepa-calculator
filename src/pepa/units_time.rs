use std::collections::HashMap;

use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    Decimal, make_abbreviations, make_abbreviations_with_prefixes, string, Unit, UnitPrefix,
};
use crate::units::Abbreviations;

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Hash)]
pub enum TimeUnit {
    Second(Option<UnitPrefix>),
    Minute,
    Hour,
    Day,
}

impl TimeUnit {
    pub fn abbreviations() -> Abbreviations {
        let mut case_sensitive = HashMap::new();
        let mut case_insensitive = HashMap::new();

        for t in TimeUnit::iter() {
            match t {
                TimeUnit::Second(_) => {
                    case_sensitive.extend(make_abbreviations_with_prefixes!(
                        TimeUnit::Second,
                        // case sensitive
                        "s"
                    ));

                    case_insensitive.extend(make_abbreviations_with_prefixes!(
                        TimeUnit::Second,
                        // case insensitive
                        "second",
                        "seconds",
                        "sec",
                        "secs"
                    ));
                }
                TimeUnit::Minute => {
                    case_sensitive.extend(make_abbreviations!(
                        t.to_unit(),
                        // case sensitive
                        "m"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        t.to_unit(),
                        // case insensitive
                        "minute",
                        "minutes",
                        "min",
                        "mins"
                    ));
                }
                TimeUnit::Hour => {
                    case_sensitive.extend(make_abbreviations!(
                        t.to_unit(),
                        // case sensitive
                        "h"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        t.to_unit(),
                        // case insensitive
                        "hours",
                        "hour",
                        "hrs"
                    ));
                }
                TimeUnit::Day => {
                    case_sensitive.extend(make_abbreviations!(
                        t.to_unit(),
                        // case sensitive
                        "d"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        t.to_unit(),
                        // case insensitive
                        "d",
                        "day",
                        "days"
                    ));
                }
            };
        }

        Abbreviations {
            case_sensitive,
            case_insensitive,
        }
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
