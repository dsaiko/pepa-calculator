use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::OnceLock;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::units_length::LengthUnit;
use crate::units_temperature::TemperatureUnit;
use crate::units_time::TimeUnit;
use crate::Decimal;

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter)]
pub enum Unit {
    Temperature(TemperatureUnit),
    Time(TimeUnit),
    Length(LengthUnit),
}

impl Unit {
    fn abbreviations(&self) -> HashMap<String, Unit> {
        match self {
            Unit::Temperature(_) => TemperatureUnit::abbreviations(),
            Unit::Time(_) => TimeUnit::abbreviations(),
            Unit::Length(_) => LengthUnit::abbreviations(),
        }
    }

    pub fn from_string(name: &str) -> Vec<Unit> {
        let mut res = Vec::new();

        for u in Unit::iter() {
            let abbreviations = u.abbreviations();
            if let Some(u) = abbreviations.get(name.to_lowercase().as_str()) {
                res.push(*u);
            }
        }

        res
    }

    pub fn conversion(&self, v: &Decimal, to: &Unit) -> Option<Decimal> {
        match self {
            Unit::Temperature(from) => match to {
                Unit::Temperature(to) => Some(to.from_reference_unit(from.to_reference_unit(*v))),
                _ => None,
            },
            Unit::Time(from) => match to {
                Unit::Time(to) => {
                    Some(v * from.reference_unit_multiplier() / to.reference_unit_multiplier())
                }
                _ => None,
            },
            Unit::Length(from) => match to {
                Unit::Length(to) => {
                    Some(v * from.reference_unit_multiplier() / to.reference_unit_multiplier())
                }
                _ => None,
            },
        }
    }

    pub fn to_string_with_plural(&self, n: &Decimal) -> String {
        match self {
            Unit::Temperature(t) => t.to_string_with_plural(n),
            Unit::Time(t) => t.to_string_with_plural(n),
            Unit::Length(l) => l.to_string_with_plural(n),
        }
    }
}
