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

pub fn run(arg: String) -> f64 {
    let exprs: Vec<&str> = arg.split_whitespace().collect();
    compute(&exprs)[0]
}

fn compute(exprs: &[&str]) -> Vec<f64> {
    if exprs.is_empty() {
        return Vec::new();
    }
    if exprs.len() == 1 {
        return vec![exprs[0].parse().expect("Invalid Token")];
    }
    let mut vals = compute(&exprs[1..]);
    let mut getval = || vals.pop().expect("Not enough operands in expression");

    let result = match exprs[0] {
        "+" => getval() + getval(),
        "-" => getval() - getval(),
        "*" => getval() * getval(),
        "/" => getval() / getval(),
        num => num.parse().expect("Invalid Token"),
    };
    vals.push(result);
    vals
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calculate(expr: &str, expected: f64) {
        let result = run(expr.to_owned());
        assert!((result - expected).abs() < std::f64::EPSILON);
    }

    #[test]
    fn addition() {
        calculate("+ 2 3", 5.0)
    }

    #[test]
    fn subtraction() {
        calculate("- 10 5", 5.0)
    }

    #[test]
    fn multiplication() {
        calculate("* 3 3", 9.0)
    }

    #[test]
    fn division() {
        calculate("/ 6 2", 3.0)
    }

    #[test]
    fn real_division() {
        calculate("/ 2 3", 2.0 / 3.0)
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn zero_division() {
        let result = run("/ 6 0".to_owned());
        assert_eq!(result, std::f64::INFINITY);
    }

}
