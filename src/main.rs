/*
 * Copyright (C) 2019 Jan Felix Langenbach
 *
 * This file is part of polish-calc.
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
use std::{
    env,
    io::{self, Read},
};

use polish_calc::run;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap();
        buf
    });
    println!("{}", run(arg));
}
