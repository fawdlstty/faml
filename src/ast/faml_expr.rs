use super::eval::{Op1Evaluator, Op2Evaluator};
use super::faml_value::FamlValue;
use crate::string_utils::IntoBaseExt;
use anyhow::anyhow;
use pest::Parser;
use pest_derive::Parser;
use serde::Deserialize;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::sync::OnceLock;

static NULL_EXPR: FamlExpr = FamlExpr::None;

fn get_op2_level(op: &str) -> usize {
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

#[derive(Parser)]
#[grammar = "../faml.pest"]
pub struct FamlParser;

#[derive(Debug, Clone)]
pub enum FamlExpr {
    None,
    Value(FamlValue),
    Array(Vec<FamlExpr>),
    Map(HashMap<String, FamlExpr>),
    TempName(String),
    Op1Prefix((String, Box<FamlExpr>)),
    Op1Suffix((Box<FamlExpr>, String)),
    Op2((Box<FamlExpr>, String, Box<FamlExpr>)),
    Op3((Box<FamlExpr>, Box<FamlExpr>, Box<FamlExpr>)),
    FormatString((Vec<String>, Vec<FamlExpr>)),
    AccessVar((Box<FamlExpr>, String)),
    InvokeFunc((Box<FamlExpr>, String, Vec<FamlExpr>)),
    IfAnno(FamlExprIfAnno),
}

#[derive(Debug, Clone)]
pub struct FamlExprIfAnno {
    pub exprs: Vec<(FamlExpr, FamlExpr)>,
    pub default: Option<Box<FamlExpr>>,
}

impl FamlExpr {
    pub fn new() -> Self {
        FamlExpr::None
    }

    pub fn make_if_anno(if_anno: FamlExpr, value: FamlExpr) -> Self {
        Self::IfAnno(FamlExprIfAnno {
            exprs: vec![(if_anno, value)],
            default: None,
        })
    }

    pub fn from_str(content: &str) -> anyhow::Result<FamlExpr> {
        let mut root = FamlParser::parse(Rule::faml, content)?;
        Self::parse_faml(root.next().ok_or(anyhow!("empty root"))?)
    }

    fn apply(&mut self, val: FamlExpr) -> anyhow::Result<()> {
        match (self, val) {
            (FamlExpr::IfAnno(if_anno), FamlExpr::IfAnno(if_anno2)) => {
                for (cond, val) in if_anno2.exprs {
                    if_anno.exprs.push((cond, val));
                }
                if if_anno.default.is_none() {
                    if_anno.default = if_anno2.default;
                }
                return Ok(());
            }
            (FamlExpr::IfAnno(if_anno), val) => {
                if_anno.default = Some(Box::new(val));
                return Ok(());
            }
            (self_, FamlExpr::IfAnno(if_anno2)) => {
                let mut self2 = FamlExpr::IfAnno(if_anno2);
                std::mem::swap(self_, &mut self2);
                if let FamlExpr::IfAnno(if_anno) = self_ {
                    if_anno.default = Some(Box::new(self2));
                }
                return Ok(());
            }
            (self_, val) => match self_ {
                FamlExpr::None => *self_ = val,
                FamlExpr::Array(arr) => {
                    if let FamlExpr::Array(arr2) = val {
                        arr.extend(arr2);
                    }
                }
                FamlExpr::Map(map) => {
                    if let FamlExpr::Map(map2) = val {
                        for (key, val) in map2.into_iter() {
                            if let Some(self_k) = map.get_mut(&key) {
                                self_k.apply(val)?;
                            } else {
                                map.insert(key, val);
                            }
                        }
                    } else {
                        *self_ = val;
                    }
                }
                _ => Err(anyhow!("disallow apply"))?,
            },
        }
        Ok(())
    }

    fn parse_faml(root: pest::iterators::Pair<'_, Rule>) -> anyhow::Result<FamlExpr> {
        let mut ret = Self::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::group_block => {
                    let val = Self::parse_block(root_item)?;
                    ret.apply(val)?;
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
        Ok(ret)
    }

    fn parse_block(root: pest::iterators::Pair<'_, Rule>) -> anyhow::Result<FamlExpr> {
        let mut anno_if_expr = None;
        let mut head = "".to_string();
        let mut is_array_head = false;
        let mut ret = HashMap::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::anno_if => {
                    anno_if_expr = Some(Self::parse_expr(root_item.into_inner().next().unwrap()))
                }
                Rule::group_head => head = Self::parse_ids(root_item),
                Rule::group_array_head => {
                    head = Self::parse_ids(root_item);
                    is_array_head = true;
                }
                Rule::assign_pair => {
                    let (key, mut value) = Self::parse_assign_pair(root_item);
                    let mut keys: Vec<_> = key.split('.').map(|key| key.to_string()).collect();
                    while keys.len() > 1 {
                        let mut tmp_map = HashMap::new();
                        tmp_map
                            .entry(keys.remove(keys.len() - 1))
                            .or_insert(FamlExpr::None)
                            .apply(value)?;
                        value = FamlExpr::Map(tmp_map);
                    }
                    ret.entry(keys.remove(0))
                        .or_insert(FamlExpr::None)
                        .apply(value)?;
                }
                _ => unreachable!(),
            }
        }
        let mut ret = FamlExpr::Map(ret);
        if is_array_head {
            ret = FamlExpr::Array(vec![ret]);
        }
        let mut keys: Vec<_> = head.split('.').map(|key| key.to_string()).collect();
        while !keys.is_empty() {
            let name = keys.remove(keys.len() - 1);
            ret = FamlExpr::Map(vec![(name, ret)].into_iter().collect());
        }
        if let Some(anno_if_expr) = anno_if_expr {
            ret = FamlExpr::IfAnno(FamlExprIfAnno {
                exprs: vec![(anno_if_expr, ret)],
                default: None,
            })
        }
        Ok(ret)
    }

    fn parse_assign_pair(root: pest::iterators::Pair<'_, Rule>) -> (String, FamlExpr) {
        let mut anno_if_expr = None;
        let mut keys = "".to_string();
        let mut value = FamlExpr::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::anno_if => {
                    // TODO
                    anno_if_expr = Some(Self::parse_expr(root_item.into_inner().next().unwrap()))
                }
                Rule::ids => keys = Self::parse_ids(root_item),
                Rule::expr => value = Self::parse_expr(root_item),
                _ => unreachable!(),
            }
        }
        if let Some(anno_if_expr) = anno_if_expr {
            value = FamlExpr::IfAnno(FamlExprIfAnno {
                exprs: vec![(anno_if_expr, value)],
                default: None,
            })
        }
        (keys, value)
    }

    fn parse_expr(root: pest::iterators::Pair<'_, Rule>) -> FamlExpr {
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::weak_expr => Self::parse_weak_expr(root_item),
            Rule::op3_expr => Self::parse_op3_expr(root_item),
            _ => unreachable!(),
        }
    }

    fn parse_base_expr(root: pest::iterators::Pair<'_, Rule>) -> FamlExpr {
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::literal => Self::parse_literal(root_item),
            Rule::ids => FamlExpr::TempName(Self::parse_ids(root_item)),
            Rule::expr => Self::parse_expr(root_item),
            _ => unreachable!(),
        }
    }

    fn parse_array_expr(root: pest::iterators::Pair<'_, Rule>) -> FamlExpr {
        let mut exprs = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::expr => exprs.push(Self::parse_expr(root_item)),
                _ => unreachable!(),
            }
        }
        FamlExpr::Array(exprs)
    }

    fn parse_map_expr(root: pest::iterators::Pair<'_, Rule>) -> FamlExpr {
        let mut map = HashMap::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::map_assign_pair => {
                    let (key, value) = Self::parse_assign_pair(root_item);
                    map.insert(key, value);
                }
                _ => unreachable!(),
            }
        }
        FamlExpr::Map(map)
    }

    fn parse_strong_expr(root: pest::iterators::Pair<'_, Rule>) -> FamlExpr {
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::base_expr => Self::parse_base_expr(root_item),
            Rule::array_expr => Self::parse_array_expr(root_item),
            Rule::map_expr => Self::parse_map_expr(root_item),
            _ => unreachable!(),
        }
    }

    fn parse_middle_expr(root: pest::iterators::Pair<'_, Rule>) -> FamlExpr {
        enum SuffixOp {
            AccessVar(String),
            InvokeFunc((String, Vec<FamlExpr>)),
            Op(String),
        }
        impl SuffixOp {
            pub fn parse(root: pest::iterators::Pair<'_, Rule>) -> Self {
                let root_str = root.as_str();
                let mut id = "".to_string();
                let mut args = None;
                for root_item in root.into_inner() {
                    match root_item.as_rule() {
                        Rule::id => id = root_item.as_str().to_string(),
                        Rule::_exprs => {
                            let mut exprs = vec![];
                            for root_item1 in root_item.into_inner() {
                                match root_item1.as_rule() {
                                    Rule::expr => exprs.push(FamlExpr::parse_expr(root_item1)),
                                    _ => unreachable!(),
                                }
                            }
                            args = Some(exprs)
                        }
                        _ => unreachable!(),
                    }
                }
                if id.is_empty() {
                    SuffixOp::Op(root_str.to_string())
                } else if let Some(args) = args {
                    SuffixOp::InvokeFunc((id, args))
                } else {
                    SuffixOp::AccessVar(id)
                }
            }
        }

        let mut expr = FamlExpr::new();
        let mut prefix_ops = vec![];
        let mut suffix_ops = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::strong_expr => expr = Self::parse_strong_expr(root_item),
                Rule::expr_prefix => prefix_ops.push(root_item.as_str().to_string()),
                Rule::expr_suffix => suffix_ops.push(SuffixOp::parse(root_item)),
                _ => unreachable!(),
            }
        }
        while !prefix_ops.is_empty() {
            let prefix_op = prefix_ops.remove(prefix_ops.len());
            expr = FamlExpr::Op1Prefix((prefix_op, Box::new(expr)));
        }
        while !suffix_ops.is_empty() {
            expr = match suffix_ops.remove(0) {
                SuffixOp::AccessVar(name) => FamlExpr::AccessVar((Box::new(expr), name)),
                SuffixOp::InvokeFunc((name, args)) => {
                    FamlExpr::InvokeFunc((Box::new(expr), name, args))
                }
                SuffixOp::Op(suffix_op) => FamlExpr::Op1Suffix((Box::new(expr), suffix_op)),
            };
        }
        expr
    }

    fn parse_weak_expr(root: pest::iterators::Pair<'_, Rule>) -> FamlExpr {
        let mut exprs = vec![];
        let mut ops = vec![];
        //
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::middle_expr => exprs.push(Self::parse_middle_expr(root_item)),
                Rule::op2 => ops.push(root_item.as_str().to_string()),
                _ => unreachable!(),
            }
        }
        let mut ops: Vec<_> = ops
            .into_iter()
            .map(|op| {
                let level = get_op2_level(&op[..]);
                (op, level)
            })
            .collect();
        //
        for i in 0..10 {
            if exprs.len() == 1 {
                break;
            }
            if i == 5 {
                for j in 1..ops.len() {
                    if ops[j - i].1 == i && ops[j].1 == i {
                        exprs.insert(j, exprs[j].clone());
                        ops.insert(j, ("&&".to_string(), get_op2_level("&&")));
                    }
                }
            }
            for idx in 0..ops.len() {
                if let Some((_, level)) = ops.get(idx) {
                    if *level != i {
                        continue;
                    }
                }
                let left = exprs.remove(idx);
                let right = exprs.remove(idx);
                let op = ops.remove(idx).0;
                let expr = FamlExpr::Op2((Box::new(left), op, Box::new(right)));
                exprs.insert(idx, expr);
            }
        }
        exprs.remove(0)
    }

    fn parse_op3_expr(root: pest::iterators::Pair<'_, Rule>) -> FamlExpr {
        let mut exprs = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::middle_expr => exprs.push(Self::parse_middle_expr(root_item)),
                _ => unreachable!(),
            }
        }
        let expr1 = Box::new(exprs.remove(0));
        let expr2 = Box::new(exprs.remove(0));
        let expr3 = Box::new(exprs.remove(0));
        FamlExpr::Op3((expr1, expr2, expr3))
    }

    fn parse_literal(root: pest::iterators::Pair<'_, Rule>) -> FamlExpr {
        let root_item = root.into_inner().next().unwrap();
        FamlExpr::Value(match root_item.as_rule() {
            Rule::boolean_literal => FamlValue::Bool(root_item.as_str() == "true"),
            Rule::number_literal => match root_item.as_str().parse::<i64>() {
                Ok(n) => FamlValue::Int64(n),
                Err(_) => FamlValue::String(root_item.as_str().into_base()),
            },
            Rule::string_literal => FamlValue::String(root_item.as_str().into_base()),
            Rule::format_string_literal => return Self::parse_format_string_literal(root_item),
            _ => unreachable!(),
        })
    }

    fn parse_format_string_literal(root: pest::iterators::Pair<'_, Rule>) -> FamlExpr {
        let mut strs = vec![];
        let mut exprs = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::format_string => {
                    return FamlExpr::Value(FamlValue::String(root_item.as_str().into_base()));
                }
                Rule::format_string_part1 => strs.push(root_item.as_str().into_base()),
                Rule::format_string_part2 => strs.push(root_item.as_str().into_base()),
                Rule::format_string_part3 => strs.push(root_item.as_str().into_base()),
                Rule::expr => exprs.push(Self::parse_expr(root_item)),
                _ => unreachable!(),
            }
        }
        FamlExpr::FormatString((strs, exprs))
    }

    fn parse_ids(root: pest::iterators::Pair<'_, Rule>) -> String {
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::ids => root_item.as_str().to_string(),
            Rule::id => root_item.as_str().to_string(),
            _ => unreachable!(),
        }
    }

    pub fn root_evalute(&self, path: &str) -> anyhow::Result<FamlValue> {
        self[path].evalute_cb(path, &|path| self.root_evalute(path))
    }

    pub fn evalute_cb(
        &self,
        path: &str,
        calc_cb: &impl Fn(&str) -> anyhow::Result<FamlValue>,
    ) -> anyhow::Result<FamlValue> {
        Ok(match self {
            FamlExpr::None => FamlValue::None,
            FamlExpr::Value(val) => val.clone(),
            FamlExpr::Array(arr) => {
                let mut ret = vec![];
                for (index, item) in arr.iter().enumerate() {
                    let val = item.evalute_cb(&path.append_num(index), calc_cb)?;
                    ret.push(val);
                }
                FamlValue::Array(ret)
            }
            FamlExpr::Map(map) => {
                let mut ret = HashMap::new();
                for (key, item) in map.iter() {
                    let val = item.evalute_cb(&path.append_str(key), calc_cb)?;
                    ret.insert(key.clone(), val);
                }
                FamlValue::Map(ret)
            }
            FamlExpr::TempName(name) => calc_cb(&path.remove_once().append_str(name))?,
            FamlExpr::Op1Prefix((name, expr)) => {
                let val = expr.evalute_cb(path, calc_cb)?;
                Op1Evaluator::eval_prefix(name, val)?
            }
            FamlExpr::Op1Suffix((expr, name)) => {
                let val = expr.evalute_cb(path, calc_cb)?;
                Op1Evaluator::eval_suffix(name, val)?
            }
            FamlExpr::Op2((left, op, right)) => {
                let left = left.evalute_cb(path, calc_cb)?;
                let right = right.evalute_cb(path, calc_cb)?;
                Op2Evaluator::eval(left, op, right)?
            }
            FamlExpr::Op3((cond, left, right)) => {
                let cond = cond.evalute_cb(path, calc_cb)?;
                let val = match cond.as_bool() {
                    Some(true) => left,
                    Some(false) => right,
                    None => return Err(anyhow!("condition is not bool")),
                };
                val.evalute_cb(path, calc_cb)?
            }
            FamlExpr::FormatString((strs, exprs)) => {
                let mut exprs1 = vec![];
                for item in exprs.iter() {
                    let val = item.evalute_cb(path, calc_cb)?;
                    exprs1.push(val);
                }
                exprs1.push(FamlValue::String("".to_string()));
                let mut ret = "".to_string();
                for (a, b) in strs.iter().zip(exprs1.iter()) {
                    ret.push_str(a);
                    ret.push_str(&b.as_str());
                }
                FamlValue::String(ret)
            }
            FamlExpr::AccessVar(_) => todo!(),
            FamlExpr::InvokeFunc(_) => todo!(),
            FamlExpr::IfAnno(if_anno) => {
                let mut ret = None;
                for (cond, value) in if_anno.exprs.iter() {
                    let val = cond.evalute_cb(path, calc_cb)?;
                    match val.as_bool().unwrap_or(false) {
                        true => {
                            ret = value.evalute_cb(path, calc_cb).ok();
                            break;
                        }
                        false => return Err(anyhow!("condition is not bool")),
                    }
                }
                ret.unwrap_or(match &if_anno.default {
                    Some(val) => val.evalute_cb(path, calc_cb)?,
                    None => FamlValue::None,
                })
            }
        })
    }

    pub fn evalute(&self) -> anyhow::Result<FamlValue> {
        let mut last_result = FamlValue::None;
        let mut count = 3;
        while count >= 0 {
            count -= 1;
            match self.evalute2("", &last_result)? {
                (result, true) => return Ok(result),
                (result, false) => last_result = result,
            }
        }
        Err(anyhow!("evalute failed"))
    }

    fn evalute2(&self, path: &str, last_result: &FamlValue) -> anyhow::Result<(FamlValue, bool)> {
        let mut success = true;
        let value = match self {
            FamlExpr::None => FamlValue::None,
            FamlExpr::Value(val) => val.clone(),
            FamlExpr::Array(arr) => {
                let mut ret = vec![];
                for (index, item) in arr.iter().enumerate() {
                    let new_path = path.append_num(index);
                    let (val, tmp_success) = item.evalute2(&new_path, last_result)?;
                    ret.push(val);
                    success &= tmp_success;
                }
                FamlValue::Array(ret)
            }
            FamlExpr::Map(map) => {
                let mut ret = HashMap::new();
                for (key, value) in map.iter() {
                    let new_path = path.append_str(key);
                    let (val, tmp_success) = value.evalute2(&new_path, last_result)?;
                    ret.insert(key.clone(), val);
                    success &= tmp_success;
                }
                FamlValue::Map(ret)
            }
            FamlExpr::TempName(name) => {
                match last_result.get(&path.remove_once().append_str(name)) {
                    Some(val) => val.clone(),
                    None => {
                        success = false;
                        FamlValue::None
                    }
                }
            }
            FamlExpr::Op1Prefix((name, expr)) => {
                let (val, tmp_success) = expr.evalute2(path, last_result)?;
                success &= tmp_success;
                if tmp_success {
                    Op1Evaluator::eval_prefix(name, val)?
                } else {
                    FamlValue::None
                }
            }
            FamlExpr::Op1Suffix((expr, name)) => {
                let (val, tmp_success) = expr.evalute2(path, last_result)?;
                success &= tmp_success;
                if tmp_success {
                    Op1Evaluator::eval_suffix(name, val)?
                } else {
                    FamlValue::None
                }
            }
            FamlExpr::Op2((left, op, right)) => {
                let (left, tmp_success1) = left.evalute2(path, last_result)?;
                success &= tmp_success1;
                let (right, tmp_success2) = right.evalute2(path, last_result)?;
                success &= tmp_success2;
                if tmp_success1 && tmp_success2 {
                    Op2Evaluator::eval(left, op, right)?
                } else {
                    FamlValue::None
                }
            }
            FamlExpr::Op3((cond, left, right)) => {
                let (cond, tmp_success) = cond.evalute2(path, last_result)?;
                if tmp_success {
                    let (value, tmp_success) = match cond.as_bool() {
                        Some(true) => left.evalute2(path, last_result)?,
                        Some(false) => right.evalute2(path, last_result)?,
                        None => return Err(anyhow!("condition must be boolean")),
                    };
                    success &= tmp_success;
                    value
                } else {
                    FamlValue::None
                }
            }
            FamlExpr::FormatString((strs, exprs)) => {
                let mut exprs1 = vec![];
                let mut tmp_success = true;
                for item in exprs.iter() {
                    let (val, tmp_success1) = item.evalute2(path, last_result)?;
                    exprs1.push(val);
                    tmp_success &= tmp_success1;
                }
                if tmp_success {
                    exprs1.push(FamlValue::String("".to_string()));
                    let mut ret = "".to_string();
                    for (a, b) in strs.iter().zip(exprs1.iter()) {
                        ret.push_str(a);
                        ret.push_str(&b.as_str());
                    }
                    FamlValue::String(ret)
                } else {
                    success = false;
                    FamlValue::None
                }
            }
            FamlExpr::AccessVar(_) => todo!(),
            FamlExpr::InvokeFunc(_) => todo!(),
            FamlExpr::IfAnno(if_anno) => {
                for (cond, value) in if_anno.exprs.iter() {
                    let (cond_val, tmp_success) = cond.evalute2(path, last_result)?;
                    if tmp_success {
                        if cond_val.as_bool().unwrap_or(false) {
                            return value.evalute2(path, last_result);
                        }
                    } else {
                        success = false;
                    }
                }
                match &if_anno.default {
                    Some(val) => {
                        return val.evalute2(path, last_result);
                    }
                    None => FamlValue::None,
                }
            }
        };
        Ok((value, success))
    }

    pub fn deserialize<T: for<'a> Deserialize<'a>>(&self) -> anyhow::Result<T> {
        Ok(serde_json::from_value(self.evalute()?.to_json())?)
    }
}

