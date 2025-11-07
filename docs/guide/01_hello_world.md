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
cargo add anyhow
```

注意：由于示例代码使用了`anyhow`库进行错误处理，我们需要同时添加`anyhow`依赖。

在Cargo.toml文件中应该包含以下依赖：

```toml
[dependencies]
faml = "0.1"
anyhow = "1.0"
```

加入示例代码：

```rust
use anyhow::Result;

fn main() -> Result<()> {
    let faml_str = r#"
[hello]
value = 12
name = $"hello {value + 12}"
"#;
    let mut root = faml::FamlExpr::from_str(faml_str)?;
    root["hello"]["value"].set_int(30);
    let name = root["hello"]["name"].evaluate()?.as_str();
    println!("{}", name); // hello 42
    Ok(())
}
```

执行后将打印hello.name属性值：`hello 42`

## 代码说明

1. 首先我们导入了`anyhow::Result`类型用于错误处理
2. 使用`faml::FamlExpr::from_str()`解析faml字符串
3. 通过`root["hello"]["value"].set_int(30)`修改value的值为30
4. 通过`root["hello"]["name"].evaluate()?.as_str()`计算并获取name的值
5. 由于value被修改为30，表达式`value + 12`的结果是42，所以最终输出`hello 42`