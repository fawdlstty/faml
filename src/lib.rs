pub mod ffi;

#[cfg(test)]
pub mod test;

mod ast;
mod string_utils;

pub use ast::oml_expr::OmlExpr;
pub use ast::oml_value::OmlValue;
use serde::Deserialize;

pub fn from_str<T: for<'a> Deserialize<'a>>(input: &str) -> anyhow::Result<T> {
    from_expr(OmlExpr::from_str(input)?)
}

pub fn from_expr<T: for<'a> Deserialize<'a>>(expr: OmlExpr) -> anyhow::Result<T> {
    from_value(expr.evalute()?)
}

pub fn from_value<T: for<'a> Deserialize<'a>>(value: OmlValue) -> anyhow::Result<T> {
    value.deserialize()
}
