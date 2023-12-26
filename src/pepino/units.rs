use strum_macros::Display;

#[derive(Debug, Clone, Eq, Copy, PartialEq, Display)]
pub enum TemperatureUnits {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug, Clone, Eq, Copy, PartialEq, Display)]
pub enum Unit {
    Temperature(TemperatureUnits),
}
