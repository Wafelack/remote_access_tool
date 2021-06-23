/*
 *  Copyright (C) 2021  Wafelack
 *
 *  This file is part of Vinal.
 *
 *  Vinal is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Vinal is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Vinal.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{
    compiler::Compiler,
    parser::{Expr, ExprT},
};

impl Compiler {
    pub fn colorscheme(&mut self, args: Vec<Expr>) -> Result<String, String> {
        if args.len() > 1 {
            return Err(format!(
                "Function `colorscheme` takes 0 or 1 arguments, but {} arguments were supplied.",
                args.len()
            ));
        }

        let scheme = if args.len() == 0 {
            None
        } else if let ExprT::String(scheme) = &args[0].exprt {
            Some(scheme)
        } else {
            return Err(format!(
                "{}:{} | Expected a String, found a {}.",
                args[0].line,
                args[0].column,
                args[0].get_type()
            ));
        };

        Ok(format!(
            "colorscheme{}",
            if scheme.is_some() {
                format!(" {}", scheme.unwrap())
            } else {
                "".to_string()
            }
        ))
    }
}
