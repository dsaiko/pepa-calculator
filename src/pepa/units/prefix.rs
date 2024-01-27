use std::fmt::{Display, Formatter};

use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;
use strum_macros::EnumIter;

use crate::Decimal;

#[derive(Debug, Clone, Eq, Copy, PartialEq, EnumIter, Hash)]
pub enum Prefix {
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

impl Prefix {
    pub fn multiplier(&self) -> Decimal {
        match self {
            Prefix::Quetta => dec!(10).powu(30),
            Prefix::Ronna => dec!(10).powu(27),
            Prefix::Yotta => dec!(10).powu(24),
            Prefix::Zetta => dec!(10).powu(21),
            Prefix::Exa => dec!(10).powu(18),
            Prefix::Peta => dec!(10).powu(15),
            Prefix::Tera => dec!(10).powu(12),
            Prefix::Giga => dec!(10).powu(9),
            Prefix::Mega => dec!(10).powu(6),
            Prefix::Kilo => dec!(10).powu(3),
            Prefix::Hecto => dec!(10).powu(2),
            Prefix::Deka => dec!(10).powu(1),

            Prefix::Deci => dec!(10).powi(-1),
            Prefix::Centi => dec!(10).powi(-2),
            Prefix::Milli => dec!(10).powi(-3),
            Prefix::Micro => dec!(10).powi(-6),
            Prefix::Nano => dec!(10).powi(-9),
            Prefix::Pico => dec!(10).powi(-12),
            Prefix::Femto => dec!(10).powi(-15),
            Prefix::Atto => dec!(10).powi(-18),
            Prefix::Zepto => dec!(10).powi(-21),
            Prefix::Yocto => dec!(10).powi(-24),
            Prefix::Ronto => dec!(10).powi(-27),
            Prefix::Quecto => dec!(10).powi(-30),
        }
    }

    pub fn abbreviations(self) -> Vec<&'static str> {
        match self {
            Prefix::Quetta => vec!["quetta", "Q"],
            Prefix::Ronna => vec!["ronna", "R"],
            Prefix::Yotta => vec!["yotta", "Y"],
            Prefix::Zetta => vec!["zetta", "Z"],
            Prefix::Exa => vec!["exa", "E"],
            Prefix::Peta => vec!["peta", "P"],
            Prefix::Tera => vec!["tera", "T"],
            Prefix::Giga => vec!["giga", "G"],
            Prefix::Mega => vec!["mega", "M"],
            Prefix::Kilo => vec!["kilo", "k"],
            Prefix::Hecto => vec!["hecto", "h"],
            Prefix::Deka => vec!["deka", "da"],
            Prefix::Deci => vec!["deci", "d"],
            Prefix::Centi => vec!["centi", "c"],
            Prefix::Milli => vec!["milli", "m"],
            Prefix::Micro => vec!["micro", "μ"],
            Prefix::Nano => vec!["nano", "n"],
            Prefix::Pico => vec!["pico", "p"],
            Prefix::Femto => vec!["femto", "f"],
            Prefix::Atto => vec!["atto", "a"],
            Prefix::Zepto => vec!["zepto", "z"],
            Prefix::Yocto => vec!["yocto", "y"],
            Prefix::Ronto => vec!["ronto", "r"],
            Prefix::Quecto => vec!["quecto", "q"],
        }
    }
}

impl Display for Prefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Prefix::Quetta => "Q",
                Prefix::Ronna => "R",
                Prefix::Yotta => "Y",
                Prefix::Zetta => "Z",
                Prefix::Exa => "E",
                Prefix::Peta => "P",
                Prefix::Tera => "T",
                Prefix::Giga => "G",
                Prefix::Mega => "M",
                Prefix::Kilo => "k",
                Prefix::Hecto => "h",
                Prefix::Deka => "da",

                Prefix::Deci => "d",
                Prefix::Centi => "c",
                Prefix::Milli => "m",
                Prefix::Micro => "μ",
                Prefix::Nano => "n",
                Prefix::Pico => "p",
                Prefix::Femto => "f",
                Prefix::Atto => "a",
                Prefix::Zepto => "z",
                Prefix::Yocto => "y",
                Prefix::Ronto => "r",
                Prefix::Quecto => "q",
            }
        )
    }
}
