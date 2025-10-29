# 入门示例

假设我们现在有一个faml配置语言，内容如下：

```faml
[hello]
value = 12
name = $"hello {value + 12}"
```

我们希望在rust里使用修改hello.value属性的值，然后获得hello.name属性的值。现创建Rust项目，并加入faml依赖：

```bash
cargo new hello_faml
cd hello_faml
cargo add faml
```

加入示例代码：

```rust
fn main() -> anyhow::Result<()> {
    let faml_str = r#"
[hello]
value = 12
name = $"hello {value + 12}"
"#;
    let mut root = faml::FamlExpr::from_str(faml_str)?;
    root["hello"]["value"].set_int(30);
    println!("{}", root["hello"]["name"].evaluate()?.as_str()); // hello 42
    Ok(())
}
```

执行后将打印hello.name属性值：`hello 42`
