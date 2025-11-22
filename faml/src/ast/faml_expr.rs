use super::eval::{Op1Evaluator, Op2Evaluator};
use super::faml_value::FamlValue;
use crate::Native;
use crate::ast::invoke::InvokeExt;
use crate::string_utils::IntoBaseExt;
use anyhow::anyhow;
use pest::Parser;
use pest_derive::Parser;
use serde::Deserialize;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::sync::{Arc, OnceLock, Weak};

#[derive(Parser)]
#[grammar = "../faml.pest"]
pub struct FamlParser;

#[derive(Debug, Clone)]
pub enum FamlExprImpl {
    None,
    Value(FamlValue),
    Array(Vec<FamlExpr>),
    Map(HashMap<String, FamlExpr>),
    TempName(Vec<String>),
    Op1Prefix((String, FamlExpr)),
    Op1Suffix((FamlExpr, String)),
    Op2((FamlExpr, String, FamlExpr)),
    Op3((FamlExpr, FamlExpr, FamlExpr)),
    FormatString((Vec<String>, Vec<FamlExpr>)),
    AccessVar((FamlExpr, FamlExpr)),
    InvokeFunc((FamlExpr, Vec<FamlExpr>)),
    IfAnno(FamlExprIfAnno),
}

#[derive(Debug, Clone)]
pub struct FamlExprIfAnno {
    pub ifcond_values: Vec<(FamlExpr, FamlExpr)>,
    pub default_value: FamlExpr,
}

#[derive(Debug, Clone)]
pub struct FamlExprBase {
    expr: FamlExprImpl,
    base_expr: WeakFamlExpr,
    super_expr: WeakFamlExpr,
}

#[derive(Debug, Clone)]
pub struct FamlExpr(Arc<UnsafeCell<FamlExprBase>>);
unsafe impl Send for FamlExpr {}
unsafe impl Sync for FamlExpr {}

#[derive(Debug, Clone)]
pub struct WeakFamlExpr(Weak<UnsafeCell<FamlExprBase>>);

impl WeakFamlExpr {
    pub fn upgrade(&self) -> anyhow::Result<FamlExpr> {
        let expr = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("base node not found"))?;
        Ok(FamlExpr(expr))
    }
}

impl FamlExprIfAnno {
    pub fn init_weak_expr(&mut self, base_expr: WeakFamlExpr, super_expr: WeakFamlExpr) {
        for (expr0, expr1) in &mut self.ifcond_values {
            expr0.init_weak_expr(base_expr.clone(), super_expr.clone());
            expr1.init_weak_expr(base_expr.clone(), super_expr.clone());
        }
        self.default_value
            .init_weak_expr(base_expr.clone(), super_expr.clone());
    }
}

impl FamlExpr {
    fn empty() -> &'static FamlExpr {
        static FAML_EMPTY: OnceLock<FamlExpr> = OnceLock::new();
        FAML_EMPTY.get_or_init(|| FamlExpr::new())
    }

    pub fn new() -> Self {
        FamlExprImpl::None.to_expr()
    }

    pub fn base(&self) -> &FamlExprBase {
        unsafe { &*self.0.get() }
    }

    pub fn base_mut(&mut self) -> &mut FamlExprBase {
        unsafe { &mut *self.0.get() }
    }

    pub fn into_base(self) -> FamlExprBase {
        match Arc::try_unwrap(self.0) {
            Ok(data) => data.into_inner(),
            Err(p) => unsafe { &*p.get() }.clone(),
        }
    }

    fn to_weak(&self) -> WeakFamlExpr {
        WeakFamlExpr(Arc::downgrade(&(self.0)))
    }

    fn is_none(&self) -> bool {
        match self.base().expr {
            FamlExprImpl::None => true,
            _ => false,
        }
    }
}

impl FamlExprBase {
    pub fn new() -> Self {
        FamlExprImpl::None.to_base()
    }

    pub fn to_expr(self) -> FamlExpr {
        FamlExpr(Arc::new(UnsafeCell::new(self)))
    }
}

impl FamlExprImpl {
    pub fn to_base(self) -> FamlExprBase {
        FamlExprBase {
            expr: self,
            base_expr: WeakFamlExpr(Weak::new()),
            super_expr: WeakFamlExpr(Weak::new()),
        }
    }

    pub fn to_expr(self) -> FamlExpr {
        self.to_base().to_expr()
    }
}

