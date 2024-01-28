use std::collections::HashMap;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::units::{Abbreviations, Unit};
use crate::units::{Length, Prefix};
use crate::utils::Pluralize;
use crate::{make_abbreviations, make_abbreviations_with_prefixes, pluralize, string};

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Hash)]
pub enum Volume {
    CubicMeter(Option<Prefix>),
    Litre(Option<Prefix>),
    CubicInch,
    CubicFoot,
    CubicYard,
    CubicMile,
    AcreFoot,
    Minim,
    Drachm,
    FluidOunce,
    Pint,
    TeaSpoon,
    TableSpoon,
    Quart,
    Gallon,
    Barrel,
    Cord,
    Peck,
    Bushel,
    Hogshead,
}

impl Volume {
    pub fn to_string_with_plural(self, _: &Decimal) -> String {
        match self {
            Volume::CubicMeter(None) => string!("m3"),
            Volume::CubicMeter(Some(p)) => string!(p) + "m3",
            Volume::Litre(None) => string!("L"),
            Volume::Litre(Some(p)) => string!(p) + "L",
            Volume::CubicInch => string!("in3"),
            Volume::CubicFoot => string!("ft3"),
            Volume::CubicYard => string!("yd3"),
            Volume::CubicMile => string!("mi3"),
            Volume::AcreFoot => string!("acft"),
            Volume::Minim => string!("min"),
            Volume::Drachm => string!("dr"),
            Volume::FluidOunce => string!("floz"),
            Volume::Pint => string!("pt"),
            Volume::TeaSpoon => string!("tsp"),
            Volume::TableSpoon => string!("tbsp"),
            Volume::Quart => {}
            Volume::Gallon => {}
            Volume::Barrel => {}
            Volume::Cord => {}
            Volume::Peck => {}
            Volume::Bushel => {}
            Volume::Hogshead => {}
        }
    }

    pub fn abbreviations() -> Abbreviations {
        let mut case_sensitive = HashMap::new();
        let mut case_insensitive = HashMap::new();

        for l in Volume::iter() {
            match l {
                _ => {}
            };
        }

        Abbreviations {
            case_sensitive,
            case_insensitive,
        }
    }

    pub fn reference_unit_multiplier(self) -> Decimal {
        match self {
            _ => dec!(1),
        }
    }

    pub fn to_unit(self) -> Unit {
        Unit::Volume(self)
    }
}

impl Default for Volume {
    fn default() -> Self {
        CubicMeter(None)
    }
}
