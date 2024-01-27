use std::collections::HashMap;

use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::units::{Abbreviations, Unit};
use crate::{make_abbreviations, string, Decimal};

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Default, Hash)]
pub enum Temperature {
    DegreesCelsius,
    DegreesFahrenheit,
    #[default]
    Kelvin,
}

impl Temperature {
    pub fn abbreviations() -> Abbreviations {
        let mut case_sensitive = HashMap::new();
        let mut case_insensitive = HashMap::new();

        for t in Temperature::iter() {
            match t {
                Temperature::DegreesCelsius => {
                    case_insensitive.extend(make_abbreviations!(
                        t.to_unit(),
                        // case insensitive
                        "°c",
                        "celsius",
                        "degreescelsius",
                        "degreecelsius"
                    ));
                }
                Temperature::DegreesFahrenheit => {
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
                Temperature::Kelvin => {
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
            Temperature::DegreesCelsius => v + dec!(273.15),
            Temperature::DegreesFahrenheit => {
                dec!(273.15) + (v - dec!(32.0)) * (dec!(5.0) / dec!(9.0))
            }
            Temperature::Kelvin => v,
        }
    }

    pub fn from_reference_unit(self, v: Decimal) -> Decimal {
        match self {
            Temperature::DegreesCelsius => v - dec!(273.15),
            Temperature::DegreesFahrenheit => {
                (v - dec!(273.15)) * (dec!(9.0) / dec!(5.0)) + dec!(32.0)
            }
            Temperature::Kelvin => v,
        }
    }

    pub fn to_string_with_plural(self, _: &Decimal) -> String {
        match self {
            Temperature::DegreesCelsius => string!("°C"),
            Temperature::DegreesFahrenheit => string!("°F"),
            Temperature::Kelvin => string!("K"),
        }
    }

    pub fn to_unit(self) -> Unit {
        Unit::Temperature(self)
    }
}
