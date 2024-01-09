use std::collections::HashMap;
use std::sync::OnceLock;

use strum::IntoEnumIterator;

use crate::units_temperature::TemperatureUnit;
use crate::units_time::TimeUnit;
use crate::Decimal;

#[derive(Debug, Clone, Eq, Copy, PartialEq)]
pub enum Unit {
    Temperature(TemperatureUnit),
    Time(TimeUnit),
}

pub fn abbreviations() -> &'static HashMap<String, Unit> {
    static MEM: OnceLock<HashMap<String, Unit>> = OnceLock::new();
    MEM.get_or_init(|| {
        let mut abbreviations = HashMap::new();

        for t in TemperatureUnit::iter() {
            let abb = t.abbreviations();
            let unit = Unit::Temperature(t);
            for abbreviation in abb {
                abbreviations.insert(abbreviation.to_lowercase(), unit);
            }
        }

        for t in TimeUnit::iter() {
            let abb = t.abbreviations();
            let unit = Unit::Time(t);
            for abbreviation in abb {
                abbreviations.insert(abbreviation.to_lowercase(), unit);
            }
        }

        abbreviations
    })
}

impl Unit {
    pub fn from_string(name: &str) -> Option<&Unit> {
        abbreviations().get(name.to_lowercase().as_str())
    }

    pub fn conversion(&self, v: Decimal, to: &Unit) -> Option<Decimal> {
        match self {
            Unit::Temperature(from) => match to {
                Unit::Temperature(to) => Some(to.from_reference_unit(from.to_reference_unit(v))),
                _ => None,
            },
            Unit::Time(from) => match to {
                Unit::Time(to) => Some(to.from_reference_unit(from.to_reference_unit(v))),
                _ => None,
            },
        }
    }

    pub fn to_string(&self, v: Decimal) -> String {
        // TODO TRAIT
        match self {
            Unit::Temperature(t) => t.to_string(v),
            Unit::Time(t) => t.to_string(v),
        }
    }
}
