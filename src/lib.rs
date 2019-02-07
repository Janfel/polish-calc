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

pub fn run(arg: String) -> f32 {
    let exprs: Vec<&str> = arg.split_whitespace().collect();
    compute(&exprs)[0]
}

fn compute(exprs: &[&str]) -> Vec<f32> {
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
