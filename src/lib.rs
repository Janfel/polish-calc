/*
 * Copyright (C) 2019 Jan Felix Langenbach
 *
 * This file is part of plc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http: //www.gnu.org/licenses/>.
 */

use std::f64::consts::{E, PI};

pub fn run(arg: String) -> f64 {
    if arg.is_empty() {
        panic!("No input")
    }
    let exprs: Vec<&str> = arg.split_whitespace().collect();
    compute(&exprs)[0]
}

fn compute(exprs: &[&str]) -> Vec<f64> {
    if exprs.is_empty() {
        return Vec::new();
    }
    let mut vals = compute(&exprs[1..]);
    let mut getval = || vals.pop().expect("Not enough operands in expression");

    let result = match exprs[0] {
        "+" => getval() + getval(),
        "-" => getval() - getval(),
        "*" => getval() * getval(),
        "/" => getval() / getval(),
        "**" => getval().powf(getval()),
        "neg" => -getval(),
        "abs" => getval().abs(),
        "sqrt" => getval().sqrt(),
        "ln" => getval().ln(),
        "sin" => getval().sin(),
        "cos" => getval().cos(),
        "tan" => getval().tan(),
        "asin" => getval().asin(),
        "acos" => getval().acos(),
        "atan" => getval().atan(),
        "E" => E,
        "PI" => PI,
        num => num
            .parse()
            .unwrap_or_else(|_| panic!("Invalid Token: {}", num)),
    };
    vals.push(result);
    vals
}

#[cfg(test)]
mod tests {
    use std::f64::{
        consts::{FRAC_PI_3, FRAC_PI_4, FRAC_PI_6},
        EPSILON, INFINITY,
    };

    use super::*;

    fn calculate(expr: &str, expected: f64) {
        let result = run(expr.to_owned());
        assert!((result - expected).abs() < EPSILON);
    }

    macro_rules! param_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
               calculate(input, expected)
            }
        )*
        }
    }

    param_tests! {
        addition: ("+ 2 3", 5.0),
        subtraction: ("- 10 5", 5.0),
        multiplication: ("* 3 3", 9.0),
        division: ("/ 6 2", 3.0),
        real_division: ("/ 2 3", 2.0 / 3.0),
        pow: ("** 3 2", 9.0),
        neg: ("neg 2", -2.0),
        abs: ("abs neg 4", 4.0),
        sqrt: ("sqrt 16", 4.0),
        ln: ("ln ** E 4", 4.0),
        sin: ("sin PI", 0.0),
        cos: ("cos PI", -1.0),
        tan: ("tan * 0.25 PI", 1.0),
        asin: ("asin 0.5", FRAC_PI_6),
        acos: ("acos 0.5", FRAC_PI_3),
        atan: ("atan 1", FRAC_PI_4),
        e: ("E", E),
        pi: ("PI", PI),
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn zero_division() {
        let result = run("/ 6 0".to_owned());
        assert_eq!(result, INFINITY);
    }

}
