use std::fmt::{Display, Formatter};

use strum_macros::EnumIter;

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

    pub fn to_reference_unit(self, v: f64) -> f64 {
        match self {
            TemperatureUnit::Celsius => v,
            TemperatureUnit::Fahrenheit => (v - 32.0) * (5.0 / 9.0),
            TemperatureUnit::Kelvin => v - 273.15,
        }
    }

    pub fn from_reference_unit(self, v: f64) -> f64 {
        match self {
            TemperatureUnit::Celsius => v,
            TemperatureUnit::Fahrenheit => v * (9.0 / 5.0) + 32.0,
            TemperatureUnit::Kelvin => v + 273.15,
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
