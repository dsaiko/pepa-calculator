use std::collections::HashMap;

use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::units::{Abbreviations, Prefix, Unit};
use crate::{make_abbreviations, make_abbreviations_with_prefixes, string, Decimal};

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Hash)]
pub enum Time {
    Second(Option<Prefix>),
    Minute,
    Hour,
    Day,
}

impl Time {
    pub fn abbreviations() -> Abbreviations {
        let mut case_sensitive = HashMap::new();
        let mut case_insensitive = HashMap::new();

        for t in Time::iter() {
            match t {
                Time::Second(_) => {
                    case_sensitive.extend(make_abbreviations_with_prefixes!(
                        Time::Second,
                        // case sensitive
                        "s"
                    ));

                    case_insensitive.extend(make_abbreviations_with_prefixes!(
                        Time::Second,
                        // case insensitive
                        "second",
                        "seconds",
                        "sec",
                        "secs"
                    ));
                }
                Time::Minute => {
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
                Time::Hour => {
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
                Time::Day => {
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
            Time::Second(None) => dec!(1),
            Time::Second(Some(p)) => p.multiplier(),
            Time::Minute => dec!(60.0),
            Time::Hour => dec!(60.0) * dec!(60.0),
            Time::Day => dec!(24.0) * dec!(60.0) * dec!(60.0),
        }
    }

    pub fn to_string_with_plural(self, _: &Decimal) -> String {
        match self {
            Time::Second(None) => string!("s"),
            Time::Second(Some(p)) => string!(p) + "s",
            Time::Minute => string!("m"),
            Time::Hour => string!("h"),
            Time::Day => string!("d"),
        }
    }

    pub fn to_unit(self) -> Unit {
        Unit::Time(self)
    }
}

impl Default for Time {
    fn default() -> Self {
        Time::Second(None)
    }
}