pub(crate) trait PathAppendExt {
    fn append_str(&self, path: &str) -> String;
    fn append_num(&self, num: usize) -> String;
    fn remove_once(&self) -> &str;
}

impl PathAppendExt for str {
    fn append_str(&self, name: &str) -> String {
        match name {
            "root" => "".to_string(),
            "super" => self.remove_once().to_string(),
            _ => match self.is_empty() {
                true => name.to_string(),
                false => format!("{self}.{name}"),
            },
        }
    }

    fn append_num(&self, num: usize) -> String {
        match self.is_empty() {
            true => num.to_string(),
            false => format!("{self}.{num}"),
        }
    }

    fn remove_once(&self) -> &str {
        match self.rfind('.') {
            Some(pos) => &self[0..pos],
            None => "",
        }
    }
}

impl Index<usize> for FamlExpr {
    type Output = FamlExpr;
    fn index(&self, index: usize) -> &Self::Output {
        self.get_at(index).unwrap_or(&NULL_EXPR)
    }
}

impl IndexMut<usize> for FamlExpr {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_at_mut(index)
    }
}

impl Index<&str> for FamlExpr {
    type Output = FamlExpr;
    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap_or(&NULL_EXPR)
    }
}

impl IndexMut<&str> for FamlExpr {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl FamlExpr {
    pub fn get_at(&self, index: usize) -> Option<&Self> {
        if let FamlExpr::Array(arr) = self {
            arr.get(index)
        } else {
            None
        }
    }

