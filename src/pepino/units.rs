use strum_macros::Display;

#[derive(Debug, Clone, Eq, PartialEq, Display)]
pub enum TemperatureUnits {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug, Clone, Eq, PartialEq, Display)]
pub enum Unit {
    Temperature(TemperatureUnits),
}
