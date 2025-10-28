use std::{collections::HashMap, f64::consts::PI};

use anyhow::anyhow;

use crate::FamlValue;

pub trait InvokeExt {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue>;
}

impl InvokeExt for FamlValue {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        match self {
            FamlValue::None => ().invoke(func, args),
            FamlValue::Bool(b) => b.invoke(func, args),
            FamlValue::Int64(i) => i.invoke(func, args),
            FamlValue::Float64(f) => f.invoke(func, args),
            FamlValue::String(s) => s.invoke(func, args),
            FamlValue::Array(arr) => arr.invoke(func, args),
            FamlValue::Map(map) => map.invoke(func, args),
        }
    }
}

impl InvokeExt for () {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        match func {
            "to_str" if args.len() == 0 => Ok("(none)".to_string().into()),
            _ => Err(anyhow!(
                "unknown ().{func} with args[count: {}]",
                args.len()
            )),
        }
    }
}

impl InvokeExt for bool {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        match func {
            "to_str" if args.len() == 0 => Ok(self.to_string().into()),
            _ => Err(anyhow!(
                "unknown bool.{func} with args[count: {}]",
                args.len()
            )),
        }
    }
}

impl InvokeExt for i64 {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        if args.len() == 0 {
            Ok(match func {
                "abs" => self.abs().into(),
                "acos" => (*self as f64).acos().into(),
                "acosh" => (*self as f64).acosh().into(),
                "asin" => (*self as f64).asin().into(),
                "asinh" => (*self as f64).asinh().into(),
                "atan" => (*self as f64).atan().into(),
                "atanh" => (*self as f64).atanh().into(),
                "cbrt" => (*self as f64).cbrt().into(),
                "cos" => (*self as f64).cos().into(),
                "cosh" => (*self as f64).cosh().into(),
                "degree_to_radian" => ((*self as f64) * PI / 180.0).into(),
                "exp" => (*self as f64).exp().into(),
                "exp2" => (*self as f64).exp2().into(),
                //"gamma" => (*self as f64).gamma().into(),
                "ln" => (*self as f64).ln().into(),
                "log10" => (*self as f64).log10().into(),
                "log2" => (*self as f64).log2().into(),
                "radian_to_degree" => ((*self as f64) / PI * 180.0).into(),
                "sin" => (*self as f64).sin().into(),
                "sinh" => (*self as f64).sinh().into(),
                "sqrt" => (*self as f64).sqrt().into(),
                "tan" => (*self as f64).tan().into(),
                "tanh" => (*self as f64).tanh().into(),
                "to_float" => (*self as f64).into(),
                "to_str" => self.to_string().into(),
                _ => Err(anyhow!(
                    "unknown i64.{func} with args[count: {}]",
                    args.len()
                ))?,
            })
        } else if args.len() == 1 {
            if let Some(arg) = args[0].as_float() {
                Ok(match func {
                    "atan2" => (*self as f64).atan2(arg).into(),
                    "hypot" => (*self as f64).hypot(arg).into(),
                    "log" => (*self as f64).log(arg).into(),
                    "pow" => (*self as f64).powf(arg).into(),
                    _ => Err(anyhow!(
                        "unknown i64.{func} with args[count: {}]",
                        args.len()
                    ))?,
                })
            } else {
                Err(anyhow!(
                    "unknown i64.{func} with args[count: {}]",
                    args.len()
                ))
            }
        } else {
            Err(anyhow!(
                "unknown i64.{func} with args[count: {}]",
                args.len()
            ))
        }
    }
}

