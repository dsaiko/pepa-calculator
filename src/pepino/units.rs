use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

use crate::units_temperature::TemperatureUnit;
use crate::units_time::TimeUnit;

#[derive(Debug, Clone, Eq, Copy, PartialEq)]
pub enum Unit {
    Temperature(TemperatureUnit),
    Time(TimeUnit),
}

static UNITS_ABBREVIATIONS: Lazy<HashMap<String, Unit>> = Lazy::new(|| {
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
});

impl Unit {
    pub fn from_string(name: &str) -> Option<&Unit> {
        UNITS_ABBREVIATIONS.get(name.to_lowercase().as_str())
    }

    pub fn conversion(&self, v: f64, to: &Unit) -> Option<f64> {
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
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Unit::Temperature(t) => t.to_string(),
                Unit::Time(t) => t.to_string(),
            }
        )
    }
}
