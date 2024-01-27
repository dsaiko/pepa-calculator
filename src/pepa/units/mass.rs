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
pub enum Mass {
    Gram(Option<Prefix>),
    Tonne(Option<Prefix>),
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

impl Mass {
    pub fn to_string_with_plural(self, v: &Decimal) -> String {
        match self {
            Mass::Gram(None) => string!("g"),
            Mass::Gram(Some(p)) => string!(p) + "g",
            Mass::Tonne(None) => string!("t"),
            Mass::Tonne(Some(p)) => string!(p) + "t",
            Mass::DekaGram => string!("dkg"),
            Mass::LongTon => string!("LT"),
            Mass::ShortTon => string!("st"),
            Mass::Pound => string!("lb"),
            Mass::Ounce => string!("oz"),
            Mass::Slug => pluralize!("slug", v),
            Mass::Grain => string!("gr"),
            Mass::TroyPound => string!("lbt"),
            Mass::TroyOunce => string!("ozt"),
        }
    }

    pub fn abbreviations() -> Abbreviations {
        let mut case_sensitive = HashMap::new();
        let mut case_insensitive = HashMap::new();

        for l in Mass::iter() {
            match l {
                Mass::Gram(_) => {
                    case_sensitive.extend(make_abbreviations_with_prefixes!(
                        Mass::Gram,
                        // case sensitive
                        "g"
                    ));

                    case_insensitive.extend(make_abbreviations_with_prefixes!(
                        Mass::Gram,
                        // case insensitive
                        "gram",
                        "grams",
                        "gramme",
                        "grammes",
                        "gramm",
                        "gramms"
                    ));
                }
                Mass::Tonne(_) => {
                    case_sensitive.extend(make_abbreviations_with_prefixes!(
                        Mass::Tonne,
                        // case sensitive
                        "t"
                    ));

                    case_insensitive.extend(make_abbreviations_with_prefixes!(
                        Mass::Tonne,
                        // case insensitive
                        "tonne",
                        "tonnes"
                    ));
                }
                Mass::DekaGram => {
                    case_sensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case sensitive
                        "dkg"
                    ));
                }
                Mass::LongTon => {
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
                Mass::ShortTon => {
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
                Mass::Pound => {
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
                Mass::Ounce => {
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
                Mass::Slug => {
                    case_insensitive.extend(make_abbreviations!(
                        l.to_unit(),
                        // case insensitive
                        "slug",
                        "slugs"
                    ));
                }
                Mass::Grain => {
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
                Mass::TroyPound => {
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
                Mass::TroyOunce => {
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
            Mass::Gram(None) => dec!(1),
            Mass::Gram(Some(p)) => p.multiplier(),
            Mass::Tonne(None) => dec!(1000000),
            Mass::Tonne(Some(p)) => Mass::Tonne(None).reference_unit_multiplier() * p.multiplier(),
            Mass::DekaGram => dec!(10),
            Mass::LongTon => Mass::Pound.reference_unit_multiplier() * dec!(2240),
            Mass::ShortTon => Mass::Pound.reference_unit_multiplier() * dec!(2000),
            Mass::Pound => dec!(453.59237),
            Mass::Ounce => Mass::Pound.reference_unit_multiplier() / dec!(16),
            Mass::Slug => Mass::Pound.reference_unit_multiplier() * dec!(32.17405),
            Mass::Grain => dec!(0.06479891),
            Mass::TroyPound => Mass::TroyOunce.reference_unit_multiplier() * dec!(12),
            Mass::TroyOunce => Mass::Grain.reference_unit_multiplier() * dec!(480),
        }
    }

    pub fn to_unit(self) -> Unit {
        Unit::Mass(self)
    }
}

impl Default for Mass {
    fn default() -> Self {
        Mass::Gram(None)
    }
}
