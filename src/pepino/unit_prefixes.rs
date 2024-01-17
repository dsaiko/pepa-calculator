use std::fmt::{Display, Formatter};

use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;
use strum_macros::EnumIter;

use crate::Decimal;

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Hash)]
pub enum UnitPrefix {
    Quetta,
    Ronna,
    Yotta,
    Zetta,
    Exa,
    Peta,
    Tera,
    Giga,
    Mega,
    Kilo,
    Hecto,
    Deka,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
    Zepto,
    Yocto,
    Ronto,
    Quecto,
}

impl UnitPrefix {
    pub fn multiplier(&self) -> Decimal {
        match self {
            UnitPrefix::Quetta => dec!(10).powu(30),
            UnitPrefix::Ronna => dec!(10).powu(27),
            UnitPrefix::Yotta => dec!(10).powu(24),
            UnitPrefix::Zetta => dec!(10).powu(21),
            UnitPrefix::Exa => dec!(10).powu(18),
            UnitPrefix::Peta => dec!(10).powu(15),
            UnitPrefix::Tera => dec!(10).powu(12),
            UnitPrefix::Giga => dec!(10).powu(9),
            UnitPrefix::Mega => dec!(10).powu(6),
            UnitPrefix::Kilo => dec!(10).powu(3),
            UnitPrefix::Hecto => dec!(10).powu(2),
            UnitPrefix::Deka => dec!(10).powu(1),

            UnitPrefix::Deci => dec!(10).powi(-1),
            UnitPrefix::Centi => dec!(10).powi(-2),
            UnitPrefix::Milli => dec!(10).powi(-3),
            UnitPrefix::Micro => dec!(10).powi(-6),
            UnitPrefix::Nano => dec!(10).powi(-9),
            UnitPrefix::Pico => dec!(10).powi(-12),
            UnitPrefix::Femto => dec!(10).powi(-15),
            UnitPrefix::Atto => dec!(10).powi(-18),
            UnitPrefix::Zepto => dec!(10).powi(-21),
            UnitPrefix::Yocto => dec!(10).powi(-24),
            UnitPrefix::Ronto => dec!(10).powi(-27),
            UnitPrefix::Quecto => dec!(10).powi(-30),
        }
    }

    pub fn abbreviations(self) -> Vec<&'static str> {
        match self {
            UnitPrefix::Quetta => vec!["quetta", "Q"],
            UnitPrefix::Ronna => vec!["ronna", "R"],
            UnitPrefix::Yotta => vec!["yotta", "Y"],
            UnitPrefix::Zetta => vec!["zetta", "Z"],
            UnitPrefix::Exa => vec!["exa", "E"],
            UnitPrefix::Peta => vec!["peta", "P"],
            UnitPrefix::Tera => vec!["tera", "T"],
            UnitPrefix::Giga => vec!["giga", "G"],
            UnitPrefix::Mega => vec!["mega", "M"],
            UnitPrefix::Kilo => vec!["kilo", "k"],
            UnitPrefix::Hecto => vec!["hecto", "h"],
            UnitPrefix::Deka => vec!["deka", "da"],
            UnitPrefix::Deci => vec!["deci", "d"],
            UnitPrefix::Centi => vec!["centi", "c"],
            UnitPrefix::Milli => vec!["milli", "m"],
            UnitPrefix::Micro => vec!["micro", "μ"],
            UnitPrefix::Nano => vec!["nano", "n"],
            UnitPrefix::Pico => vec!["pico", "p"],
            UnitPrefix::Femto => vec!["femto", "f"],
            UnitPrefix::Atto => vec!["atto", "a"],
            UnitPrefix::Zepto => vec!["zepto", "z"],
            UnitPrefix::Yocto => vec!["yocto", "y"],
            UnitPrefix::Ronto => vec!["ronto", "r"],
            UnitPrefix::Quecto => vec!["quecto", "q"],
        }
    }
}

impl Display for UnitPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnitPrefix::Quetta => "Q",
                UnitPrefix::Ronna => "R",
                UnitPrefix::Yotta => "Y",
                UnitPrefix::Zetta => "Z",
                UnitPrefix::Exa => "E",
                UnitPrefix::Peta => "P",
                UnitPrefix::Tera => "T",
                UnitPrefix::Giga => "G",
                UnitPrefix::Mega => "M",
                UnitPrefix::Kilo => "k",
                UnitPrefix::Hecto => "h",
                UnitPrefix::Deka => "da",

                UnitPrefix::Deci => "d",
                UnitPrefix::Centi => "c",
                UnitPrefix::Milli => "m",
                UnitPrefix::Micro => "μ",
                UnitPrefix::Nano => "n",
                UnitPrefix::Pico => "p",
                UnitPrefix::Femto => "f",
                UnitPrefix::Atto => "a",
                UnitPrefix::Zepto => "z",
                UnitPrefix::Yocto => "y",
                UnitPrefix::Ronto => "r",
                UnitPrefix::Quecto => "q",
            }
        )
    }
}