    pub fn get_at_mut(&mut self, index: usize) -> &mut Self {
        if let FamlExpr::Array(arr) = self {
            if (index + 1) > arr.len() {
                arr.extend(
                    (arr.len()..(index + 1))
                        .into_iter()
                        .map(|_| FamlExpr::new())
                        .collect::<Vec<_>>(),
                )
            }
        } else {
            *self = FamlExpr::Array(
                (0..(index + 1))
                    .into_iter()
                    .map(|_| FamlExpr::new())
                    .collect(),
            );
        }
        if let FamlExpr::Array(arr) = self {
            arr.get_mut(index).unwrap()
        } else {
            panic!()
        }
    }

    pub fn get(&self, index: &str) -> Option<&Self> {
        let path_items: Vec<_> = index.split('.').collect();
        let mut obj_ref = self;
        for path_item in path_items.into_iter() {
            if path_item.len() == 0 {
                continue;
            } else if path_item.starts_with('[') {
                let num = &path_item[1..path_item.len() - 1];
                if let Ok(num) = num.parse() {
                    if let Some(obj) = obj_ref.get_at(num) {
                        obj_ref = obj;
                        continue;
                    }
                }
                return None;
            } else {
                if let FamlExpr::Map(map) = obj_ref {
                    if let Some(obj) = map.get(path_item) {
                        obj_ref = obj;
                        continue;
                    }
                }
                return None;
            }
        }
        Some(obj_ref)
    }

