use rust_decimal_macros::dec;
use std::collections::HashMap;
use strum_macros::EnumIter;

use crate::{make_abbreviations, string, Decimal, Unit};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Default)]
pub enum TemperatureUnit {
    DegreesCelsius,
    DegreesFahrenheit,
    #[default]
    Kelvin,
}

impl TemperatureUnit {
    pub fn abbreviations() -> HashMap<String, Unit> {
        let mut abbreviations = HashMap::new();

        for t in TemperatureUnit::iter() {
            abbreviations.extend(match t {
                TemperatureUnit::DegreesCelsius => make_abbreviations!(
                    t.to_unit(),
                    "Celsius",
                    "°C",
                    "DegreesCelsius",
                    "DegreeCelsius"
                ),
                TemperatureUnit::DegreesFahrenheit => make_abbreviations!(
                    t.to_unit(),
                    "Fahrenheit",
                    "Fahrenheits",
                    "°F",
                    "DegreesFahrenheit",
                    "DegreeFahrenheit",
                    "DegreesFahrenheits",
                    "DegreeFahrenheits"
                ),
                TemperatureUnit::Kelvin => make_abbreviations!(t.to_unit(), "Kelvin", "Kelvins"),
            });
        }

        abbreviations
    }

    pub fn to_reference_unit(self, v: Decimal) -> Decimal {
        match self {
            TemperatureUnit::DegreesCelsius => v + dec!(273.15),
            TemperatureUnit::DegreesFahrenheit => {
                dec!(273.15) + (v - dec!(32.0)) * (dec!(5.0) / dec!(9.0))
            }
            TemperatureUnit::Kelvin => v,
        }
    }

    pub fn from_reference_unit(self, v: Decimal) -> Decimal {
        match self {
            TemperatureUnit::DegreesCelsius => v - dec!(273.15),
            TemperatureUnit::DegreesFahrenheit => {
                (v - dec!(273.15)) * (dec!(9.0) / dec!(5.0)) + dec!(32.0)
            }
            TemperatureUnit::Kelvin => v,
        }
    }

    pub fn to_string_with_plural(self, _: &Decimal) -> String {
        match self {
            TemperatureUnit::DegreesCelsius => string!("°C"),
            TemperatureUnit::DegreesFahrenheit => string!("°F"),
            TemperatureUnit::Kelvin => string!("K"),
        }
    }

    pub fn to_unit(self) -> Unit {
        Unit::Temperature(self)
    }
}
