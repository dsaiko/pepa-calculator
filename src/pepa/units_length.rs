use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::unit_prefixes::UnitPrefix;
use crate::utils::Pluralize;
use crate::{make_abbreviations, make_abbreviations_with_prefixes, pluralize, string, Unit};

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Hash)]
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
    pub fn to_string_with_plural(self, v: &Decimal) -> String {
        match self {
            LengthUnit::Meter(None) => string!("m"),
            LengthUnit::Meter(Some(p)) => string!(p) + "m",
            LengthUnit::AstronomicalUnit => string!("au"),
            LengthUnit::LightYear => string!("ly"),
            LengthUnit::Parsec(None) => string!("pc"),
            LengthUnit::Parsec(Some(p)) => string!(p) + "pc",
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

    pub fn abbreviations() -> HashMap<String, Unit> {
        let mut abbreviations = HashMap::new();

        for l in LengthUnit::iter() {
            abbreviations.extend(match l {
                LengthUnit::Meter(_) => {
                    make_abbreviations_with_prefixes!(
                        LengthUnit::Meter,
                        "m",
                        "meter",
                        "metre",
                        "meters",
                        "metres"
                    )
                }
                LengthUnit::Parsec(_) => {
                    make_abbreviations_with_prefixes!(LengthUnit::Meter, "pc", "parsec", "parsecs")
                }
                LengthUnit::AstronomicalUnit => make_abbreviations!(l.to_unit(), "au"),
                LengthUnit::LightYear => {
                    make_abbreviations!(l.to_unit(), "ly", "lightyear", "lightyears")
                }
                LengthUnit::Thou => make_abbreviations!(l.to_unit(), "th", "thou", "thous"),
                LengthUnit::Barleycorn => {
                    make_abbreviations!(l.to_unit(), "barleycorn", "barleycorns")
                }
                LengthUnit::Inch => make_abbreviations!(l.to_unit(), "in", "inch", "inches"),
                LengthUnit::Foot => {
                    make_abbreviations!(l.to_unit(), "ft", "foot", "feet", "feets", "foots")
                }
                LengthUnit::Yard => make_abbreviations!(l.to_unit(), "yd", "yard", "yards"),
                LengthUnit::Mile => make_abbreviations!(l.to_unit(), "mi", "mile", "miles"),
                LengthUnit::League => make_abbreviations!(l.to_unit(), "lea", "league", "leagues"),
                LengthUnit::Pole => make_abbreviations!(l.to_unit(), "pole", "poles"),
                LengthUnit::Furlong => {
                    make_abbreviations!(l.to_unit(), "fur", "furlong", "furlongs")
                }
                LengthUnit::Chain => make_abbreviations!(l.to_unit(), "ch", "chain", "chains"),
                LengthUnit::Fathom => make_abbreviations!(l.to_unit(), "ftm", "fathom", "fathoms"),
                LengthUnit::NauticalMile => {
                    make_abbreviations!(l.to_unit(), "NM", "NauticalMile", "NauticalMiles")
                }
                LengthUnit::Finger => make_abbreviations!(l.to_unit(), "finger", "fingers"),
                LengthUnit::Rod => make_abbreviations!(l.to_unit(), "rod", "rods"),
                LengthUnit::NauticalLeague => {
                    make_abbreviations!(l.to_unit(), "NL", "NauticalLeague", "NauticalLeagues")
                }
            });
        }

        abbreviations
    }

    pub fn reference_unit_multiplier(self) -> Decimal {
        match self {
            LengthUnit::Meter(None) => dec!(1),
            LengthUnit::Meter(Some(p)) => p.multiplier(),
            LengthUnit::AstronomicalUnit => dec!(149_597_870_700),
            LengthUnit::LightYear => dec!(9_460_730_472_580_800),
            LengthUnit::Parsec(None) => dec!(648000) / Decimal::PI * dec!(149_597_870_700), // 648000/π * au
            LengthUnit::Parsec(Some(p)) => {
                LengthUnit::Parsec(None).reference_unit_multiplier() * p.multiplier()
            }
            LengthUnit::Thou => LengthUnit::Inch.reference_unit_multiplier() / dec!(1000),
            LengthUnit::Barleycorn => LengthUnit::Inch.reference_unit_multiplier() / dec!(3),
            LengthUnit::Inch => dec!(0.0254),
            LengthUnit::Foot => LengthUnit::Yard.reference_unit_multiplier() / dec!(3),
            LengthUnit::Yard => dec!(0.9144),
            LengthUnit::Mile => LengthUnit::Foot.reference_unit_multiplier() * dec!(5280),
            LengthUnit::Pole => LengthUnit::Foot.reference_unit_multiplier() * dec!(16.5),
            LengthUnit::Rod => LengthUnit::Foot.reference_unit_multiplier() * dec!(16.5),
            LengthUnit::Furlong => LengthUnit::Yard.reference_unit_multiplier() / dec!(220),
            LengthUnit::Chain => LengthUnit::Yard.reference_unit_multiplier() / dec!(22),
            LengthUnit::Fathom => LengthUnit::Foot.reference_unit_multiplier() * dec!(6),
            LengthUnit::NauticalMile => LengthUnit::Foot.reference_unit_multiplier() * dec!(6080),
            LengthUnit::Finger => LengthUnit::Inch.reference_unit_multiplier() * dec!(7) / dec!(8),
            LengthUnit::League => LengthUnit::Mile.reference_unit_multiplier() * dec!(3),
            LengthUnit::NauticalLeague => {
                LengthUnit::NauticalMile.reference_unit_multiplier() * dec!(3)
            }
        }
    }

    pub fn to_unit(self) -> Unit {
        Unit::Length(self)
    }
}

impl Default for LengthUnit {
    fn default() -> Self {
        LengthUnit::Meter(None)
    }
}
