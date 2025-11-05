use crate::ast::invoke::DurationExt;
use crate::{FamlExpr, FamlExprImpl};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum FamlValue {
    None,
    Bool(bool),
    Int64(i64),
    Float64(f64),
    String(String),
    Array(Vec<FamlValue>),
    Map(HashMap<String, FamlValue>),
    Duration(Duration),
    Distance(Distance),
}

impl Serialize for FamlValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            FamlValue::None => serializer.serialize_unit(),
            FamlValue::Bool(b) => serializer.serialize_bool(*b),
            FamlValue::Int64(i) => serializer.serialize_i64(*i),
            FamlValue::Float64(f) => serializer.serialize_f64(*f),
            FamlValue::String(s) => serializer.serialize_str(s),
            FamlValue::Array(arr) => arr.serialize(serializer),
            FamlValue::Map(map) => map.serialize(serializer),
            FamlValue::Duration(dur) => {
                let root = json!({ "famltype": "duration", "unit": "seconds", "value": dur.as_nanos() as f64 / 1_000_000_000.0 });
                root.serialize(serializer)
            }
            FamlValue::Distance(dis) => {
                let root = json!({ "famltype": "distance", "unit": "meters", "value": dis.0 });
                root.serialize(serializer)
            }
        }
    }
}

impl FamlValue {
    pub fn to_expr(self) -> FamlExpr {
        FamlExprImpl::Value(self).to_expr()
    }

    pub fn is_none(&self) -> bool {
        match self {
            FamlValue::None => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            FamlValue::Bool(_) => true,
            _ => false,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            FamlValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            FamlValue::Int64(_) => true,
            _ => false,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            FamlValue::Int64(i) => Some(*i),
            _ => None,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            FamlValue::Float64(_) => true,
            _ => false,
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            FamlValue::String(_) => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            FamlValue::None => "null".to_string(),
            FamlValue::Bool(b) => b.to_string(),
            FamlValue::Int64(i) => i.to_string(),
            FamlValue::Float64(f) => f.to_string(),
            FamlValue::String(s) => s.clone(),
            FamlValue::Array(arr) => {
                let arr: Vec<_> = arr.iter().map(|item| item.as_str()).collect();
                format!("[ {} ]", arr.join(", "))
            }
            FamlValue::Map(map) => {
                let mut ret = "{ ".to_string();
                for (key, value) in map.iter() {
                    if !ret.is_empty() {
                        ret.push_str(", ");
                    }
                    ret.push_str(key);
                    ret.push_str(": ");
                    ret.push_str(&value.as_str());
                }
                ret.push_str(" }");
                ret
            }
            FamlValue::Duration(dur) => dur.to_str(),
            FamlValue::Distance(dis) => dis.to_str(),
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            FamlValue::Float64(f) => Some(*f),
            FamlValue::Int64(i) => Some(*i as f64),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            FamlValue::Array(_) => true,
            _ => false,
        }
    }

    pub fn as_array(&self) -> Option<Vec<FamlValue>> {
        match self {
            FamlValue::Array(arr) => Some(arr.clone()),
            _ => None,
        }
    }

    pub fn is_map(&self) -> bool {
        match self {
            FamlValue::Map(_) => true,
            _ => false,
        }
    }

    pub fn as_map(&self) -> Option<HashMap<String, FamlValue>> {
        match self {
            FamlValue::Map(map) => Some(map.clone()),
            _ => None,
        }
    }

    fn apply(&mut self, val: FamlValue) {
        match self {
            FamlValue::Array(arr) => arr.push(val),
            FamlValue::Map(map) => {
                if let FamlValue::Map(map2) = val {
                    map.apply(map2);
                } else {
                    *self = val;
                }
            }
            _ => *self = val,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            FamlValue::None => serde_json::Value::Null,
            FamlValue::Bool(b) => (*b).into(),
            FamlValue::Int64(i) => (*i).into(),
            FamlValue::Float64(f) => (*f).into(),
            FamlValue::String(s) => s.clone().into(),
            FamlValue::Array(vals) => {
                let mut rets = vec![];
                for val in vals.iter() {
                    rets.push(val.to_json());
                }
                rets.into()
            }
            FamlValue::Map(maps) => {
                let mut rets = serde_json::Map::new();
                for (k, v) in maps.iter() {
                    rets.insert(k.clone(), v.to_json());
                }
                rets.into()
            }
            FamlValue::Duration(dur) => dur.to_str().into(),
            FamlValue::Distance(dis) => dis.to_str().into(),
        }
    }

