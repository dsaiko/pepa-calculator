use rust_decimal_macros::dec;
use strum_macros::EnumIter;

use crate::{string, Decimal};

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter)]
pub enum TemperatureUnit {
    DegreesCelsius,
    DegreesFahrenheit,
    Kelvin,
}

impl TemperatureUnit {
    pub fn abbreviations(self) -> Vec<&'static str> {
        match self {
            TemperatureUnit::DegreesCelsius => {
                vec!["Celsius", "°C", "DegreesCelsius", "DegreeCelsius"]
            }
            TemperatureUnit::DegreesFahrenheit => {
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
            TemperatureUnit::DegreesCelsius => v,
            TemperatureUnit::DegreesFahrenheit => (v - dec!(32.0)) * (dec!(5.0) / dec!(9.0)),
            TemperatureUnit::Kelvin => v - dec!(273.15),
        }
    }

    pub fn from_reference_unit(self, v: Decimal) -> Decimal {
        match self {
            TemperatureUnit::DegreesCelsius => v,
            TemperatureUnit::DegreesFahrenheit => v * (dec!(9.0) / dec!(5.0)) + dec!(32.0),
            TemperatureUnit::Kelvin => v + dec!(273.15),
        }
    }

    pub fn to_string(self, _: Decimal) -> String {
        match self {
            TemperatureUnit::DegreesCelsius => string!("°C"),
            TemperatureUnit::DegreesFahrenheit => string!("°F"),
            TemperatureUnit::Kelvin => string!("K"),
        }
    }
}
