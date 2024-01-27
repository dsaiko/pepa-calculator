use std::collections::HashMap;

use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{Decimal, make_abbreviations, string, Unit};
use crate::units::Abbreviations;

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Default, Hash)]
pub enum TemperatureUnit {
    DegreesCelsius,
    DegreesFahrenheit,
    #[default]
    Kelvin,
}

impl TemperatureUnit {
    pub fn abbreviations() -> Abbreviations {
        let mut case_sensitive = HashMap::new();
        let mut case_insensitive = HashMap::new();

        for t in TemperatureUnit::iter() {
            match t {
                TemperatureUnit::DegreesCelsius => {
                    case_insensitive.extend(make_abbreviations!(
                        t.to_unit(),
                        // case insensitive
                        "°c",
                        "celsius",
                        "degreescelsius",
                        "degreecelsius"
                    ));
                }
                TemperatureUnit::DegreesFahrenheit => {
                    case_insensitive.extend(make_abbreviations!(
                        t.to_unit(),
                        // case insensitive
                        "°f",
                        "fahrenheit",
                        "fahrenheits",
                        "degreesfahrenheit",
                        "degreefahrenheit",
                        "degreesfahrenheits",
                        "degreefahrenheits"
                    ));
                }
                TemperatureUnit::Kelvin => {
                    case_sensitive.extend(make_abbreviations!(
                        t.to_unit(),
                        // case sensitive
                        "K"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        t.to_unit(),
                        // case insensitive
                        "kelvin",
                        "kelvins"
                    ));
                }
            };
        }

        Abbreviations {
            case_sensitive,
            case_insensitive,
        }
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
