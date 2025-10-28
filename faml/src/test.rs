use crate::FamlExpr;

#[test]
fn test1() -> anyhow::Result<()> {
    let faml_str = r#"
[hello]
value = 12
name = $"hello world {value + 12}"
"#;
    let mut root = FamlExpr::from_str(faml_str)?;
    root["hello"]["value"].set_int(30);
    let root = root.evalute()?;
    assert_eq!(root["hello"]["name"].as_str(), "hello world 42");
    Ok(())
}

#[derive(serde::Deserialize)]
struct MyStructHello {
    pub age: i32,
    pub name: String,
}

#[derive(serde::Deserialize)]
struct MyStruct {
    pub hello: MyStructHello,
}

#[test]
fn test2() -> anyhow::Result<()> {
    let faml_str = r#"
[hello]
age = 12
name = "maria"
"#;
    let mut root = FamlExpr::from_str(faml_str)?;
    root["hello"]["age"].set_int(30);
    let data: MyStruct = root.deserialize()?;
    assert_eq!(data.hello.age, 30);
    assert_eq!(data.hello.name, "maria");
    Ok(())
}

#[test]
fn test3() -> anyhow::Result<()> {
    let faml_str = r#"
[hello]
value1 = 12
value2 = 13

[hello.test]
value3 = $"value1[{super.value1}], value2[{base.hello.value2}]"
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let value3 = root["hello"]["test"]["value3"].evalute()?.as_str();
    assert_eq!(value3, "value1[12], value2[13]");
    Ok(())
}

#[test]
fn test4() -> anyhow::Result<()> {
    let faml_str = r#"
[hello]
value1 = -12
value2 = value1.abs()
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let value3 = root["hello"]["value2"].evalute()?.as_str();
    assert_eq!(value3, "12");
    Ok(())
}

#[test]
fn test5() -> anyhow::Result<()> {
    let faml_str = r#"
        [hello]
        value = 12
    
        @if value == 12
        name = "hello world"
    
        @if value == 14
        name = $"hello world {value}"
    
        "#;
    let mut root = FamlExpr::from_str(faml_str)?;
    root["hello"]["value"].set_int(14);
    let name = root["hello"]["name"].evalute()?.as_str();
    assert_eq!(name, "hello world 14");
    Ok(())
}
