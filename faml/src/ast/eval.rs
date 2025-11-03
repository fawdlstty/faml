use super::faml_value::{ApplyExt, FamlValue};
use crate::ast::faml_value::Distance;
use anyhow::anyhow;
use std::ops::*;
use std::{collections::HashMap, sync::OnceLock, time::Duration};

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
            _ => return Err(anyhow!("illegal prefix operator: {op}")),
        })
    }

    pub fn eval_suffix(left: FamlValue, op: &str) -> anyhow::Result<FamlValue> {
        const G: f64 = Duration::from_secs(1).as_nanos() as f64;
        const D: f64 = Duration::from_secs(86400).as_secs() as f64;
        Ok(match (op, &left) {
            ("++", &FamlValue::Int64(n)) => FamlValue::Int64(n + 1),
            ("++", &FamlValue::Float64(n)) => FamlValue::Float64(n + 1.0),
            ("--", &FamlValue::Int64(n)) => FamlValue::Int64(n - 1),
            ("--", &FamlValue::Float64(n)) => FamlValue::Float64(n - 1.0),
            _ => {
                let n = left
                    .as_float()
                    .ok_or_else(|| anyhow!("cannot calc date unit for another type"))?;

                match op {
                    "nanoseconds" => Duration::from_nanos(n.round() as u64).into(),
                    "microseconds" => Duration::from_nanos((n * 1_000.0).round() as u64).into(),
                    "milliseconds" => Duration::from_nanos((n * 1_000_000.0).round() as u64).into(),
                    "seconds" => Duration::from_nanos((n * G).round() as u64).into(),
                    "mins" => Duration::from_nanos((n * G * 60.0).round() as u64).into(),
                    "hours" => Duration::from_nanos((n * G * 3600.0).round() as u64).into(),
                    "days" => Duration::from_nanos((n * G * D).round() as u64).into(),
                    "weeks" => Duration::from_nanos((n * G * D * 7.0).round() as u64).into(),
                    "months" => Duration::from_nanos((n * G * D * 30.0).round() as u64).into(),
                    "years" => Duration::from_nanos((n * G * D * 365.0).round() as u64).into(),
                    "nanometers" => Distance::from_nanometers(n).into(),
                    "micrometers" => Distance::from_micrometers(n).into(),
                    "millimeters" => Distance::from_millimeters(n).into(),
                    "meters" => Distance::from_meters(n).into(),
                    "kilometers" => Distance::from_kilometers(n).into(),
                    "megameters" => Distance::from_megameters(n).into(),
                    "KB" => (n * 1024.0).into(),
                    "MB" => (n * 1024.0 * 1024.0).into(),
                    "GB" => (n * 1024.0 * 1024.0 * 1024.0).into(),
                    "TB" => (n * 1024.0 * 1024.0 * 1024.0 * 1024.0).into(),
                    _ => return Err(anyhow!("illegal suffix operator: {op}")),
                }
            }
        })
    }
}

pub(crate) struct Op2Evaluator {}

