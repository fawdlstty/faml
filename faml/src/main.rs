use faml::FamlExpr;
use serde::Deserialize;

// fn main() {
//     let faml_str = r#"
// [hello]
// value = 12

// @if value == 12
// name = $"hello world a~~~~{value}"

// @if value == 14
// name = $"hello world b~~~~{value}"

// "#;
//     let mut root = FamlExprImpl::from_str(faml_str).unwrap();
//     root["hello"]["value"].set_int(14);
//     let root = root.evalute().unwrap();
//     println!("{}", root["hello"]["name"].as_str());
// }

#[derive(Debug, Deserialize)]
pub struct MyStructHello {
    pub age: i32,
    pub desp: String,
}

#[derive(Debug, Deserialize)]
pub struct MyStruct {
    pub hello: MyStructHello,
}

fn main() {
    let faml_str = r#"
[hello]
value = 12
name = $"hello {value + 12}"
"#;
    let mut eroot = FamlExpr::from_str(faml_str).unwrap();
    eroot["hello"]["value"].set_int(30);
    let root = eroot.evalute().unwrap();
    println!("{}", root["hello"]["name"].as_str()); // hello 42
}

// cargo publish --allow-dirty --registry crates-io
