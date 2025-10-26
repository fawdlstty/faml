use anyhow::anyhow;

use super::faml_value::{ApplyExt, FamlValue};

pub(crate) struct Op1Evaluator {}

impl Op1Evaluator {
    pub fn eval_prefix(op: &str, right: FamlValue) -> anyhow::Result<FamlValue> {
        Ok(match (op, right) {
            ("++", FamlValue::Int64(n)) => FamlValue::Int64(n + 1),
            ("++", FamlValue::Float64(n)) => FamlValue::Float64(n + 1.0),
            ("--", FamlValue::Int64(n)) => FamlValue::Int64(n - 1),
            ("--", FamlValue::Float64(n)) => FamlValue::Float64(n - 1.0),
            ("!", FamlValue::Bool(b)) => FamlValue::Bool(!b),
            ("-", FamlValue::Int64(n)) => FamlValue::Int64(-n),
            ("-", FamlValue::Float64(n)) => FamlValue::Float64(-n),
            ("~", FamlValue::Int64(n)) => FamlValue::Int64(!n),
            _ => return Err(anyhow!("illegal operator: {op}")),
        })
    }

    pub fn eval_suffix(op: &str, left: FamlValue) -> anyhow::Result<FamlValue> {
        Ok(match (op, left) {
            ("++", FamlValue::Int64(n)) => FamlValue::Int64(n + 1),
            ("++", FamlValue::Float64(n)) => FamlValue::Float64(n + 1.0),
            ("--", FamlValue::Int64(n)) => FamlValue::Int64(n - 1),
            ("--", FamlValue::Float64(n)) => FamlValue::Float64(n - 1.0),
            _ => return Err(anyhow!("illegal operator: {op}")),
        })
    }
}

pub(crate) struct Op2Evaluator {}

impl Op2Evaluator {
    pub fn eval(left: FamlValue, op: &str, right: FamlValue) -> anyhow::Result<FamlValue> {
        match (left, op, right) {
            (FamlValue::Bool(left), _, FamlValue::Bool(right)) => {
                Ok(FamlValue::Bool(Self::eval_bool(left, op, right)?))
            }
            (FamlValue::Int64(left), _, FamlValue::Int64(right)) => {
                Self::eval_int64(left, op, right)
            }
            (FamlValue::Float64(left), _, FamlValue::Float64(right)) => {
                Self::eval_float64(left, op, right)
            }
            (FamlValue::Int64(left), _, FamlValue::Float64(right)) => {
                Self::eval_float64(left as f64, op, right)
            }
            (FamlValue::Float64(left), _, FamlValue::Int64(right)) => {
                Self::eval_float64(left, op, right as f64)
            }
            (FamlValue::String(left), _, FamlValue::String(right)) => {
                Self::eval_string(&left, op, &right)
            }
            (FamlValue::String(left), "*", FamlValue::Int64(right)) if right >= 0 => {
                Ok(FamlValue::String(left.repeat(right as usize)))
            }
            (FamlValue::Array(left), "+", FamlValue::Array(right)) => {
                let mut left = left.clone();
                left.extend(right.clone());
                Ok(FamlValue::Array(left))
            }
            (FamlValue::Map(left), "+", FamlValue::Map(right)) => {
                let mut left = left.clone();
                left.apply(right.clone());
                Ok(FamlValue::Map(left))
            }
            _ => Err(anyhow!("illegal operator: {op}")),
        }
    }

    fn eval_bool(left: bool, op: &str, right: bool) -> anyhow::Result<bool> {
        Ok(match op {
            "&&" => left && right,
            "||" => left || right,
            "==" => left == right,
            "!=" => left != right,
            _ => return Err(anyhow!("illegal operator: {op}")),
        })
    }

    fn eval_int64(left: i64, op: &str, right: i64) -> anyhow::Result<FamlValue> {
        Ok(FamlValue::Int64(match op {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            "**" if right < 0 => return Ok(FamlValue::Float64((left as f64).powf(right as f64))),
            "**" => left.pow(right as u32),
            "%" => left % right,
            "|" => left | right,
            "&" => left & right,
            "<<" => left << right,
            ">>" => left >> right,
            "^" => left ^ right,
            "<" => return Ok(FamlValue::Bool(left < right)),
            "<=" => return Ok(FamlValue::Bool(left <= right)),
            ">" => return Ok(FamlValue::Bool(left > right)),
            ">=" => return Ok(FamlValue::Bool(left >= right)),
            "==" => return Ok(FamlValue::Bool(left == right)),
            "!=" => return Ok(FamlValue::Bool(left != right)),
            _ => return Err(anyhow!("illegal operator: {op}")),
        }))
    }

    fn eval_float64(left: f64, op: &str, right: f64) -> anyhow::Result<FamlValue> {
        Ok(FamlValue::Float64(match op {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            "**" => left.powf(right),
            "%" => left % right,
            "<" => return Ok(FamlValue::Bool(left < right)),
            "<=" => return Ok(FamlValue::Bool(left <= right)),
            ">" => return Ok(FamlValue::Bool(left > right)),
            ">=" => return Ok(FamlValue::Bool(left >= right)),
            "==" => return Ok(FamlValue::Bool(left == right)),
            "!=" => return Ok(FamlValue::Bool(left != right)),
            _ => return Err(anyhow!("illegal operator: {op}")),
        }))
    }

    fn eval_string(left: &str, op: &str, right: &str) -> anyhow::Result<FamlValue> {
        match op {
            "+" => Ok(FamlValue::String(format!("{}{}", left, right))),
            "==" => Ok(FamlValue::Bool(left == right)),
            "!=" => Ok(FamlValue::Bool(left != right)),
            _ => Err(anyhow!("illegal operator: {op}")),
        }
    }
}