    pub fn get_mut(&mut self, index: &str) -> &mut Self {
        let path_items: Vec<_> = index.split('.').collect();
        let mut obj_ref = self;
        for path_item in path_items.into_iter() {
            if path_item.len() == 0 {
                continue;
            } else if path_item.starts_with('[') {
                let num = &path_item[1..path_item.len() - 1];
                if let Ok(num) = num.parse() {
                    obj_ref = obj_ref.get_at_mut(num);
                    continue;
                }
            } else {
                let map = match obj_ref {
                    FamlExpr::Map(map) => map,
                    _ => {
                        *obj_ref = FamlExpr::Map(HashMap::new());
                        match obj_ref {
                            FamlExpr::Map(map) => map,
                            _ => panic!(),
                        }
                    }
                };
                obj_ref = map.entry(path_item.to_string()).or_insert(FamlExpr::new());
            }
        }
        obj_ref
    }
}

impl FamlExpr {
    pub fn is_map(&self) -> bool {
        match self {
            FamlExpr::Map(_) => true,
            _ => false,
        }
    }

    pub fn set_null(&mut self) {
        *self = FamlExpr::None;
    }

    pub fn set_bool(&mut self, val: bool) {
        *self = FamlExpr::Value(FamlValue::Bool(val));
    }

    pub fn set_int(&mut self, val: i64) {
        *self = FamlExpr::Value(FamlValue::Int64(val));
    }