impl InvokeExt for f64 {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        if args.len() == 0 {
            Ok(match func {
                "abs" => self.abs().into(),
                "acos" => self.acos().into(),
                "acosh" => self.acosh().into(),
                "asin" => self.asin().into(),
                "asinh" => self.asinh().into(),
                "atan" => self.atan().into(),
                "atanh" => self.atanh().into(),
                "cbrt" => self.cbrt().into(),
                "ceil" => self.ceil().into(),
                "ceili" => (self.ceil() as i64).into(),
                "cos" => self.cos().into(),
                "cosh" => self.cosh().into(),
                "degree_to_radian" => (*self * PI / 180.0).into(),
                "exp" => self.exp().into(),
                "exp2" => self.exp2().into(),
                "ln" => self.ln().into(),
                "log10" => self.log10().into(),
                "log2" => self.log2().into(),
                "floor" => self.floor().into(),
                "floori" => (self.floor() as i64).into(),
                "fract" => self.fract().into(),
                "fracti" => (self.fract() as i64).into(),
                //"gamma" => self.gamma().into(),
                "radian_to_degree" => (*self / PI * 180.0).into(),
                "round" => self.round().into(),
                "roundi" => (self.round() as i64).into(),
                "round_ties_even" => self.round_ties_even().into(),
                "round_ties_eveni" => (self.round_ties_even() as i64).into(),
                "sin" => self.sin().into(),
                "sinh" => self.sinh().into(),
                "sqrt" => self.sqrt().into(),
                "tan" => self.tan().into(),
                "tanh" => self.tanh().into(),
                "trunc" => self.trunc().into(),
                "trunci" => (self.trunc() as i64).into(),
                "to_str" => self.to_string().into(),
                _ => Err(anyhow!(
                    "unknown f64.{func} with args[count: {}]",
                    args.len()
                ))?,
            })
        } else if args.len() == 1 {
            if let Some(arg) = args[0].as_float() {
                Ok(match func {
                    "atan2" => self.atan2(arg).into(),
                    "hypot" => self.hypot(arg).into(),
                    "log" => self.log(arg).into(),
                    "pow" => self.powf(arg).into(),
                    _ => Err(anyhow!(
                        "unknown f64.{func} with args[count: {}]",
                        args.len()
                    ))?,
                })
            } else {
                Err(anyhow!(
                    "unknown f64.{func} with args[count: {}]",
                    args.len()
                ))
            }
        } else {
            Err(anyhow!(
                "unknown f64.{func} with args[count: {}]",
                args.len()
            ))
        }
    }
}

impl InvokeExt for String {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        match func {
            "to_str" if args.len() == 0 => Ok(FamlValue::String(self.to_string())),
            _ => Err(anyhow!(
                "unknown string.{func} with args[count: {}]",
                args.len()
            )),
        }
    }
}

impl InvokeExt for Vec<FamlValue> {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        match func {
            "len" if args.len() == 0 => Ok(FamlValue::Int64(self.len() as i64)),
            "pop" if args.len() == 0 => self.pop().ok_or(anyhow!("Array is empty")),
            "push" => {
                for arg in args {
                    self.push(arg.clone());
                }
                Ok(FamlValue::None)
            }
            "reverse" if args.len() == 0 => {
                self.reverse();
                Ok(FamlValue::None)
            }
            "to_str" if args.len() == 0 => {
                let mut s = "[ ".to_string();
                for (i, item) in self.iter_mut().enumerate() {
                    if i > 0 {
                        s += ", ";
                    }
                    s += &item.invoke("to_str", &vec![])?.as_str();
                }
                s += " ]";
                Ok(FamlValue::String(s))
            }
            _ => Err(anyhow!(
                "unknown vec.{func} with args[count: {}]",
                args.len()
            )),
        }
    }
}

impl InvokeExt for HashMap<String, FamlValue> {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        match func {
            "to_str" if args.len() == 0 => {
                let mut s = "{ ".to_string();
                for (i, (key, item)) in self.iter_mut().enumerate() {
                    if i > 0 {
                        s += ", ";
                    }
                    s += &key;
                    s += ": ";
                    s += &item.invoke("to_str", &vec![])?.as_str();
                }
                s += " }";
                Ok(FamlValue::String(s))
            }
            _ => Err(anyhow!(
                "unknown map.{func} with args[count: {}]",
                args.len()
            )),
        }
    }
}