    pub fn deserialize<T: for<'a> Deserialize<'a>>(&self) -> anyhow::Result<T> {
        Ok(serde_json::from_value(self.to_json())?)
    }
}

impl Index<usize> for FamlValue {
    type Output = FamlValue;
    fn index(&self, index: usize) -> &Self::Output {
        static NULL_EXPR: FamlValue = FamlValue::None;
        match self {
            FamlValue::Array(arr) => arr.get(index).unwrap_or(&NULL_EXPR),
            _ => &NULL_EXPR,
        }
    }
}

impl IndexMut<usize> for FamlValue {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            FamlValue::Array(arr) => {
                while arr.len() <= index {
                    arr.push(FamlValue::None);
                }
                arr.get_mut(index).unwrap()
            }
            _ => {
                let mut tmp = FamlValue::Array(vec![]);
                std::mem::swap(self, &mut tmp);
                self.index_mut(index)
            }
        }
    }
}

impl Index<&str> for FamlValue {
    type Output = FamlValue;
    fn index(&self, index: &str) -> &Self::Output {
        static NULL_EXPR: FamlValue = FamlValue::None;
        if index == "" {
            return self;
        } else if let Some(p) = index.find('.') {
            let (a, b) = index.split_at(p);
            self.index(a).index(&b[1..])
        } else {
            match self {
                FamlValue::Map(map) => map.get(index).unwrap_or(&NULL_EXPR),
                _ => &NULL_EXPR,
            }
        }
    }
}

impl IndexMut<&str> for FamlValue {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        if index == "" {
            return self;
        } else {
            if !self.is_map() {
                *self = FamlValue::Map(HashMap::new());
            }
            if let FamlValue::Map(map) = self {
                if map.get(index).is_none() {
                    let val = FamlValue::None;
                    map.insert(index.to_string(), val.clone());
                }
                map.get_mut(index).unwrap()
            } else {
                panic!()
            }
        }
    }
}

impl FamlValue {
    pub fn get_at(&self, index: usize) -> Option<&Self> {
        if let FamlValue::Array(arr) = self {
            arr.get(index)
        } else {
            None
        }
    }

    pub fn get_at_mut(&mut self, index: usize) -> Option<&mut Self> {
        if let FamlValue::Array(arr) = self {
            arr.get_mut(index)
        } else {
            None
        }
    }

    pub fn get(&self, index: &str) -> Option<&Self> {
        match index.split_once('.') {
            Some((a, b)) => {
                let ret = match a.parse::<usize>() {
                    Ok(i) => self.get_at(i),
                    Err(_) => self.get(a),
                };
                match ret {
                    Some(val) => val.get(b),
                    None => None,
                }
            }
            None => {
                if let FamlValue::Map(map) = self {
                    map.get(index)
                } else {
                    None
                }
            }
        }
    }

    pub fn get_mut(&mut self, index: &str) -> Option<&mut Self> {
        if let FamlValue::Map(map) = self {
            map.get_mut(index)
        } else {
            None
        }
    }

    pub fn get_with_path_mut(&mut self, path: &str) -> Option<&mut Self> {
        let path_items: Vec<_> = path.split('.').collect();
        let mut obj_ref = self;
        for path_item in path_items.into_iter() {
            if path_item.starts_with('[') {
                let num = &path_item[1..path_item.len() - 1];
                let num: usize = num.parse().unwrap();
                if let Some(obj) = obj_ref.get_at_mut(num) {
                    obj_ref = obj;
                } else {
                    return None;
                }
            }
            if let Some(obj) = obj_ref.get_mut(path_item) {
                obj_ref = obj;
            } else {
                return None;
            }
        }
        Some(obj_ref)
    }