    pub fn set_float(&mut self, val: f64) {
        *self = FamlExpr::Value(FamlValue::Float64(val));
    }

    pub fn set_string(&mut self, val: impl Into<String>) {
        *self = FamlExpr::Value(FamlValue::String(val.into()));
    }

    pub fn wrap(&'_ self) -> FamlExprWrap<'_> {
        FamlExprWrap {
            expr: self,
            path: UnsafeCell::new("".to_string()),
        }
    }
}

pub struct FamlExprWrap<'a> {
    expr: &'a FamlExpr,
    path: UnsafeCell<String>,
}

impl<'a> Index<usize> for FamlExprWrap<'a> {
    type Output = FamlExprWrap<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        let path = unsafe { &mut *self.path.get() };
        *path = match path.len() {
            0 => format!("[{index}]"),
            _ => format!("{path}.[{index}]"),
        };
        self
    }
}

impl<'a> IndexMut<usize> for FamlExprWrap<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let path = self.path.get_mut();
        *path = match path.len() {
            0 => format!("[{index}]"),
            _ => format!("{path}.[{index}]"),
        };
        self
    }
}

impl<'a> Index<&str> for FamlExprWrap<'a> {
    type Output = FamlExprWrap<'a>;
    fn index(&self, index: &str) -> &Self::Output {
        let path = unsafe { &mut *self.path.get() };
        *path = match path.len() {
            0 => index.to_string(),
            _ => format!("{path}.{index}"),
        };
        self
    }
}

impl<'a> IndexMut<&str> for FamlExprWrap<'a> {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        let path = self.path.get_mut();
        *path = match path.len() {
            0 => index.to_string(),
            _ => format!("{path}.{index}"),
        };
        self
    }
}

impl<'a> FamlExprWrap<'a> {
    pub fn evalute(&self) -> anyhow::Result<FamlValue> {
        let path = unsafe { &*self.path.get() }.clone();
        self.expr.root_evalute(&path[..])
    }
}