impl FamlExpr {
    pub fn get_at(&self, index: usize) -> Option<&Self> {
        let expr_impl = &unsafe { &*self.0.get() }.expr;
        if let FamlExprImpl::Array(arr) = expr_impl {
            return Some(&arr[index]);
        }
        return None;
    }

    pub fn get_at_mut(&mut self, index: usize) -> &mut Self {
        let expr_impl = &mut unsafe { &mut *self.0.get() }.expr;
        let arr = match expr_impl {
            FamlExprImpl::Array(arr) => arr,
            _ => {
                *expr_impl = FamlExprImpl::Array(vec![]);
                match expr_impl {
                    FamlExprImpl::Array(arr) => arr,
                    _ => panic!(),
                }
            }
        };
        if (index + 1) > arr.len() {
            arr.extend(
                (arr.len()..(index + 1))
                    .into_iter()
                    .map(|_| FamlExpr::new())
                    .collect::<Vec<_>>(),
            )
        }
        return &mut arr[index];
    }

    pub fn get(&self, index: &str) -> Option<&Self> {
        let paths: Vec<_> = index
            .split('.')
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .collect();
        if paths.is_empty() {
            return None;
        }
        let mut obj_ref = self;
        for path in paths {
            if path.starts_with('[') && path.ends_with(']') {
                let num: usize = path[1..path.len() - 1].parse().ok()?;
                obj_ref = obj_ref.get_at(num)?;
            } else {
                let map = match &obj_ref.base().expr {
                    FamlExprImpl::Map(map) => map,
                    _ => return None,
                };
                obj_ref = map.get(path)?;
            }
        }
        Some(obj_ref)
    }

    pub fn get_mut(&mut self, index: &str) -> &mut Self {
        let paths: Vec<_> = index
            .split('.')
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .collect();
        let mut obj_ref = self;
        for path in paths {
            if path.starts_with('[') && path.ends_with(']') {
                let num: usize = path[1..path.len() - 1].parse().unwrap_or(0);
                obj_ref = obj_ref.get_at_mut(num);
            } else {
                let expr_impl = &mut obj_ref.base_mut().expr;
                let map = match expr_impl {
                    FamlExprImpl::Map(map) => map,
                    _ => {
                        *expr_impl = FamlExprImpl::Map(HashMap::new());
                        match expr_impl {
                            FamlExprImpl::Map(map) => map,
                            _ => panic!(),
                        }
                    }
                };
                obj_ref = map.entry(path.to_string()).or_insert_with(FamlExpr::new);
            }
        }
        obj_ref
    }

    pub fn make_if_anno(if_anno: FamlExpr, value: FamlExpr) -> Self {
        FamlExprImpl::IfAnno(FamlExprIfAnno {
            ifcond_values: vec![(if_anno, value)],
            default_value: FamlExpr::new(),
        })
        .to_expr()
    }
}

impl FamlExprImpl {
    pub fn is_map(&self) -> bool {
        match self {
            FamlExprImpl::Map(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            FamlExprImpl::Array(_) => true,
            _ => false,
        }
    }

    pub fn set_null(&mut self) {
        *self = FamlExprImpl::None;
    }

    pub fn set_bool(&mut self, val: bool) {
        *self = FamlExprImpl::Value(FamlValue::Bool(val));
    }

    pub fn set_int(&mut self, val: i64) {
        *self = FamlExprImpl::Value(FamlValue::Int64(val));
    }

    pub fn set_float(&mut self, val: f64) {
        *self = FamlExprImpl::Value(FamlValue::Float64(val));
    }

    pub fn set_string(&mut self, val: impl Into<String>) {
        *self = FamlExprImpl::Value(FamlValue::String(val.into()));
    }
}

impl Index<usize> for FamlExpr {
    type Output = Self;
    fn index(&self, index: usize) -> &Self::Output {
        self.get_at(index).unwrap_or(Self::empty())
    }
}

impl IndexMut<usize> for FamlExpr {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_at_mut(index)
    }
}

impl Index<&str> for FamlExpr {
    type Output = Self;
    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap_or(Self::empty())
    }
}

impl IndexMut<&str> for FamlExpr {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl FamlExpr {
    pub fn from_str(content: &str) -> anyhow::Result<Self> {
        let mut root = FamlParser::parse(Rule::faml, content)?;
        let mut expr = match root.next() {
            Some(root) => Self::parse_faml(root),
            None => Err(anyhow!("cannot parse content")),
        }?;
        let base_expr = expr.to_weak();
        expr.init_weak_expr(base_expr.clone(), base_expr);
        Ok(expr)
    }

