use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::unit_prefixes::UnitPrefix;
use crate::utils::Pluralize;
use crate::{pluralize, string};

#[derive(Debug, Clone, Eq, Copy, PartialEq)]
pub enum LengthUnit {
    Meter(Option<UnitPrefix>),

    AstronomicalUnit,
    LightYear,
    Parsec(Option<UnitPrefix>),

    Thou,
    Barleycorn,
    Inch,
    Finger,
    Foot,
    Yard,
    Mile,
    Pole,
    Rod,
    Furlong,
    Chain,
    Fathom,
    NauticalMile,
    League,
    NauticalLeague,
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

    pub fn to_reference_unit(self, v: Decimal) -> Decimal {
        match self {
            LengthUnit::Meter(None) => v,
            LengthUnit::Meter(Some(p)) => v * p.multiplier(),
            LengthUnit::AstronomicalUnit => v * dec!(149_597_870_700),
            LengthUnit::LightYear => v * dec!(9_460_730_472_580_800),
            LengthUnit::Parsec(None) => dec!(648000) / Decimal::PI * dec!(149_597_870_700), // 648000/π * au
            LengthUnit::Parsec(Some(p)) => {
                LengthUnit::Parsec(None).to_reference_unit(v) * p.multiplier()
            }
            LengthUnit::Thou => LengthUnit::Inch.to_reference_unit(v) / dec!(1000),
            LengthUnit::Barleycorn => LengthUnit::Inch.to_reference_unit(v) / dec!(3),
            LengthUnit::Inch => v * dec!(0.0254),
            LengthUnit::Foot => LengthUnit::Yard.to_reference_unit(v) / dec!(3),
            LengthUnit::Yard => v * dec!(0.9144),
            LengthUnit::Mile => LengthUnit::Foot.to_reference_unit(v) * dec!(5280),
            LengthUnit::Pole => LengthUnit::Foot.to_reference_unit(v) * dec!(16.5),
            LengthUnit::Rod => LengthUnit::Foot.to_reference_unit(v) * dec!(16.5),
            LengthUnit::Furlong => LengthUnit::Yard.to_reference_unit(v) / dec!(220),
            LengthUnit::Chain => LengthUnit::Yard.to_reference_unit(v) / dec!(22),
            LengthUnit::Fathom => LengthUnit::Foot.to_reference_unit(v) * dec!(6),
            LengthUnit::NauticalMile => LengthUnit::Foot.to_reference_unit(v) * dec!(6080),
            LengthUnit::Finger => LengthUnit::Inch.to_reference_unit(v) * dec!(7) / dec!(8),
            LengthUnit::League => LengthUnit::Mile.to_reference_unit(v) * dec!(3),
            LengthUnit::NauticalLeague => LengthUnit::NauticalMile.to_reference_unit(v) * dec!(3),
        }
    }

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

    pub fn to_string(self, v: Decimal) -> String {
        match self {
            LengthUnit::Meter(None) => string!("m"),
            LengthUnit::Meter(Some(p)) => p.to_string() + "m",
            LengthUnit::AstronomicalUnit => string!("au"),
            LengthUnit::LightYear => string!("ly"),
            LengthUnit::Parsec(None) => string!("pc"),
            LengthUnit::Parsec(Some(p)) => p.to_string() + "pc",
            LengthUnit::Thou => string!("th"),
            LengthUnit::Barleycorn => pluralize!("barleycorn", v),
            LengthUnit::Inch => string!("in"),
            LengthUnit::Foot => string!("ft"),
            LengthUnit::Yard => string!("yd"),
            LengthUnit::Mile => string!("mi"),
            LengthUnit::Pole => pluralize!("pole", v),
            LengthUnit::Rod => pluralize!("rod", v),
            LengthUnit::Furlong => string!("fur"),
            LengthUnit::Chain => string!("ch"),
            LengthUnit::Fathom => string!("ftm"),
            LengthUnit::NauticalMile => string!("NM"),
            LengthUnit::Finger => pluralize!("finger", v),
            LengthUnit::League => string!("lea"),
            LengthUnit::NauticalLeague => string!("NL"),
        }
    }
}
