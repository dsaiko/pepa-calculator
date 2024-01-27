use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{Decimal, string};
use crate::units_length::LengthUnit;
use crate::units_mass::MassUnit;
use crate::units_temperature::TemperatureUnit;
use crate::units_time::TimeUnit;

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Hash)]
pub enum Unit {
    Temperature(TemperatureUnit),
    Time(TimeUnit),
    Length(LengthUnit),
    Mass(MassUnit),
}

pub struct Abbreviations {
    pub case_sensitive: HashMap<String, Unit>,
    pub case_insensitive: HashMap<String, Unit>,
}

impl Unit {
    fn abbreviations(&self) -> Abbreviations {
        match self {
            Unit::Temperature(_) => TemperatureUnit::abbreviations(),
            Unit::Time(_) => TimeUnit::abbreviations(),
            Unit::Length(_) => LengthUnit::abbreviations(),
            Unit::Mass(_) => MassUnit::abbreviations(),
        }
    }

    pub fn from_string(name: &str) -> Vec<Unit> {
        let mut res = Vec::new();

        for u in Unit::iter() {
            let abbreviations = u.abbreviations();
            if let Some(u) = abbreviations.case_sensitive.get(&string!(name)) {
                res.push(*u);
            }
        }

        if !res.is_empty() {
            return res;
        }

        for u in Unit::iter() {
            let abbreviations = u.abbreviations();
            if let Some(u) = abbreviations.case_insensitive.get(&name.to_lowercase()) {
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
            Unit::Mass(from) => match to {
                Unit::Mass(to) => {
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
            Unit::Mass(m) => m.to_string_with_plural(n),
        }
    }
}
