use std::collections::HashMap;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::units::{Abbreviations, Prefix, Unit};
use crate::utils::Pluralize;
use crate::{make_abbreviations, make_abbreviations_with_prefixes, pluralize, string};

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Hash)]
pub enum Length {
    Meter(Option<Prefix>),

    AstronomicalUnit,
    LightYear,
    Parsec(Option<Prefix>),

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

impl Length {
    pub fn to_string_with_plural(self, v: &Decimal) -> String {
        match self {
            Length::Meter(None) => string!("m"),
            Length::Meter(Some(p)) => string!(p) + "m",
            Length::AstronomicalUnit => string!("au"),
            Length::LightYear => string!("ly"),
            Length::Parsec(None) => string!("pc"),
            Length::Parsec(Some(p)) => string!(p) + "pc",
            Length::Thou => string!("th"),
            Length::Barleycorn => pluralize!("barleycorn", v),
            Length::Inch => pluralize!("inch", "inches", v),
            Length::Foot => string!("ft"),
            Length::Yard => string!("yd"),
            Length::Mile => string!("mi"),
            Length::Pole => pluralize!("pole", v),
            Length::Rod => pluralize!("rod", v),
            Length::Furlong => string!("fur"),
            Length::Chain => string!("ch"),
            Length::Fathom => string!("ftm"),
            Length::NauticalMile => string!("NM"),
            Length::League => string!("lea"),
            Length::NauticalLeague => string!("NL"),
        }
    }

    pub fn abbreviations() -> Abbreviations {
        let mut case_sensitive = HashMap::new();
        let mut case_insensitive = HashMap::new();

        for l in Length::iter() {
            match l {
                Length::Meter(_) => {
                    case_sensitive.extend(make_abbreviations_with_prefixes!(
                        Length::Meter,
                        // case sensitive
                        "m"
                    ));

                    case_insensitive.extend(make_abbreviations_with_prefixes!(
                        Length::Meter,
                        // case insensitive
                        "meter",
                        "metre",
                        "meters",
                        "metres"
                    ));
                }
                Length::Parsec(_) => {
                    case_sensitive.extend(make_abbreviations_with_prefixes!(
                        Length::Parsec,
                        // case sensitive
                        "pc"
                    ));

                    case_insensitive.extend(make_abbreviations_with_prefixes!(
                        Length::Parsec,
                        // case insensitive
                        "parsec",
                        "parsecs"
                    ));
                }
                Length::AstronomicalUnit => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "au"
                    ));
                }
                Length::LightYear => {
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
                Length::Thou => {
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
                Length::Barleycorn => {
                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "barleycorn",
                        "barleycorns"
                    ));
                }
                Length::Inch => {
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
                Length::Foot => {
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
                Length::Yard => {
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
                Length::Mile => {
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
                Length::League => {
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
                Length::Pole => {
                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "pole",
                        "poles"
                    ));
                }
                Length::Furlong => {
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
                Length::Chain => {
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
                Length::Fathom => {
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
                Length::NauticalMile => {
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

                Length::Rod => {
                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "rod",
                        "rods"
                    ));
                }
                Length::NauticalLeague => {
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
            Length::Meter(None) => dec!(1),
            Length::Meter(Some(p)) => p.multiplier(),
            Length::AstronomicalUnit => dec!(149_597_870_700),
            Length::LightYear => dec!(9_460_730_472_580_800),
            Length::Parsec(None) => dec!(648000) / Decimal::PI * dec!(149_597_870_700), // 648000/π * au
            Length::Parsec(Some(p)) => {
                Length::Parsec(None).reference_unit_multiplier() * p.multiplier()
            }
            Length::Thou => Length::Inch.reference_unit_multiplier() / dec!(1000),
            Length::Barleycorn => Length::Inch.reference_unit_multiplier() / dec!(3),
            Length::Inch => dec!(0.0254),
            Length::Foot => Length::Yard.reference_unit_multiplier() / dec!(3),
            Length::Yard => dec!(0.9144),
            Length::Mile => Length::Foot.reference_unit_multiplier() * dec!(5280),
            Length::Pole => Length::Foot.reference_unit_multiplier() * dec!(16.5),
            Length::Rod => Length::Foot.reference_unit_multiplier() * dec!(16.5),
            Length::Furlong => Length::Yard.reference_unit_multiplier() * dec!(220),
            Length::Chain => Length::Yard.reference_unit_multiplier() * dec!(22),
            Length::Fathom => Length::Foot.reference_unit_multiplier() * dec!(6),
            Length::NauticalMile => dec!(1852),
            Length::League => Length::Mile.reference_unit_multiplier() * dec!(3),
            Length::NauticalLeague => Length::NauticalMile.reference_unit_multiplier() * dec!(3),
        }
    }

    pub fn to_unit(self) -> Unit {
        Unit::Length(self)
    }
}

impl Default for Length {
    fn default() -> Self {
        Length::Meter(None)
    }
}
