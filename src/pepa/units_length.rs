use std::collections::HashMap;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{make_abbreviations, make_abbreviations_with_prefixes, pluralize, string, Unit};
use crate::unit_prefixes::UnitPrefix;
use crate::units::Abbreviations;
use crate::utils::Pluralize;

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Hash)]
pub enum LengthUnit {
    Meter(Option<UnitPrefix>),

    AstronomicalUnit,
    LightYear,
    Parsec(Option<UnitPrefix>),

    Thou,
    Barleycorn,
    Inch,
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
            LengthUnit::Inch => pluralize!("inch", "inches", v),
            LengthUnit::Foot => string!("ft"),
            LengthUnit::Yard => string!("yd"),
            LengthUnit::Mile => string!("mi"),
            LengthUnit::Pole => pluralize!("pole", v),
            LengthUnit::Rod => pluralize!("rod", v),
            LengthUnit::Furlong => string!("fur"),
            LengthUnit::Chain => string!("ch"),
            LengthUnit::Fathom => string!("ftm"),
            LengthUnit::NauticalMile => string!("NM"),
            LengthUnit::League => string!("lea"),
            LengthUnit::NauticalLeague => string!("NL"),
        }
    }

    pub fn abbreviations() -> Abbreviations {
        let mut case_sensitive = HashMap::new();
        let mut case_insensitive = HashMap::new();

        for l in LengthUnit::iter() {
            match l {
                LengthUnit::Meter(_) => {
                    case_sensitive.extend(make_abbreviations_with_prefixes!(
                        LengthUnit::Meter,
                        // case sensitive
                        "m"
                    ));

                    case_insensitive.extend(make_abbreviations_with_prefixes!(
                        LengthUnit::Meter,
                        // case insensitive
                        "meter",
                        "metre",
                        "meters",
                        "metres"
                    ));
                }
                LengthUnit::Parsec(_) => {
                    case_sensitive.extend(make_abbreviations_with_prefixes!(
                        LengthUnit::Parsec,
                        // case sensitive
                        "pc"
                    ));

                    case_insensitive.extend(make_abbreviations_with_prefixes!(
                        LengthUnit::Parsec,
                        // case insensitive
                        "parsec",
                        "parsecs"
                    ));
                }
                LengthUnit::AstronomicalUnit => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "au"
                    ));
                }
                LengthUnit::LightYear => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "ly"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "lightyear",
                        "lightyears"
                    ));
                }
                LengthUnit::Thou => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "th"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "thou",
                        "thous"
                    ));
                }
                LengthUnit::Barleycorn => {
                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "barleycorn",
                        "barleycorns"
                    ));
                }
                LengthUnit::Inch => {
                    // conflict wit "in" (conversion) keyword
                    // case_sensitive.extend(make_abbreviations!(
                    //     l.to_unit(),
                    //     // case sensitive
                    //     "in"
                    // ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "inch",
                        "inches"
                    ));
                }
                LengthUnit::Foot => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "ft"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "foot",
                        "feet",
                        "feets",
                        "foots"
                    ));
                }
                LengthUnit::Yard => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "yd"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "yard",
                        "yards"
                    ));
                }
                LengthUnit::Mile => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "mi"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "mile",
                        "miles"
                    ));
                }
                LengthUnit::League => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "lea"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "league",
                        "leagues"
                    ));
                }
                LengthUnit::Pole => {
                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "pole",
                        "poles"
                    ));
                }
                LengthUnit::Furlong => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "fur"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "furlong",
                        "furlongs"
                    ));
                }
                LengthUnit::Chain => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "ch"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "chain",
                        "chains"
                    ));
                }
                LengthUnit::Fathom => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "ftm"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "fathom",
                        "fathoms"
                    ));
                }
                LengthUnit::NauticalMile => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "NM"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "nauticalmile",
                        "nauticalmiles"
                    ));
                }

                LengthUnit::Rod => {
                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "rod",
                        "rods"
                    ));
                }
                LengthUnit::NauticalLeague => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "NL"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "nauticalleague",
                        "nauticalleagues"
                    ));
                }
            };
        }

        Abbreviations {
            case_sensitive,
            case_insensitive,
        }
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
            LengthUnit::Furlong => LengthUnit::Yard.reference_unit_multiplier() * dec!(220),
            LengthUnit::Chain => LengthUnit::Yard.reference_unit_multiplier() * dec!(22),
            LengthUnit::Fathom => LengthUnit::Foot.reference_unit_multiplier() * dec!(6),
            LengthUnit::NauticalMile => dec!(1852),
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
