use crate::{Calculator, NumericExpression, Unit};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub(super) fn split_string_by_comma(s: &str) -> Vec<String> {
    let s = s.trim();
    if s.is_empty() {
        return Vec::new();
    }

    if !s.contains(',') {
        return vec![s.to_owned()];
    }

    let mut res = Vec::new();
    let mut token = String::new();

    let mut parentheses_count = 0;

    for c in s.chars() {
        match c {
            '(' => {
                parentheses_count += 1;
                token.push(c);
            }
            ')' => {
                parentheses_count -= 1;
                token.push(c);
            }
            ',' => {
                if parentheses_count > 0 {
                    token.push(c);
                } else {
                    let t = token.trim();
                    if !t.is_empty() {
                        res.push(t.to_owned());
                    }
                    token.clear();
                }
            }
            _ => token.push(c),
        }
    }

    let t = token.trim();
    if !t.is_empty() {
        res.push(t.to_owned());
    }
    token.clear();

    res
}

pub(super) trait Pluralize {
    fn is_plural(&self) -> bool;
}

impl Pluralize for &Decimal {
    fn is_plural(&self) -> bool {
        self.abs() != Decimal::ONE
    }
}

impl Pluralize for i64 {
    fn is_plural(&self) -> bool {
        self.abs() != 1
    }
}

#[macro_export]
macro_rules! pluralize {
    ($s:expr, $x:expr) => {
        if Pluralize::is_plural(&$x) {
            $s.to_string() + "s"
        } else {
            $s.to_string()
        }
    };
    ($s:expr, $p:expr, $x:expr) => {
        if Pluralize::is_plural(&$x) {
            $p.to_string()
        } else {
            $s.to_string()
        }
    };
}

#[macro_export]
macro_rules! string {
    ($s:expr) => {
        $s.to_string()
    };
}

#[macro_export]
macro_rules! make_abbreviations {
    ($u:expr, $($args:expr),*) => {
        vec![$($args),*]
            .iter()
            .map(|a| string!(a))
            .map(|a| (a, $u))
            .collect::<HashMap<_, _>>()
    };
}

#[macro_export]
macro_rules! make_abbreviations_with_prefixes {
    ($t:path, $($args:expr),*) => {{
        let mut abbreviations = HashMap::new();
        let names = vec![$($args),*];

        for name in names {
            abbreviations.insert(name.to_owned(), $t(None).to_unit());

            for prefix in Prefix::iter() {
                for p in prefix.abbreviations() {
                    abbreviations.insert(
                        format!("{}{}", p, name),
                        $t(Some(prefix)).to_unit(),
                    );
                }
            }
        }

        abbreviations
    }};
}

pub(super) fn flatten_lines<T: Clone>(lines: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut ret = Vec::new();
    if lines.is_empty() {
        return ret;
    }

    let head = &lines[0];
    let tail = lines[1..].to_vec();

    for h in head {
        let l = flatten_lines(&tail);
        if l.is_empty() {
            ret.push(vec![h.clone()]);
        } else {
            for line in flatten_lines(&tail) {
                let mut l = vec![h.clone()];
                l.extend(line);
                ret.push(l);
            }
        }
    }

    ret
}
