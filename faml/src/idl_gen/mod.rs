use crate::{FamlExpr, FamlExprImpl, string_utils::IntoBaseExt};

pub struct IDLGenerator {}

impl IDLGenerator {
    pub fn gen_rust(expr: &FamlExpr) -> anyhow::Result<String> {
        let mut s = "".to_string();
        if let FamlExprImpl::Map(map) = &expr.base().expr {
            for (group_name, group) in map {
                s.push_str(&format!("pub struct {} {{\n", group_name.to_pascal_case()));
                s.push_str("}\n\n");
            }
        }
        todo!()
    }
}
