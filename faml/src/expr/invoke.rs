use crate::{FamlValue, expr::faml_value::Distance};
use anyhow::anyhow;
use std::{collections::HashMap, f64::consts::PI, time::Duration};

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
            FamlValue::Duration(dur) => dur.invoke(func, args),
            FamlValue::Distance(dist) => dist.invoke(func, args),
            FamlValue::Json(root) => root.invoke(func, args),
            FamlValue::Yaml(root) => root.invoke(func, args),
        }
    }
}

impl InvokeExt for () {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        match func {
            "to_str" if args.len() == 0 => Ok("null".to_string().into()),
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
                "gamma" => (*self as f64).f64_gamma().into(),
                "is_negative" => self.is_negative().into(),
                "is_positive" => self.is_positive().into(),
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
                "to_quantified" => (*self as f64).to_quantified().into(),
                "to_str" => self.to_string().into(),
                _ => Err(anyhow!(
                    "unknown i64.{func} with args[count: {}]",
                    args.len()
                ))?,
            })
        } else if args.len() == 1 {
            if let Some(arg) = args[0].as_int() {
                Ok(match func {
                    "max" => (*self).max(arg).into(),
                    "min" => (*self).min(arg).into(),
                    _ => Err(anyhow!(
                        "unknown i64.{func} with args[count: {}]",
                        args.len()
                    ))?,
                })
            } else if let Some(arg) = args[0].as_float() {
                Ok(match func {
                    "atan2" => (*self as f64).atan2(arg).into(),
                    "hypot" => (*self as f64).hypot(arg).into(),
                    "log" => (*self as f64).log(arg).into(),
                    "max" => (*self as f64).max(arg).into(),
                    "min" => (*self as f64).min(arg).into(),
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
                "exp" => self.exp().into(),
                "exp2" => self.exp2().into(),
                "floor" => self.floor().into(),
                "floori" => (self.floor() as i64).into(),
                "fract" => self.fract().into(),
                "gamma" => self.f64_gamma().into(),
                "is_finite" => self.is_finite().into(),
                "is_infinite" => self.is_infinite().into(),
                "is_nan" => self.is_nan().into(),
                "is_negative" => self.is_sign_negative().into(),
                "is_positive" => self.is_sign_positive().into(),
                "ln" => self.ln().into(),
                "log10" => self.log10().into(),
                "log2" => self.log2().into(),
                "next_down" => self.next_down().into(),
                "next_up" => self.next_up().into(),
                "round" => self.round().into(),
                "roundi" => (self.round() as i64).into(),
                "round_ties_even" => self.round_ties_even().into(),
                "round_ties_eveni" => (self.round_ties_even() as i64).into(),
                "signum" => self.signum().into(),
                "sin" => self.sin().into(),
                "sinh" => self.sinh().into(),
                "sqrt" => self.sqrt().into(),
                "tan" => self.tan().into(),
                "tanh" => self.tanh().into(),
                "trunc" => self.trunc().into(),
                "trunci" => (self.trunc() as i64).into(),
                "to_quantified" => self.to_quantified().into(),
                "to_degrees" => self.to_degrees().into(),
                "to_radians" => self.to_radians().into(),
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
                    "max" => self.max(arg).into(),
                    "min" => self.min(arg).into(),
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
        if args.len() == 0 {
            Ok(match func {
                "is_empty" => FamlValue::Bool(self.is_empty()),
                "len" => FamlValue::Int64(self.len() as i64),
                "lines" => self.invoke("split", &vec![FamlValue::String("\n".to_string())])?,
                "to_lowercase" => FamlValue::String(self.to_lowercase()),
                "to_str" => FamlValue::String(self.clone()),
                "to_uppercase" => FamlValue::String(self.to_uppercase()),
                "trim" => FamlValue::String(self.trim().to_string()),
                _ => Err(anyhow!(
                    "unknown string.{func} with args[count: {}]",
                    args.len()
                ))?,
            })
        } else {
            if func == "split" || func == "split_once" || func == "split_without_empty" {
                let args: Vec<_> = args.iter().map(|p| p.as_str()).collect();
                let mut ret: Vec<FamlValue> = vec![];
                let mut target = &self[..];
                let is_nempty = func == "split_without_empty";
                while !target.is_empty() {
                    let mut ps: Vec<_> = args
                        .iter()
                        .filter_map(|p| target.find(p).map(|n| (n, p.len())))
                        .collect();
                    ps.sort_by_key(|p| p.0);
                    if let Some((n, len)) = ps.first() {
                        let r = target[..*n].to_string();
                        if !is_nempty || r.len() > 0 {
                            ret.push(FamlValue::String(r));
                        }
                        target = &target[*n + *len..];
                    }
                    if ps.len() == 0 || func == "split_once" {
                        let r = target.to_string();
                        if !is_nempty || r.len() > 0 {
                            ret.push(FamlValue::String(r));
                        }
                        break;
                    }
                }
                Ok(FamlValue::Array(ret))
            } else if args.len() == 1 {
                Ok(match func {
                    "contains" => {
                        let arg = args[0].as_str();
                        FamlValue::Bool(self.contains(&arg))
                    }
                    "ends_with" => {
                        let arg = args[0].as_str();
                        FamlValue::Bool(self.ends_with(&arg))
                    }
                    "find" => {
                        let arg = args[0].as_str();
                        match self.find(&arg) {
                            Some(n) => FamlValue::Int64(n as i64),
                            None => FamlValue::None,
                        }
                    }
                    "repeat" => {
                        let arg = args[0]
                            .as_int()
                            .ok_or(anyhow!("only type[int] arg for method[repeat]"))?;
                        let mut ret = "".to_string();
                        for _ in 0..arg {
                            ret += self;
                        }
                        FamlValue::String(ret)
                    }
                    "rfind" => {
                        let arg = args[0].as_str();
                        match self.rfind(&arg) {
                            Some(n) => FamlValue::Int64(n as i64),
                            None => FamlValue::None,
                        }
                    }
                    "split_at" => {
                        let arg = args[0]
                            .as_int()
                            .ok_or(anyhow!("only type[int] arg for method[split_at]"))?;
                        match self.len() >= arg as usize && arg >= 0 {
                            true => FamlValue::Array(vec![
                                FamlValue::String(self[..arg as usize].to_string()),
                                FamlValue::String(self[arg as usize..].to_string()),
                            ]),
                            false => FamlValue::Array(vec![FamlValue::String(self.to_string())]),
                        }
                    }
                    "starts_with" => {
                        let arg = args[0].as_str();
                        FamlValue::Bool(self.starts_with(&arg))
                    }
                    _ => Err(anyhow!(
                        "unknown string.{func} with args[count: {}]",
                        args.len()
                    ))?,
                })
            } else if args.len() == 2 {
                let pre = args[0].as_str();
                let post = args[1].as_str();
                Ok(FamlValue::String(match func {
                    "replace_once" => self.replacen(&pre, &post, 1),
                    "replace" => self.replace(&pre, &post),
                    _ => panic!("unreachable"),
                }))
            } else {
                Err(anyhow!(
                    "unknown string.{func} with args[count: {}]",
                    args.len()
                ))?
            }
        }
    }
}

impl InvokeExt for Vec<FamlValue> {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        match func {
            "join" if args.len() == 1 => {
                let sep = args[0].as_str();
                let mut ret = "".to_string();
                for (i, item) in self.iter().enumerate() {
                    if i > 0 {
                        ret += &sep;
                    }
                    ret += &item.as_str();
                }
                Ok(FamlValue::String(ret))
            }
            "len" if args.len() == 0 => Ok(FamlValue::Int64(self.len() as i64)),
            "pop" if args.len() == 0 => self.pop().ok_or(anyhow!("Array is empty")),
            "push" => {
                for arg in args {
                    self.push(arg.clone());
                }
                Ok(FamlValue::None)
            }
            "reverse" if args.len() == 0 => {
                let mut ret = self.clone();
                ret.reverse();
                Ok(FamlValue::Array(ret))
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
            "len" if args.len() == 0 => Ok(FamlValue::Int64(self.len() as i64)),
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

impl InvokeExt for Duration {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        const G: f64 = Duration::from_secs(1).as_nanos() as f64;
        const D: f64 = Duration::from_secs(86400).as_secs() as f64;
        if args.len() == 0 {
            Ok(match func {
                "as_nanoseconds" => (self.as_nanos() as f64).into(),
                "as_microseconds" => (self.as_nanos() as f64 * 1_000.0).into(),
                "as_milliseconds" => (self.as_nanos() as f64 * 1_000_000.0).into(),
                "as_seconds" => (self.as_nanos() as f64 * G).into(),
                "as_mins" => (self.as_nanos() as f64 * G * 60.0).into(),
                "as_hours" => (self.as_nanos() as f64 * G * 3600.0).into(),
                "as_days" => (self.as_nanos() as f64 * G * D).into(),
                "as_weeks" => (self.as_nanos() as f64 * G * D * 7.0).into(),
                "as_months" => (self.as_nanos() as f64 * G * D * 30.0).into(),
                "as_years" => (self.as_nanos() as f64 * G * D * 365.0).into(),
                "to_str" => self.to_str().into(),
                _ => Err(anyhow!(
                    "unknown duration.{func} with args[count: {}]",
                    args.len()
                ))?,
            })
        } else {
            Err(anyhow!(
                "unknown duration.{func} with args[count: {}]",
                args.len()
            ))
        }
    }
}

impl InvokeExt for Distance {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        if args.len() == 0 {
            Ok(match func {
                "to_megameters" => self.to_megameters().into(),
                "to_kilometers" => self.to_kilometers().into(),
                "to_meters" => self.to_meters().into(),
                "to_millimeters" => self.to_millimeters().into(),
                "to_micrometers" => self.to_micrometers().into(),
                "to_nanometers" => self.to_nanometers().into(),
                "to_str" => FamlValue::String(self.to_str()),
                _ => Err(anyhow!(
                    "unknown distance.{func} with args[count: {}]",
                    args.len()
                ))?,
            })
        } else {
            Err(anyhow!(
                "unknown distance.{func} with args[count: {}]",
                args.len()
            ))
        }
    }
}

impl InvokeExt for serde_json::Value {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        Err(anyhow!(
            "unknown json.{func} with args[count: {}]",
            args.len()
        ))?
    }
}

impl InvokeExt for serde_yaml::Value {
    fn invoke(&mut self, func: &str, args: &Vec<FamlValue>) -> anyhow::Result<FamlValue> {
        Err(anyhow!(
            "unknown yaml.{func} with args[count: {}]",
            args.len()
        ))?
    }
}

pub trait F64Ext {
    fn f64_gamma(self) -> f64;
    fn to_quantified(self) -> String;
}

impl F64Ext for f64 {
    fn f64_gamma(self) -> f64 {
        let x = self;
        const G: f64 = 7.0;
        const P: [f64; 9] = [
            0.99999999999980993,
            676.5203681218851,
            -1259.1392167224028,
            771.32342877765313,
            -176.61502916214059,
            12.507343278686905,
            -0.13857109526572012,
            9.9843695780195716e-6,
            1.5056327351493116e-7,
        ];

        if x <= 0.0 {
            std::f64::consts::PI / ((-x * std::f64::consts::PI).sin() * (1.0 - x).f64_gamma())
        } else if x < 0.5 {
            std::f64::consts::PI / ((std::f64::consts::PI * x).sin() * (1.0 - x).f64_gamma())
        } else {
            let x = x - 1.0;
            let mut a = P[0];
            let t = x + G + 0.5;
            for (i, &p) in P.iter().enumerate().skip(1) {
                a += p / (x + i as f64);
            }
            (2.0 * std::f64::consts::PI).sqrt() * t.powf(x + 0.5) * (-t).exp() * a
        }
    }

    fn to_quantified(self) -> String {
        const U: f64 = 1024.0;
        match self {
            f if f <= U => format!("{f} B"),
            f if f <= U * U => format!("{} KB", f / U),
            f if f <= U * U * U => format!("{} MB", f / U / U),
            f if f <= U * U * U * U => format!("{} GB", f / U / U / U),
            f => format!("{} TB", f / U / U / U / U),
        }
    }
}

pub trait DurationExt {
    fn to_str(&self) -> String;
}

impl DurationExt for Duration {
    fn to_str(&self) -> String {
        const G: f64 = Duration::from_secs(1).as_nanos() as f64;
        const D: f64 = Duration::from_secs(86400).as_secs() as f64;
        match self.as_nanos() as f64 {
            v if v < 1_000.0 => format!("{v} nanoseconds").into(),
            v if v < 1_000_000.0 => format!("{} microseconds", v / 1_000.0).into(),
            v if v < G => format!("{} milliseconds", v / 1_000_000.0).into(),
            v if v < G * 60.0 => format!("{} seconds", v / G).into(),
            v if v < G * 3_600.0 => format!("{} mins", v / G / 60.0).into(),
            v if v < G * D => format!("{} hours", v / G / 3_600.0).into(),
            v if v < G * D * 7.0 => format!("{} days", v / G / D).into(),
            v if v < G * D * 30.0 => format!("{} weeks", v / G / D / 7.0).into(),
            v if v < G * D * 365.0 => format!("{} months", v / G / D / 30.0).into(),
            v => format!("{} years", v / G / D / 365.0).into(),
        }
    }
}