impl Op2Evaluator {
    pub fn get_level(op: &str) -> usize {
        static OP2_LEVELS: OnceLock<HashMap<&'static str, usize>> = OnceLock::new();
        *OP2_LEVELS
            .get_or_init(|| {
                [
                    ("**", 0),
                    ("*", 1),
                    ("/", 1),
                    ("%", 1),
                    ("+", 2),
                    ("-", 2),
                    ("<<", 3),
                    (">>", 3),
                    ("^", 4),
                    ("|", 4),
                    ("&", 4),
                    ("<", 5),
                    ("<=", 5),
                    (">", 5),
                    (">=", 5),
                    ("==", 6),
                    ("!=", 6),
                    ("&&", 7),
                    ("||", 8),
                ]
                .into_iter()
                .collect()
            })
            .get(op)
            .unwrap_or(&9)
    }

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
            (FamlValue::Duration(left), _, FamlValue::Duration(right)) => {
                Self::eval_duration(&left, op, &right)
            }
            (FamlValue::Duration(left), _, FamlValue::Float64(right)) => {
                Self::eval_duration_float(&left, op, right)
            }
            (FamlValue::Float64(left), _, FamlValue::Duration(right)) => {
                Self::eval_float_duration(left, op, &right)
            }
            (FamlValue::Distance(left), _, FamlValue::Distance(right)) => {
                Self::eval_distance(&left, op, &right)
            }
            (FamlValue::Distance(left), _, FamlValue::Float64(right)) => {
                Self::eval_distance_float(&left, op, right)
            }
            (FamlValue::Float64(left), _, FamlValue::Distance(right)) => {
                Self::eval_float_distance(left, op, &right)
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
            "+" => Ok(FamlValue::String(format!("{left}{right}"))),
            "==" => Ok(FamlValue::Bool(left == right)),
            "!=" => Ok(FamlValue::Bool(left != right)),
            _ => Err(anyhow!("illegal operator: {op}")),
        }
    }

    fn eval_duration(left: &Duration, op: &str, right: &Duration) -> anyhow::Result<FamlValue> {
        Ok(FamlValue::Duration(match op {
            "+" => *left + *right,
            "-" => *left - *right,
            "<" => return Ok(FamlValue::Bool(left < right)),
            "<=" => return Ok(FamlValue::Bool(left <= right)),
            ">" => return Ok(FamlValue::Bool(left > right)),
            ">=" => return Ok(FamlValue::Bool(left >= right)),
            "==" => return Ok(FamlValue::Bool(left == right)),
            "!=" => return Ok(FamlValue::Bool(left != right)),
            _ => return Err(anyhow!("illegal operator: {op}")),
        }))
    }

    fn eval_duration_float(left: &Duration, op: &str, right: f64) -> anyhow::Result<FamlValue> {
        let left_nanos = left.as_nanos() as f64;
        let val_nanos = match op {
            "*" => left_nanos * right,
            "/" => left_nanos / right,
            _ => return Err(anyhow!("illegal operator: {op}")),
        };
        Ok(FamlValue::Duration(Duration::from_nanos(
            val_nanos.round() as u64
        )))
    }

    fn eval_float_duration(left: f64, op: &str, right: &Duration) -> anyhow::Result<FamlValue> {
        let right_nanos = right.as_nanos() as f64;
        let val_nanos = match op {
            "*" => left * right_nanos,
            "/" => left / right_nanos,
            _ => return Err(anyhow!("illegal operator: {op}")),
        };
        Ok(FamlValue::Duration(Duration::from_nanos(
            val_nanos.round() as u64
        )))
    }

    fn eval_distance(left: &Distance, op: &str, right: &Distance) -> anyhow::Result<FamlValue> {
        let (left_val, right_val) = (left.to_meters(), right.to_meters());
        Ok(FamlValue::Distance(match op {
            "+" => Distance::from_meters(left_val + right_val),
            "-" => Distance::from_meters(left_val - right_val),
            "<" => return Ok(FamlValue::Bool(left_val < right_val)),
            "<=" => return Ok(FamlValue::Bool(left_val <= right_val)),
            ">" => return Ok(FamlValue::Bool(left_val > right_val)),
            ">=" => return Ok(FamlValue::Bool(left_val >= right_val)),
            "==" => return Ok(FamlValue::Bool(left_val == right_val)),
            "!=" => return Ok(FamlValue::Bool(left_val != right_val)),
            _ => return Err(anyhow!("illegal operator: {op}")),
        }))
    }

    fn eval_distance_float(left: &Distance, op: &str, right: f64) -> anyhow::Result<FamlValue> {
        let left_meters = left.to_meters();
        let val_meters = match op {
            "*" => left_meters * right,
            "/" => left_meters / right,
            _ => return Err(anyhow!("illegal operator: {op}")),
        };
        Ok(FamlValue::Distance(Distance::from_meters(
            val_meters.round(),
        )))
    }

    fn eval_float_distance(left: f64, op: &str, right: &Distance) -> anyhow::Result<FamlValue> {
        let right_meters = right.to_meters();
        let val_meters = match op {
            "*" => left * right_meters,
            "/" => left / right_meters,
            _ => return Err(anyhow!("illegal operator: {op}")),
        };
        Ok(FamlValue::Distance(Distance::from_meters(
            val_meters.round(),
        )))
    }
}

macro_rules! impl_calc {
    ($trait:ident, $method:ident, $op:expr) => {
        impl $trait<f64> for FamlValue {
            type Output = anyhow::Result<FamlValue>;
            fn $method(self, rhs: f64) -> Self::Output {
                Op2Evaluator::eval(self, $op, rhs.into())
            }
        }

        impl $trait<FamlValue> for f64 {
            type Output = anyhow::Result<FamlValue>;
            fn $method(self, rhs: FamlValue) -> Self::Output {
                Op2Evaluator::eval(self.into(), $op, rhs)
            }
        }
    };
}

impl_calc!(Mul, mul, "*");
impl_calc!(Div, div, "/");
impl_calc!(Add, add, "+");
impl_calc!(Sub, sub, "-");
