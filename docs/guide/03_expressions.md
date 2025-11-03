# 表达式

表达式是FAML中用于计算的语法，支持基本的算术运算、逻辑运算和条件运算。

首先是对其他配置项的引用。直接写配置名称即可实现引用：

```rust
fn main() -> anyhow::Result<()> {
    let faml_str = r#"
[group1]
int1_val = 20

[group2]
int2_val = 22
int3_val = int2_val                                     // 设置值为当前层级的参数值
int4_val = base.group1.int1_val + super.group1.int1_val // 从顶层开始索引以及从上一级开始索引
format_str_field = $"hello, {int4_val}"                 // 格式化字符串
"#;
    let root = faml::FamlExpr::from_str(faml_str)?;
    let value = root.evaluate()?;
    let json = serde_json::to_string_pretty(&value)?;
    println!("{json}");
    Ok(())
}
```

以上代码的json内容为（注意，输出可能是乱序）：

```json
{
  "group1": {
    "int1_val": 20
  },
  "group2": {
    "int2_val": 22,
    "int3_val": 22,
    "int4_val": 40,
    "format_str_field": "hello, 40"
  }
}
```
