# 结构和类型

注释使用类似编译语言的注释：
```faml
// 单行注释
/* 多行注释 */
```

基本结构的写法近似于yaml，但注意标识符类似编程语言，只能为下划线、字母或数字的组合，且只允许下划线或字母开头。示例代码：

```rust
fn main() -> anyhow::Result<()> {
    let faml_str = r#"
[group]
field = 123

[[groups]]
field1 = 111

[[groups]]
field2 = 111
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
  "group": { "field": 123 },
  "groups": [
    { "field1": 111 },
    { "field2": 111 }
  ]
}
```

数据类型相对于yaml有极大扩展：

```rust
fn main() -> anyhow::Result<()> {
    let faml_str = r#"
[group]
bool_field = true
int_field = 123
float_field = 123.456
quantified_float_field = 123.456 KB            // 量化数字类型
str_field = "hello world"
array_field = [1, 2, 3, 4, 5]                  // 数组类型
map_field = { foo: "bar", baz: 123 }
duration_field = 123.456 seconds               // 持续时间类型
distance_field = 123.456 meters                // 距离类型
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
  "group": {
    "bool_field": true,
    "int_field": 123,
    "float_field": 123.456,
    "quantified_float_field": 126418.944,
    "str_field": "hello world",
    "array_field": [ 1, 2, 3, 4, 5 ],
    "map_field": { "baz": 123, "foo": "bar" },
    "duration_field": {
      "famltype": "duration",
      "unit": "seconds",
      "value": 123.456
    },
    "distance_field": {
      "famltype": "distance",
      "unit": "meters",
      "value": 123.456
    }
  }
}
```
