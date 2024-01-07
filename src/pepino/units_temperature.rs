use std::fmt::{Display, Formatter};

use rust_decimal_macros::dec;
use strum_macros::EnumIter;

use crate::Decimal;

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl TemperatureUnit {
    pub fn abbreviations(self) -> Vec<&'static str> {
        match self {
            TemperatureUnit::Celsius => vec!["Celsius", "°C", "DegreesCelsius", "DegreeCelsius"],
            TemperatureUnit::Fahrenheit => {
                vec![
                    "Fahrenheit",
                    "Fahrenheits",
                    "°F",
                    "DegreesFahrenheit",
                    "DegreeFahrenheit",
                    "DegreesFahrenheits",
                    "DegreeFahrenheits",
                ]
            }
            TemperatureUnit::Kelvin => vec!["Kelvin", "Kelvins"],
        }
    }

    pub fn to_reference_unit(self, v: Decimal) -> Decimal {
        match self {
            TemperatureUnit::Celsius => v,
            TemperatureUnit::Fahrenheit => (v - dec!(32.0)) * (dec!(5.0) / dec!(9.0)),
            TemperatureUnit::Kelvin => v - dec!(273.15),
        }
    }

    pub fn from_reference_unit(self, v: Decimal) -> Decimal {
        match self {
            TemperatureUnit::Celsius => v,
            TemperatureUnit::Fahrenheit => v * (dec!(9.0) / dec!(5.0)) + dec!(32.0),
            TemperatureUnit::Kelvin => v + dec!(273.15),
        }
    }
}

impl Display for TemperatureUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TemperatureUnit::Celsius => "°C",
                TemperatureUnit::Fahrenheit => "°F",
                TemperatureUnit::Kelvin => "Kelvin",
            }
        )
    }
}
