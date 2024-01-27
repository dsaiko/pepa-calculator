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

#[cfg(test)]
pub fn test_units(test: &str, res: &[(Decimal, Option<Unit>)]) {
    let mut computer = Calculator::default();
    let statement = computer.compute(test).unwrap();

    if let Err(e) = &statement.expression {
        panic!("Error in expression: '{:?}': {:?}", test, e);
    }

    match &statement.result {
        None => panic!("No result for: '{:?}'", test),
        Some(Err(e)) => panic!("Error in computation: '{:?}': {:?}", test, e),
        Some(Ok(n)) => {
            let t = NumericExpression::with_multiple_units(res.to_vec());
            let mut ok = true;

            let mut v1 = t.values();
            let mut v2 = n.values();

            if v1.len() != v2.len() {
                ok = false;
            } else {
                for i in 0..v1.len() {
                    if v1[i].1 != v2[i].1 {
                        ok = false;
                    }

                    let n1 = (v1[i].0 * dec!(100)).round() / dec!(100);
                    let n2 = (v2[i].0 * dec!(100)).round() / dec!(100);

                    v1[i].0 = n1;
                    v2[i].0 = n2;

                    if n1 != n2 {
                        ok = false;
                    }
                }
            }

            if !ok {
                panic!("{:?}: {:?} != {:?}", test, v1, v2);
            }
        }
    }
}
