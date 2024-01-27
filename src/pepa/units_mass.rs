use std::collections::HashMap;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::unit_prefixes::UnitPrefix;
use crate::units::Abbreviations;
use crate::utils::Pluralize;
use crate::{make_abbreviations, make_abbreviations_with_prefixes, pluralize, string, Unit};

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Hash)]
pub enum MassUnit {
    Gram(Option<UnitPrefix>),
    Tonne(Option<UnitPrefix>),
    DekaGram,
    LongTon,
    ShortTon,
    Pound,
    Ounce,
    TroyPound,
    TroyOunce,
    Slug,
    Grain,
}

impl MassUnit {
    pub fn to_string_with_plural(self, v: &Decimal) -> String {
        match self {
            MassUnit::Gram(None) => string!("g"),
            MassUnit::Gram(Some(p)) => string!(p) + "g",
            MassUnit::Tonne(None) => string!("t"),
            MassUnit::Tonne(Some(p)) => string!(p) + "t",
            MassUnit::DekaGram => string!("dkg"),
            MassUnit::LongTon => string!("LT"),
            MassUnit::ShortTon => string!("st"),
            MassUnit::Pound => string!("lb"),
            MassUnit::Ounce => string!("oz"),
            MassUnit::Slug => pluralize!("slug", v),
            MassUnit::Grain => string!("gr"),
            MassUnit::TroyPound => string!("lbt"),
            MassUnit::TroyOunce => string!("ozt"),
        }
    }

    pub fn abbreviations() -> Abbreviations {
        let mut case_sensitive = HashMap::new();
        let mut case_insensitive = HashMap::new();

        for l in MassUnit::iter() {
            match l {
                MassUnit::Gram(_) => {
                    case_sensitive.extend(make_abbreviations_with_prefixes!(
                        MassUnit::Gram,
                        // case sensitive
                        "g"
                    ));

                    case_insensitive.extend(make_abbreviations_with_prefixes!(
                        MassUnit::Gram,
                        // case insensitive
                        "gram",
                        "grams",
                        "gramme",
                        "grammes",
                        "gramm",
                        "gramms"
                    ));
                }
                MassUnit::Tonne(_) => {
                    case_sensitive.extend(make_abbreviations_with_prefixes!(
                        MassUnit::Tonne,
                        // case sensitive
                        "t"
                    ));

                    case_insensitive.extend(make_abbreviations_with_prefixes!(
                        MassUnit::Tonne,
                        // case insensitive
                        "tonne",
                        "tonnes"
                    ));
                }
                MassUnit::DekaGram => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "dkg"
                    ));
                }
                MassUnit::LongTon => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "LT"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "longton",
                        "longtons"
                    ));
                }
                MassUnit::ShortTon => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "st"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "shortton",
                        "shorttons"
                    ));
                }
                MassUnit::Pound => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "lb"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "pound",
                        "pounds",
                        "libra",
                        "libras",
                        "libre",
                        "libres"
                    ));
                }
                MassUnit::Ounce => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "oz"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "ounce",
                        "ounces"
                    ));
                }
                MassUnit::Slug => {
                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "slug",
                        "slugs"
                    ));
                }
                MassUnit::Grain => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "gr"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "grain",
                        "grains"
                    ));
                }
                MassUnit::TroyPound => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "lbt"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "troypound",
                        "troypounds",
                        "troylibra",
                        "troylibras",
                        "troylibre",
                        "troylibres"
                    ));
                }
                MassUnit::TroyOunce => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "ozt"
                    ));

                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "troyounce",
                        "troyounces"
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
            MassUnit::Gram(None) => dec!(1),
            MassUnit::Gram(Some(p)) => p.multiplier(),
            MassUnit::Tonne(None) => dec!(1000000),
            MassUnit::Tonne(Some(p)) => {
                MassUnit::Tonne(None).reference_unit_multiplier() * p.multiplier()
            }
            MassUnit::DekaGram => dec!(10),
            MassUnit::LongTon => MassUnit::Pound.reference_unit_multiplier() * dec!(2240),
            MassUnit::ShortTon => MassUnit::Pound.reference_unit_multiplier() * dec!(2000),
            MassUnit::Pound => dec!(453.59237),
            MassUnit::Ounce => MassUnit::Pound.reference_unit_multiplier() / dec!(16),
            MassUnit::Slug => MassUnit::Pound.reference_unit_multiplier() * dec!(32.17405),
            MassUnit::Grain => dec!(0.06479891),
            MassUnit::TroyPound => MassUnit::TroyOunce.reference_unit_multiplier() * dec!(12),
            MassUnit::TroyOunce => MassUnit::Grain.reference_unit_multiplier() * dec!(480),
        }
    }

    pub fn to_unit(self) -> Unit {
        Unit::Mass(self)
    }
}

impl Default for MassUnit {
    fn default() -> Self {
        MassUnit::Gram(None)
    }
}