    pub fn get_with_path(&self, path: &str) -> Option<&Self> {
        let path_items: Vec<_> = path.split('.').collect();
        let mut obj_ref = self;
        for path_item in path_items.into_iter() {
            if path_item.starts_with('[') {
                let num = &path_item[1..path_item.len() - 1];
                let num: usize = num.parse().unwrap();
                if let Some(obj) = obj_ref.get_at(num) {
                    obj_ref = obj;
                } else {
                    return None;
                }
            }
            if let Some(obj) = obj_ref.get(path_item) {
                obj_ref = obj;
            } else {
                return None;
            }
        }
        Some(obj_ref)
    }
}

pub trait ApplyExt {
    fn apply(&mut self, val: Self);
}

impl ApplyExt for HashMap<String, FamlValue> {
    fn apply(&mut self, val: Self) {
        for (key, val) in val.into_iter() {
            if let Some(self_k) = self.get_mut(&key) {
                self_k.apply(val);
            } else {
                self.insert(key, val);
            }
        }
    }
}

impl FamlValue {
    pub fn set_null(&mut self) {
        *self = FamlValue::None;
    }

    pub fn set_bool(&mut self, val: bool) {
        *self = FamlValue::Bool(val);
    }

    pub fn set_int(&mut self, val: i64) {
        *self = FamlValue::Int64(val);
    }

    pub fn set_float(&mut self, val: f64) {
        *self = FamlValue::Float64(val);
    }

    pub fn set_string(&mut self, val: impl Into<String>) {
        *self = FamlValue::String(val.into());
    }
}

impl Into<FamlValue> for () {
    fn into(self) -> FamlValue {
        FamlValue::None
    }
}

impl Into<FamlValue> for bool {
    fn into(self) -> FamlValue {
        FamlValue::Bool(self)
    }
}

impl Into<FamlValue> for i64 {
    fn into(self) -> FamlValue {
        FamlValue::Int64(self)
    }
}

impl Into<FamlValue> for f64 {
    fn into(self) -> FamlValue {
        FamlValue::Float64(self)
    }
}

impl Into<FamlValue> for String {
    fn into(self) -> FamlValue {
        FamlValue::String(self)
    }
}

impl Into<FamlValue> for Duration {
    fn into(self) -> FamlValue {
        FamlValue::Duration(self)
    }
}

impl Into<FamlValue> for Distance {
    fn into(self) -> FamlValue {
        FamlValue::Distance(self)
    }
}

#[derive(Debug, Clone)]
pub struct Distance(f64);

impl Distance {
    pub fn from_megameters(meter: f64) -> Self {
        Distance(meter * 1_000_000.0)
    }
    pub fn from_kilometers(meter: f64) -> Self {
        Distance(meter * 1_000.0)
    }
    pub fn from_meters(meter: f64) -> Self {
        Distance(meter)
    }
    pub fn from_millimeters(meter: f64) -> Self {
        Distance(meter / 1_000.0)
    }
    pub fn from_micrometers(meter: f64) -> Self {
        Distance(meter / 1_000_000.0)
    }
    pub fn from_nanometers(meter: f64) -> Self {
        Distance(meter / 1_000_000_000.0)
    }

    pub fn to_megameters(&self) -> f64 {
        self.0 / 1_000_000.0
    }
    pub fn to_kilometers(&self) -> f64 {
        self.0 / 1_000.0
    }
    pub fn to_meters(&self) -> f64 {
        self.0
    }
    pub fn to_millimeters(&self) -> f64 {
        self.0 * 1_000.0
    }
    pub fn to_micrometers(&self) -> f64 {
        self.0 * 1_000_000.0
    }
    pub fn to_nanometers(&self) -> f64 {
        self.0 * 1_000_000_000.0
    }

    pub fn to_str(&self) -> String {
        match self.0 {
            n if n < 0.000_000_1 => format!("{} nanometers", n * 1_000_000_000.0),
            n if n < 0.000_1 => format!("{} micrometers", n * 1_000_000.0),
            n if n < 1.0 => format!("{} millimeters", n * 1_000.0),
            n if n < 1_000.0 => format!("{n} meters"),
            n if n < 1_000_000.0 => format!("{} kilometers", n / 1_000.0),
            n => format!("{} megameters", n / 1_000_000.0),
        }
    }
}
