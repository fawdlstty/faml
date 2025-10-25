pub mod ffi;

// #[cfg(test)]
// pub mod test;

mod ast;
mod string_utils;

pub use ast::faml_expr::FamlExpr;
pub use ast::faml_value::FamlValue;
use serde::Deserialize;

pub fn from_str<T: for<'a> Deserialize<'a>>(input: &str) -> anyhow::Result<T> {
    from_expr(FamlExpr::from_str(input)?)
}

pub fn from_expr<T: for<'a> Deserialize<'a>>(expr: FamlExpr) -> anyhow::Result<T> {
    from_value(expr.evalute()?)
}

pub fn from_value<T: for<'a> Deserialize<'a>>(value: FamlValue) -> anyhow::Result<T> {
    value.deserialize()
}
