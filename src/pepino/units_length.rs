use std::fmt::Display;

use crate::unit_prefixes::UnitPrefix;

#[derive(Debug, Clone, Eq, Copy, PartialEq)]
pub enum LengthUnit {
    Meter(Option<UnitPrefix>),

    // micron
    // mm ... ???
    AstronomicalUnit,
    LightYear,
    Parsec(Option<UnitPrefix>),

    Thou,
    Barleycorn,
    Inch,
    Foot,
    Yard,
    Mile,
    League,
    Pole,
    Furlong,
    Chain,

    Fathom,
    NauticalMile,
}

impl LengthUnit {
    //     pub fn abbreviations(self) -> Vec<&'static str> {
    //         match self {
    //             LengthUnit::Meter(None) => "meter",
    //             LengthUnit::Meter(Some(p)) => (p.to_string() + "meter").as_str(),
    //             LengthUnit::AstronomicalUnit => {}
    //             LengthUnit::LightYear => {}
    //             LengthUnit::Parsec => {}
    //             LengthUnit::Thou => {}
    //             LengthUnit::Barleycorn => {}
    //             LengthUnit::Inch => {}
    //             LengthUnit::Foot => {}
    //             LengthUnit::Yard => {}
    //             LengthUnit::Mile => {}
    //             LengthUnit::League => {}
    //             LengthUnit::Pole => {}
    //             LengthUnit::Furlong => {}
    //             LengthUnit::Chain => {}
    //             LengthUnit::Fathom => {}
    //             LengthUnit::NauticalMile => {}
    //         }
    //     }
    //
    // pub fn to_reference_unit(self, v: Decimal) -> Decimal {
    //     match self {
    //         LengthUnit::Meter(None) => v,
    //         LengthUnit::Meter(Some(p)) => v * p.multiplier(),
    //         LengthUnit::AstronomicalUnit => v * dec!(149_597_870_700),
    //         LengthUnit::LightYear => v * dec!(9_460_730_472_580_800),
    //         LengthUnit::Parsec(None) => dec!(648000) / Decimal::PI * dec!(149_597_870_700), // 648000/π * au
    //         LengthUnit::Parsec(Some(p)) => {
    //             LengthUnit::Parsec(None).to_reference_unit(v) * p.multiplier()
    //         }
    //         LengthUnit::Thou => {}
    //         LengthUnit::Barleycorn => {}
    //         LengthUnit::Inch => {}
    //         LengthUnit::Foot => {}
    //         LengthUnit::Yard => {}
    //         LengthUnit::Mile => {}
    //         LengthUnit::League => {}
    //         LengthUnit::Pole => {}
    //         LengthUnit::Furlong => {}
    //         LengthUnit::Chain => {}
    //         LengthUnit::Fathom => {}
    //         LengthUnit::NauticalMile => {}
    //     }
    // }

    //
    //     pub fn from_reference_unit(self, v: f64) -> f64 {
    //         match self {
    //             LengthUnit::Meter(None) => v,
    //             LengthUnit::Meter(Some(p)) => (p.to_string() + "meter").as_str(),
    //             LengthUnit::AstronomicalUnit => {}
    //             LengthUnit::LightYear => {}
    //             LengthUnit::Parsec => {}
    //             LengthUnit::Thou => {}
    //             LengthUnit::Barleycorn => {}
    //             LengthUnit::Inch => {}
    //             LengthUnit::Foot => {}
    //             LengthUnit::Yard => {}
    //             LengthUnit::Mile => {}
    //             LengthUnit::League => {}
    //             LengthUnit::Pole => {}
    //             LengthUnit::Furlong => {}
    //             LengthUnit::Chain => {}
    //             LengthUnit::Fathom => {}
    //             LengthUnit::NauticalMile => {}
    //         }
    //     }
    // }
    //
    // impl Display for LengthUnit {
    //     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    //         write!(
    //             f,
    //             "{}",
    //             match self {
    //                 LengthUnit::Meter(None) => "meter",
    //                 LengthUnit::Meter(Some(p)) => (p.to_string() + "meter").as_str(),
    //                 LengthUnit::AstronomicalUnit => "au",
    //                 LengthUnit::LightYear => "ly",
    //                 LengthUnit::Parsec => "parsec",
    //                 LengthUnit::Thou => "thou",
    //                 LengthUnit::Barleycorn => "barleycorn",
    //                 LengthUnit::Inch => "inch",
    //                 LengthUnit::Foot => "foot",
    //                 LengthUnit::Yard => "yard",
    //                 LengthUnit::Mile => "mile",
    //                 LengthUnit::League => "league",
    //                 LengthUnit::Pole => "pole",
    //                 LengthUnit::Furlong => "furlong",
    //                 LengthUnit::Chain => "chain",
    //                 LengthUnit::Fathom => "fathom",
    //                 LengthUnit::NauticalMile => "nautical mile",
    //             }
    //         )
    //     }
}