    pub fn from_json(root: serde_json::Value) -> anyhow::Result<Self> {
        let mut expr = FamlValue::from_json(root)?.to_expr();
        let base_expr = expr.to_weak();
        expr.init_weak_expr(base_expr.clone(), base_expr);
        Ok(expr)
    }

    pub fn from_yaml(root: serde_yaml::Value) -> anyhow::Result<Self> {
        let mut expr = FamlValue::from_yaml(root)?.to_expr();
        let base_expr = expr.to_weak();
        expr.init_weak_expr(base_expr.clone(), base_expr);
        Ok(expr)
    }

    fn parse_faml(root: pest::iterators::Pair<'_, Rule>) -> anyhow::Result<Self> {
        let mut ret = FamlExpr::new();
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

    fn parse_block(root: pest::iterators::Pair<'_, Rule>) -> anyhow::Result<Self> {
        let mut anno_if_expr = None;
        let mut head = vec![];
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
                            .or_insert(FamlExpr::new())
                            .apply(value)?;
                        value = FamlExprImpl::Map(tmp_map).to_expr();
                    }
                    ret.entry(keys.remove(0))
                        .or_insert(FamlExpr::new())
                        .apply(value)?;
                }
                _ => unreachable!(),
            }
        }
        let mut ret = FamlExprImpl::Map(ret).to_expr();
        if is_array_head {
            ret = FamlExprImpl::Array(vec![ret]).to_expr();
        }
        while !head.is_empty() {
            let name = head.remove(head.len() - 1);
            ret = FamlExprImpl::Map(vec![(name, ret)].into_iter().collect()).to_expr();
        }
        if let Some(anno_if_expr) = anno_if_expr {
            ret = FamlExprImpl::IfAnno(FamlExprIfAnno {
                ifcond_values: vec![(anno_if_expr, ret)],
                default_value: FamlExpr::new(),
            })
            .to_expr()
        }
        Ok(ret)
    }

    fn parse_assign_pair(root: pest::iterators::Pair<'_, Rule>) -> (String, Self) {
        let mut anno_if_expr = None;
        let mut keys = vec![];
        let mut value = FamlExpr::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::anno_if => {
                    anno_if_expr = Some(Self::parse_expr(root_item.into_inner().next().unwrap()))
                }
                Rule::ids => keys = Self::parse_ids(root_item),
                Rule::expr => value = Self::parse_expr(root_item),
                _ => unreachable!(),
            }
        }
        if let Some(anno_if_expr) = anno_if_expr {
            value = FamlExprImpl::IfAnno(FamlExprIfAnno {
                ifcond_values: vec![(anno_if_expr, value)],
                default_value: FamlExpr::new(),
            })
            .to_expr();
        }
        while keys.len() > 1 {
            let mut tmp_map = HashMap::new();
            tmp_map.insert(keys.remove(keys.len() - 1), value);
            value = FamlExprImpl::Map(tmp_map).to_expr();
        }
        (keys.remove(0), value)
    }

    fn parse_expr(root: pest::iterators::Pair<'_, Rule>) -> Self {
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::weak_expr => Self::parse_weak_expr(root_item),
            Rule::op3_expr => Self::parse_op3_expr(root_item),
            _ => unreachable!(),
        }
    }

    fn parse_base_expr(root: pest::iterators::Pair<'_, Rule>) -> Self {
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::literal => Self::parse_literal(root_item),
            Rule::ids => FamlExprImpl::TempName(Self::parse_ids(root_item)).to_expr(),
            _ => unreachable!(),
        }
    }

    fn parse_array_expr(root: pest::iterators::Pair<'_, Rule>) -> Self {
        let mut exprs = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::expr => exprs.push(Self::parse_expr(root_item)),
                Rule::exprs => return Self::parse_array_expr(root_item),
                _ => unreachable!(),
            }
        }
        FamlExprImpl::Array(exprs).to_expr()
    }

    fn parse_map_expr(root: pest::iterators::Pair<'_, Rule>) -> Self {
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
        FamlExprImpl::Map(map).to_expr()
    }

    fn parse_strong_expr(root: pest::iterators::Pair<'_, Rule>) -> Self {
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::base_expr => Self::parse_base_expr(root_item),
            Rule::array_expr => Self::parse_array_expr(root_item),
            Rule::map_expr => Self::parse_map_expr(root_item),
            _ => unreachable!(),
        }
    }

    fn parse_middle_expr(root: pest::iterators::Pair<'_, Rule>) -> Self {
        enum SuffixOp {
            AccessVar(FamlExpr),
            InvokeFunc(Vec<FamlExpr>),
            Op(String),
        }
        impl SuffixOp {
            pub fn parse(root: pest::iterators::Pair<'_, Rule>) -> Self {
                let root_str = root.as_str();
                let mut args = None;
                for root_item in root.into_inner() {
                    match root_item.as_rule() {
                        Rule::num_unit => return SuffixOp::Op(root_str.to_string()),
                        Rule::exprs => {
                            let mut exprs = vec![];
                            for root_item1 in root_item.into_inner() {
                                match root_item1.as_rule() {
                                    Rule::expr => exprs.push(FamlExpr::parse_expr(root_item1)),
                                    _ => unreachable!(),
                                }
                            }
                            args = Some(exprs)
                        }
                        Rule::expr => return SuffixOp::AccessVar(FamlExpr::parse_expr(root_item)),
                        _ => unreachable!(),
                    }
                }
                if let Some(args) = args {
                    SuffixOp::InvokeFunc(args)
                } else {
                    SuffixOp::Op(root_str.to_string())
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
        while let Some(prefix_op) = prefix_ops.pop() {
            expr = FamlExprImpl::Op1Prefix((prefix_op, expr)).to_expr();
        }
        while !suffix_ops.is_empty() {
            expr = match suffix_ops.remove(0) {
                SuffixOp::AccessVar(arg) => FamlExprImpl::AccessVar((expr, arg)),
                SuffixOp::InvokeFunc(args) => FamlExprImpl::InvokeFunc((expr, args)),
                SuffixOp::Op(suffix_op) => FamlExprImpl::Op1Suffix((expr, suffix_op)),
            }
            .to_expr();
        }
        expr
    }

    fn parse_weak_expr(root: pest::iterators::Pair<'_, Rule>) -> Self {
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
                let level = Op2Evaluator::get_level(&op[..]);
                (op, level)
            })
            .collect();
        //
        for i in 0..10 {
            if exprs.len() == 1 {
                break;
            }
            if i == 5 {
                let mut j = 1;
                while j < ops.len() {
                    if ops[j - 1].1 == i && ops[j].1 == i {
                        exprs.insert(j, exprs[j].clone());
                        ops.insert(j, ("&&".to_string(), Op2Evaluator::get_level("&&")));
                        j += 2; // Skip the newly inserted elements
                    } else {
                        j += 1;
                    }
                }
            }
            let mut idx = 0;
            while idx < ops.len() {
                if let Some((_, level)) = ops.get(idx) {
                    if *level != i {
                        idx += 1;
                        continue;
                    }
                }
                // Check bounds before removing elements
                if idx >= exprs.len() || idx + 1 >= exprs.len() || idx >= ops.len() {
                    break;
                }
                let left = exprs.remove(idx);
                let right = exprs.remove(idx);
                let op = ops.remove(idx).0;
                let expr = FamlExprImpl::Op2((left, op, right)).to_expr();
                exprs.insert(idx, expr);
                // Don't increment idx since we've modified the vectors
            }
        }
        exprs.remove(0)
    }

    fn parse_op3_expr(root: pest::iterators::Pair<'_, Rule>) -> Self {
        let mut exprs = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::middle_expr => exprs.push(Self::parse_middle_expr(root_item)),
                Rule::weak_expr => exprs.push(Self::parse_weak_expr(root_item)),
                _ => unreachable!(),
            }
        }
        let expr1 = exprs.remove(0);
        let expr2 = exprs.remove(0);
        let expr3 = exprs.remove(0);
        FamlExprImpl::Op3((expr1, expr2, expr3)).to_expr()
    }

    fn parse_literal(root: pest::iterators::Pair<'_, Rule>) -> Self {
        let root_item = root.into_inner().next().unwrap();
        FamlExprImpl::Value(match root_item.as_rule() {
            Rule::boolean_literal => FamlValue::Bool(root_item.as_str() == "true"),
            Rule::number_literal => match root_item.as_str().parse::<i64>() {
                Ok(n) => FamlValue::Int64(n),
                Err(_) => match root_item.as_str().parse::<f64>() {
                    Ok(f) => FamlValue::Float64(f),
                    Err(_) => FamlValue::String(root_item.as_str().into_base()),
                },
            },
            Rule::string_literal => FamlValue::String(root_item.as_str().into_base()),
            Rule::format_string_literal => return Self::parse_format_string_literal(root_item),
            _ => unreachable!(),
        })
        .to_expr()
    }

    fn parse_format_string_literal(root: pest::iterators::Pair<'_, Rule>) -> Self {
        let mut strs = vec![];
        let mut exprs = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::format_string => {
                    return FamlExprImpl::Value(FamlValue::String(root_item.as_str().into_base()))
                        .to_expr();
                }
                Rule::format_string_part1 => strs.push(root_item.as_str().into_base()),
                Rule::format_string_part2 => strs.push(root_item.as_str().into_base()),
                Rule::format_string_part3 => strs.push(root_item.as_str().into_base()),
                Rule::expr => exprs.push(Self::parse_expr(root_item)),
                _ => unreachable!(),
            }
        }
        FamlExprImpl::FormatString((strs, exprs)).to_expr()
    }

    fn parse_ids(root: pest::iterators::Pair<'_, Rule>) -> Vec<String> {
        let mut ret = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::ids => {
                    let ids: Vec<_> = root_item.as_str().split('.').collect();
                    for id in ids {
                        ret.push(id.trim().to_string());
                    }
                }
                Rule::id => ret.push(root_item.as_str().trim().to_string()),
                _ => unreachable!(),
            };
        }
        ret
    }

    pub fn apply(&mut self, val: Self) -> anyhow::Result<()> {
        let expr1_impl = &mut self.base_mut().expr;
        let expr2_impl = val.into_base().expr;
        match (expr1_impl, expr2_impl) {
            (FamlExprImpl::IfAnno(if_anno), FamlExprImpl::IfAnno(if_anno2)) => {
                for (cond, val) in if_anno2.ifcond_values {
                    if_anno.ifcond_values.push((cond, val));
                }
                if if_anno.default_value.is_none() {
                    if_anno.default_value = if_anno2.default_value;
                }
                return Ok(());
            }
            (FamlExprImpl::IfAnno(if_anno), expr2_impl) => {
                if_anno.default_value = expr2_impl.to_expr();
                return Ok(());
            }
            (self_, FamlExprImpl::IfAnno(if_anno2)) => {
                let mut self2 = FamlExprImpl::IfAnno(if_anno2);
                std::mem::swap(self_, &mut self2);
                if let FamlExprImpl::IfAnno(if_anno) = self_ {
                    if_anno.default_value = self2.to_expr();
                }
                return Ok(());
            }
            (self_, val) => match self_ {
                FamlExprImpl::None => *self_ = val,
                FamlExprImpl::Array(arr) => {
                    if let FamlExprImpl::Array(arr2) = val {
                        arr.extend(arr2);
                    }
                }
                FamlExprImpl::Map(map) => {
                    if let FamlExprImpl::Map(map2) = val {
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
                _ => *self_ = val,
            },
        }
        let base_expr = self.to_weak();
        self.init_weak_expr(base_expr.clone(), base_expr);
        Ok(())
    }

    fn get_temp_name_expr(&self, names: &Vec<String>) -> anyhow::Result<FamlExpr> {
        let mut names: Vec<_> = names.iter().map(|p| &p[..]).collect();
        let mut expr = match names.first() {
            Some(&"nan") => {
                names.remove(0);
                FamlValue::Float64(f64::NAN).to_expr()
            }
            Some(&"infinity") => {
                names.remove(0);
                FamlValue::Float64(f64::INFINITY).to_expr()
            }
            Some(&"null") => {
                names.remove(0);
                FamlValue::None.to_expr()
            }
            Some(&"base") => {
                names.remove(0);
                self.base().base_expr.upgrade()?
            }
            Some(&"super") => {
                names.remove(0);
                let tmp = self.base().super_expr.upgrade()?;
                tmp.base().super_expr.upgrade()?
            }
            _ => self.base().super_expr.upgrade()?,
        };
        for name in names {
            expr = expr
                .get(name)
                .ok_or_else(|| anyhow!("node has no field1[{name}]"))?
                .clone();
        }
        Ok(expr)
    }

    pub fn evaluate(&self) -> anyhow::Result<FamlValue> {
        match &self.base().expr {
            FamlExprImpl::None => Ok(FamlValue::None),
            FamlExprImpl::Value(val) => Ok(val.clone()),
            FamlExprImpl::Array(arr) => {
                let mut ret = Vec::new();
                for item in arr.iter() {
                    ret.push(item.evaluate()?);
                }
                Ok(FamlValue::Array(ret))
            }
            FamlExprImpl::Map(map) => {
                let mut ret = HashMap::new();
                for (key, item) in map.iter() {
                    ret.insert(key.clone(), item.evaluate()?);
                }
                Ok(FamlValue::Map(ret))
            }
            FamlExprImpl::TempName(names) => {
                let expr = self.get_temp_name_expr(names)?;
                expr.evaluate()
            }
            FamlExprImpl::Op1Prefix((op, a)) => {
                let a = a.evaluate()?;
                Op1Evaluator::eval_prefix(&op, a)
            }
            FamlExprImpl::Op1Suffix((a, op)) => {
                let a = a.evaluate()?;
                Op1Evaluator::eval_suffix(a, &op)
            }
            FamlExprImpl::Op2((a, op, b)) => {
                let a = a.evaluate()?;
                let b = b.evaluate()?;
                Op2Evaluator::eval(a, &op, b)
            }
            FamlExprImpl::Op3((a, b, c)) => match a.evaluate()?.as_bool() {
                Some(true) => b.evaluate(),
                Some(false) => c.evaluate(),
                None => Err(anyhow!("bool expected"))?,
            },
            FamlExprImpl::FormatString((strs, exprs)) => {
                let mut str_exprs = vec![];
                for expr in exprs {
                    str_exprs.push(expr.evaluate()?.as_str());
                }
                str_exprs.push("".to_string());
                let mut ret = "".to_string();
                for (a, b) in strs.iter().zip(str_exprs.iter()) {
                    ret.push_str(a);
                    ret.push_str(b);
                }
                Ok(FamlValue::String(ret))
            }
            FamlExprImpl::AccessVar((expr, arg)) => {
                let expr = expr.evaluate()?;
                let arg = arg.evaluate()?;
                let val = match arg {
                    FamlValue::Int64(i) => expr.get_at(i as usize),
                    FamlValue::String(s) => expr.get(&s),
                    _ => Err(anyhow!("unexpected index type"))?,
                };
                Ok(val.cloned().unwrap_or(FamlValue::None))
            }
            FamlExprImpl::InvokeFunc((expr, args)) => {
                if let FamlExprImpl::TempName(names) = &expr.base().expr {
                    let mut arg_vals = vec![];
                    for arg in args {
                        arg_vals.push(arg.evaluate()?);
                    }
                    //
                    if names.len() == 2 && names[0] == "native" {
                        let func = Native::get_func(&names[1])
                            .ok_or_else(|| anyhow!("native func not found"))?;
                        Ok(func.call(arg_vals))
                    } else {
                        let mut names = names.clone();
                        let func = names.pop().ok_or_else(|| anyhow!("func name expected"))?;
                        let mut obj_val = {
                            let mut obj_expr = FamlExprImpl::TempName(names).to_expr();
                            obj_expr.base_mut().base_expr = expr.base().base_expr.clone();
                            obj_expr.base_mut().super_expr = expr.base().super_expr.clone();
                            obj_expr.evaluate()?
                        };
                        obj_val.invoke(&func, &arg_vals)
                    }
                } else {
                    Err(anyhow!("unsupported invoke type"))?
                }
            }
            FamlExprImpl::IfAnno(if_anno) => {
                for (cond, value) in &if_anno.ifcond_values {
                    if cond.evaluate()?.as_bool() == Some(true) {
                        return value.evaluate();
                    }
                }
                return if_anno.default_value.evaluate();
            }
        }
    }

    fn trace_internal(
        &self,
        atom_str: bool,
        maps: &mut HashMap<String, (FamlValue, String)>,
    ) -> anyhow::Result<(FamlValue, String)> {
        Ok(match &self.base().expr {
            FamlExprImpl::None => (FamlValue::None, "null".to_string()),
            FamlExprImpl::Value(val) => (val.clone(), val.as_str()),
            FamlExprImpl::Array(exprs) => {
                let mut vals = vec![];
                let mut vstrs = vec![];
                for expr in exprs {
                    let (val, vstr) = expr.trace_internal(false, maps)?;
                    vals.push(val);
                    vstrs.push(vstr);
                }
                (FamlValue::Array(vals), format!("[{}]", vstrs.join(", ")))
            }
            FamlExprImpl::Map(map) => {
                let mut vals = HashMap::new();
                let mut vstrs = HashMap::new();
                for (key, expr) in map {
                    let (val, vstr) = expr.trace_internal(false, maps)?;
                    vals.insert(key.clone(), val);
                    vstrs.insert(key.clone(), vstr);
                }
                let vstrs: Vec<_> = vstrs
                    .into_iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect();
                (FamlValue::Map(vals), format!("{{{}}}", vstrs.join(", ")))
            }
            FamlExprImpl::TempName(items) => {
                let expr = self.get_temp_name_expr(items)?;
                let name = items.join(".");
                let (val, vstr) = expr.trace_internal(false, maps)?;
                maps.insert(name.clone(), (val.clone(), vstr));
                (val, name)
            }
            FamlExprImpl::Op1Prefix((op, expr)) => {
                let (val, vstr) = expr.trace_internal(true, maps)?;
                let val = Op1Evaluator::eval_prefix(&op, val)?;
                let mut vstr = format!("{op}{vstr}");
                vstr = if atom_str { format!("({vstr})") } else { vstr };
                (val, vstr)
            }
            FamlExprImpl::Op1Suffix((expr, op)) => {
                let (val, vstr) = expr.trace_internal(true, maps)?;
                let val = Op1Evaluator::eval_suffix(val, &op)?;
                let mut vstr = format!("{vstr} {op}");
                vstr = if atom_str { format!("({vstr})") } else { vstr };
                (val, vstr)
            }
            FamlExprImpl::Op2((a, op, b)) => {
                let (val_a, vstr_a) = a.trace_internal(true, maps)?;
                let (val_b, vstr_b) = b.trace_internal(true, maps)?;
                let val = Op2Evaluator::eval(val_a, &op, val_b)?;
                let mut vstr = format!("{vstr_a} {op} {vstr_b}");
                vstr = if atom_str { format!("({vstr})") } else { vstr };
                (val, vstr)
            }
            FamlExprImpl::Op3((a, b, c)) => {
                let (val_a, vstr_a) = a.trace_internal(true, maps)?;
                let (val_b, vstr_b) = b.trace_internal(true, maps)?;
                let (val_c, vstr_c) = c.trace_internal(true, maps)?;
                let cond = val_a.as_bool().ok_or_else(|| anyhow!("bool expected"))?;
                let val = if cond { val_b } else { val_c };
                let mut vstr = format!("{vstr_a} ? {vstr_b} : {vstr_c}");
                vstr = if atom_str { format!("({vstr})") } else { vstr };
                (val, vstr)
            }
            FamlExprImpl::FormatString((strs, exprs)) => {
                if strs.len() == 1 && exprs.is_empty() {
                    let vstr = format!("$\"{}\"", strs[0].escape(true));
                    (strs[0].clone().into(), vstr)
                } else {
                    let mut rval = "".to_string();
                    let mut rvstr = "".to_string();
                    for (idx, expr) in exprs.iter().enumerate() {
                        let (val, vstr) = expr.trace_internal(false, maps)?;
                        rval.push_str(&strs[idx]);
                        rval.push_str(&val.as_str());
                        let vprefix = if idx == 0 { "$\"" } else { "}" };
                        let cur_str = strs[idx].escape(true);
                        rvstr.push_str(&format!("{vprefix}{cur_str}{{{vstr}"));
                    }
                    rval.push_str(&strs[strs.len() - 1]);
                    rvstr.push_str(&format!("}}{}\"", strs[strs.len() - 1]));
                    (rval.into(), rvstr)
                }
            }
            FamlExprImpl::AccessVar((a, b)) => {
                let val = self.evaluate()?;
                let (_, a) = a.trace_internal(true, maps)?;
                let (_, b) = b.trace_internal(true, maps)?;
                (val, format!("{a}.{b}"))
            }
            FamlExprImpl::InvokeFunc((expr, args)) => {
                if let FamlExprImpl::TempName(names) = &expr.base().expr {
                    let expr_str = names.join(".");
                    let mut arg_strs = vec![];
                    for arg in args {
                        let (_, arg_str) = arg.trace_internal(true, maps)?;
                        arg_strs.push(arg_str);
                    }
                    let vstr = format!("{}({})", expr_str, arg_strs.join(", "));
                    (self.evaluate()?, vstr)
                } else {
                    Err(anyhow!("unsupported invoke type"))?
                }
            }
            FamlExprImpl::IfAnno(if_anno) => {
                let mut val = FamlExpr::new();
                for (cond, value) in &if_anno.ifcond_values {
                    if cond.evaluate()?.as_bool() == Some(true) {
                        val = value.clone();
                        continue;
                    }
                }
                let (_, val_str) = val.trace_internal(atom_str, maps)?;
                (self.evaluate()?, val_str)
            }
        })
    }

    pub fn trace(&self, name: &str) -> anyhow::Result<String> {
        let format_vstr = |name: &str, val: FamlValue, vstr: &str| {
            let val_str = val.as_str();
            match name == vstr || vstr == val_str {
                true => format!("{name} = {val_str}"),
                false => format!("{name} = {vstr} // ={val_str}"),
            }
        };

        let mut maps = HashMap::new();
        let (val, vstr) = self.trace_internal(false, &mut maps)?;
        let mut maps = maps
            .into_iter()
            .map(|(name, (val, vstr))| format_vstr(&name, val, &vstr))
            .collect::<Vec<_>>();
        maps.sort();
        maps.push(format_vstr(name, val, &vstr));
        Ok(maps.join("\n"))
    }

    pub fn set_null(&mut self) {
        self.base_mut().expr.set_null();
    }

    pub fn set_bool(&mut self, val: bool) {
        self.base_mut().expr.set_bool(val);
    }

    pub fn set_int(&mut self, val: i64) {
        self.base_mut().expr.set_int(val);
    }

    pub fn set_float(&mut self, val: f64) {
        self.base_mut().expr.set_float(val);
    }

    pub fn set_string(&mut self, val: impl Into<String>) {
        self.base_mut().expr.set_string(val);
    }

    pub fn set_value(&mut self, val: FamlValue) {
        self.base_mut().expr = FamlExprImpl::Value(val);
    }

    pub fn deserialize<T: for<'a> Deserialize<'a>>(&self) -> anyhow::Result<T> {
        Ok(serde_json::from_value(self.evaluate()?.to_json())?)
    }
}

impl FamlExpr {
    fn init_weak_expr(&mut self, base_expr: WeakFamlExpr, super_expr: WeakFamlExpr) {
        let self_expr = self.to_weak();
        self.base_mut()
            .init_weak_expr(base_expr, super_expr, self_expr);
    }
}

impl FamlExprBase {
    fn init_weak_expr(
        &mut self,
        base_expr: WeakFamlExpr,
        super_expr: WeakFamlExpr,
        self_expr: WeakFamlExpr,
    ) {
        self.base_expr = base_expr.clone();
        self.super_expr = super_expr.clone();
        match &mut self.expr {
            FamlExprImpl::None => (),
            FamlExprImpl::Value(_) => (),
            FamlExprImpl::Array(arr) => {
                for item in arr {
                    item.init_weak_expr(base_expr.clone(), self_expr.clone());
                }
            }
            FamlExprImpl::Map(map) => {
                for (_, item) in map {
                    item.init_weak_expr(base_expr.clone(), self_expr.clone());
                }
            }
            FamlExprImpl::TempName(_) => (),
            FamlExprImpl::Op1Prefix((_, expr)) => {
                expr.init_weak_expr(base_expr.clone(), super_expr.clone());
            }
            FamlExprImpl::Op1Suffix((expr, _)) => {
                expr.init_weak_expr(base_expr.clone(), super_expr.clone());
            }
            FamlExprImpl::Op2((expr1, _, expr2)) => {
                expr1.init_weak_expr(base_expr.clone(), super_expr.clone());
                expr2.init_weak_expr(base_expr.clone(), super_expr.clone());
            }
            FamlExprImpl::Op3((expr1, expr2, expr3)) => {
                expr1.init_weak_expr(base_expr.clone(), super_expr.clone());
                expr2.init_weak_expr(base_expr.clone(), super_expr.clone());
                expr3.init_weak_expr(base_expr.clone(), super_expr.clone());
            }
            FamlExprImpl::FormatString((_, exprs)) => {
                for expr in exprs {
                    expr.init_weak_expr(base_expr.clone(), super_expr.clone());
                }
            }
            FamlExprImpl::AccessVar((expr, _)) => {
                expr.init_weak_expr(base_expr.clone(), super_expr.clone());
            }
            FamlExprImpl::InvokeFunc((expr, args)) => {
                expr.init_weak_expr(base_expr.clone(), super_expr.clone());
                for arg in args {
                    arg.init_weak_expr(base_expr.clone(), super_expr.clone());
                }
            }
            FamlExprImpl::IfAnno(if_anno) => {
                if_anno.init_weak_expr(base_expr.clone(), super_expr.clone());
            }
        }
    }
}
