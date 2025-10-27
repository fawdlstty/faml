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
            _ => todo!(),
        }
    }
}

impl InvokeExt for bool {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        match func {
            "to_str" if args.len() == 0 => Ok(self.to_string().into()),
            _ => todo!(),
        }
    }
}

impl InvokeExt for i64 {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        if args.len() == 0 {
            Ok(match func {
                "abs" => self.abs().into(),
                "acos" => (*self as f64).acos().into(),
                "asin" => (*self as f64).asin().into(),
                "atan" => (*self as f64).atan().into(),
                "cos" => (*self as f64).cos().into(),
                "degree_to_radian" => ((*self as f64) * PI / 180.0).into(),
                "radian_to_degree" => ((*self as f64) / PI * 180.0).into(),
                "sin" => (*self as f64).sin().into(),
                "tan" => (*self as f64).tan().into(),
                "to_float" => (*self as f64).into(),
                "to_str" => self.to_string().into(),
                _ => todo!(),
            })
        } else {
            todo!()
        }
    }
}

impl InvokeExt for f64 {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        if args.len() == 0 {
            Ok(match func {
                "abs" => self.abs().into(),
                "acos" => self.acos().into(),
                "asin" => self.asin().into(),
                "atan" => self.atan().into(),
                "ceil" => self.ceil().into(),
                "ceili" => (self.ceil() as i64).into(),
                "cos" => self.cos().into(),
                "degree_to_radian" => (*self * PI / 180.0).into(),
                "floor" => self.floor().into(),
                "floori" => (self.floor() as i64).into(),
                "fract" => self.fract().into(),
                "fracti" => (self.fract() as i64).into(),
                "sin" => self.sin().into(),
                "radian_to_degree" => (*self / PI * 180.0).into(),
                "round" => self.round().into(),
                "roundi" => (self.round() as i64).into(),
                "round_ties_even" => self.round_ties_even().into(),
                "round_ties_eveni" => (self.round_ties_even() as i64).into(),
                "tan" => self.tan().into(),
                "trunc" => self.trunc().into(),
                "trunci" => (self.trunc() as i64).into(),
                "to_str" => self.to_string().into(),
                _ => todo!(),
            })
        } else {
            todo!()
        }
    }
}

impl InvokeExt for String {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        match func {
            "to_str" if args.len() == 0 => Ok(FamlValue::String(self.to_string())),
            _ => todo!(),
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
            _ => todo!(),
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
            _ => todo!(),
        }
    }
}
