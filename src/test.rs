use crate::FamlExpr;

#[test]
fn test1() {
    let faml_str = r#"
[hello]
value = 12
name = $"hello world {value + 12}"
"#;
    let mut root = match FamlExpr::from_str(faml_str) {
        Ok(root) => root,
        Err(err) => panic!("Error: {}", err),
    };
    root["hello"]["value"].set_int(30);
    let root = match root.evalute() {
        Ok(root) => root,
        Err(err) => panic!("Error: {}", err),
    };
    assert_eq!(root["hello"]["name"].as_str(), "hello world 42");
}
