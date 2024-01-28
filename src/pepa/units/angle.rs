use std::collections::HashMap;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::units::Prefix;
use crate::units::{Abbreviations, Unit};
use crate::utils::Pluralize;
use crate::{make_abbreviations, make_abbreviations_with_prefixes, pluralize, string};

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Hash)]
pub enum Angle {
    Radian,
    Degree,
    Gradian,
    Turn,
}

impl Angle {
    pub fn to_string_with_plural(self, _: &Decimal) -> String {
        match self {
            Angle::Radian => string!("rad"),
            Angle::Degree => string!("°"),
            Angle::Gradian => string!("gon"),
            Angle::Turn => string!("tr"),
        }
    }

    pub fn abbreviations() -> Abbreviations {
        let mut case_sensitive = HashMap::new();
        let mut case_insensitive = HashMap::new();

        for l in Angle::iter() {
            match l {
                Angle::Radian => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "rad"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "radian",
                        "radians"
                    ));
                }
                Angle::Degree => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "°"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "degree",
                        "degrees"
                    ));
                }
                Angle::Gradian => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "gon"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "gons",
                        "grad",
                        "grads",
                        "grade",
                        "grades",
                        "gradian",
                        "gradians"
                    ));
                }
                Angle::Turn => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "tr"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "pla",
                        "turn",
                        "turns",
                        "rev",
                        "revolution",
                        "revolutions",
                        "cyc",
                        "cycle",
                        "cycles"
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
            Angle::Radian => dec!(1),
            Angle::Degree => Decimal::PI / dec!(180),
            Angle::Turn => dec!(2) * Decimal::PI,
            Angle::Gradian => Decimal::PI / dec!(200),
        }
    }

    pub fn to_unit(self) -> Unit {
        Unit::Angle(self)
    }
}

impl Default for Angle {
    fn default() -> Self {
        Angle::Radian
    }
}
