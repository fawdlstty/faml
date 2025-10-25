use crate::FamlExprBase;

#[test]
fn test1() -> anyhow::Result<()> {
    let faml_str = r#"
[hello]
value = 12
name = $"hello world {value + 12}"
"#;
    let mut root = FamlExprBase::from_str(faml_str)?;
    root["hello"]["value"].set_int(30);
    let root = root.evalute()?;
    assert_eq!(root["hello"]["name"].as_str(), "hello world 42");
    Ok(())
}
