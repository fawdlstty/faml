use crate::{FamlExpr, FamlValue};

#[test]
fn test1() -> anyhow::Result<()> {
    let faml_str = r#"
[hello]
value = 12
name = $"hello world {value + 12}"
"#;
    let mut root = FamlExpr::from_str(faml_str)?;
    root["hello"]["value"].set_int(30);
    let root = root.evaluate()?;
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
    let value3 = root["hello"]["test"]["value3"].evaluate()?.as_str();
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
    let value3 = root["hello"]["value2"].evaluate()?.as_str();
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
    let name = root["hello"]["name"].evaluate()?.as_str();
    assert_eq!(name, "hello world 14");
    Ok(())
}

// 测试注释功能
#[test]
fn test_comments() -> anyhow::Result<()> {
    let faml_str = r#"
// 单行注释
/* 多行注释 */
[group]
field = 123
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let value = root["group"]["field"].evaluate()?.as_int().unwrap();
    assert_eq!(value, 123);
    Ok(())
}

// 测试基本结构：单个对象配置
#[test]
fn test_single_object() -> anyhow::Result<()> {
    let faml_str = r#"
[group]
field = 123
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let value = root["group"]["field"].evaluate()?.as_int().unwrap();
    assert_eq!(value, 123);
    Ok(())
}

// 测试基本结构：对象数组配置
#[test]
fn test_object_array() -> anyhow::Result<()> {
    let faml_str = r#"
[[groups]]
field1 = 111

[[groups]]
field2 = 111
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let groups = root["groups"].evaluate()?;
    // 使用as_array获取数组并检查长度
    let array = groups.as_array().unwrap();
    assert_eq!(array.len(), 2);
    assert_eq!(array[0]["field1"].as_int().unwrap(), 111);
    assert_eq!(array[1]["field2"].as_int().unwrap(), 111);
    Ok(())
}

// 测试条件判断
#[test]
fn test_conditional_fields() -> anyhow::Result<()> {
    let faml_str = r#"
[group]
value = 12

@if value == 12
conditional_field = "this field is active"

@if value != 12
inactive_field = "this field is not active"
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let evaluated = root.evaluate()?;
    assert_eq!(
        evaluated["group"]["conditional_field"].as_str(),
        "this field is active"
    );
    // 检查inactive_field不存在
    assert_eq!(evaluated["group"]["inactive_field"], FamlValue::None);
    Ok(())
}

// 测试引用其他配置项
#[test]
fn test_field_references() -> anyhow::Result<()> {
    let faml_str = r#"
[group]
base_value = 10
derived_value = base_value + 5
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let derived_value = root["group"]["derived_value"].evaluate()?.as_int().unwrap();
    assert_eq!(derived_value, 15);
    Ok(())
}

// 测试基本数据类型
#[test]
fn test_basic_types() -> anyhow::Result<()> {
    let faml_str = r#"
[group]
bool_field = true
int_field = 123
float_field = 123.456
str_field = "hello world"
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let evaluated = root.evaluate()?;
    assert_eq!(evaluated["group"]["bool_field"].as_bool().unwrap(), true);
    assert_eq!(evaluated["group"]["int_field"].as_int().unwrap(), 123);
    assert_eq!(
        evaluated["group"]["float_field"].as_float().unwrap(),
        123.456
    );
    assert_eq!(evaluated["group"]["str_field"].as_str(), "hello world");
    Ok(())
}

// 测试复杂数据类型
#[test]
fn test_complex_types() -> anyhow::Result<()> {
    let faml_str = r#"
[group]
// 数组类型
array_field = [1, 2, 3, 4, 5]

// 映射类型
map_field = { foo: "bar", baz: 123 }
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let evaluated = root.evaluate()?;
    let array = evaluated["group"]["array_field"].as_array().unwrap();
    assert_eq!(array.len(), 5);
    assert_eq!(array[0].as_int().unwrap(), 1);
    assert_eq!(array[4].as_int().unwrap(), 5);

    let map = evaluated["group"]["map_field"].as_map().unwrap();
    assert_eq!(map.get("foo").unwrap().as_str(), "bar");
    assert_eq!(map.get("baz").unwrap().as_int().unwrap(), 123);
    Ok(())
}

// 测试特殊类型
#[test]
fn test_special_types() -> anyhow::Result<()> {
    let faml_str = r#"
[group]
// 量化数字类型
quantified_float_field = 123.456 KB

// 持续时间类型
duration_field = 123.456 seconds

// 距离类型
distance_field = 123.456 meters
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let evaluated = root.evaluate()?;
    // 量化数字类型，123.456 KB = 123.456 * 1024 = 126418.944
    assert_eq!(
        evaluated["group"]["quantified_float_field"]
            .as_float()
            .unwrap(),
        126418.944
    );
    Ok(())
}

// 测试表达式的基本引用
#[test]
fn test_expression_references() -> anyhow::Result<()> {
    let faml_str = r#"
[group1]
int1_val = 20

[group2]
int2_val = 22
int3_val = int2_val
int4_val = base.group1.int1_val + super.group1.int1_val
format_str_field = $"hello, {int4_val}"
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let evaluated = root.evaluate()?;
    assert_eq!(evaluated["group2"]["int3_val"].as_int().unwrap(), 22);
    assert_eq!(evaluated["group2"]["int4_val"].as_int().unwrap(), 40);
    assert_eq!(
        evaluated["group2"]["format_str_field"].as_str(),
        "hello, 40"
    );
    Ok(())
}

// 测试数组和哈希表成员访问
#[test]
fn test_array_map_access() -> anyhow::Result<()> {
    let faml_str = r#"
[group1]
array_field = [1, 2, 3, 4, 5]
map_field = { foo: "bar", baz: 123 }
array2 = array_field[2]
mapfoo = map_field.foo
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let evaluated = root.evaluate()?;
    assert_eq!(evaluated["group1"]["array2"].as_int().unwrap(), 3);
    assert_eq!(evaluated["group1"]["mapfoo"].as_str(), "bar");
    Ok(())
}

// 测试算术运算符
#[test]
fn test_arithmetic_operators() -> anyhow::Result<()> {
    let faml_str = r#"
[arithmetic]
add = 10 + 5
sub = 10 - 5
mul = 10 * 5
div = 10 / 5
mod = 10 % 3
pow = 2 ** 3
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let evaluated = root.evaluate()?;
    assert_eq!(evaluated["arithmetic"]["add"].as_int().unwrap(), 15);
    assert_eq!(evaluated["arithmetic"]["sub"].as_int().unwrap(), 5);
    assert_eq!(evaluated["arithmetic"]["mul"].as_int().unwrap(), 50);
    assert_eq!(evaluated["arithmetic"]["div"].as_int().unwrap(), 2);
    assert_eq!(evaluated["arithmetic"]["mod"].as_int().unwrap(), 1);
    assert_eq!(evaluated["arithmetic"]["pow"].as_int().unwrap(), 8);
    Ok(())
}

// 测试比较运算符
#[test]
fn test_comparison_operators() -> anyhow::Result<()> {
    let faml_str = r#"
[comparison]
eq = 10 == 5
ne = 10 != 5
lt = 10 < 5
le = 10 <= 10
gt = 10 > 5
ge = 10 >= 15
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let evaluated = root.evaluate()?;
    assert_eq!(evaluated["comparison"]["eq"].as_bool().unwrap(), false);
    assert_eq!(evaluated["comparison"]["ne"].as_bool().unwrap(), true);
    assert_eq!(evaluated["comparison"]["lt"].as_bool().unwrap(), false);
    assert_eq!(evaluated["comparison"]["le"].as_bool().unwrap(), true);
    assert_eq!(evaluated["comparison"]["gt"].as_bool().unwrap(), true);
    assert_eq!(evaluated["comparison"]["ge"].as_bool().unwrap(), false);
    Ok(())
}

// 测试逻辑运算符
#[test]
fn test_logic_operators() -> anyhow::Result<()> {
    let faml_str = r#"
[logic]
and = true && false
or = true || false
not = !true
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let evaluated = root.evaluate()?;
    assert_eq!(evaluated["logic"]["and"].as_bool().unwrap(), false);
    assert_eq!(evaluated["logic"]["or"].as_bool().unwrap(), true);
    assert_eq!(evaluated["logic"]["not"].as_bool().unwrap(), false);
    Ok(())
}

// 测试条件运算符（三元运算符）
#[test]
fn test_conditional_operator() -> anyhow::Result<()> {
    let faml_str = r#"
[conditional]
value = 10
result = value > 5 ? "large" : "small"
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let evaluated = root.evaluate()?;
    assert_eq!(evaluated["conditional"]["result"].as_str(), "large");
    Ok(())
}

// 测试格式化字符串
#[test]
fn test_format_strings() -> anyhow::Result<()> {
    let faml_str = r#"
[format]
name = "Alice"
age = 30
greeting = $"Hello, {name}! You are {age} years old."
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let evaluated = root.evaluate()?;
    assert_eq!(
        evaluated["format"]["greeting"].as_str(),
        "Hello, Alice! You are 30 years old."
    );
    Ok(())
}

// 测试函数调用
#[test]
fn test_function_calls() -> anyhow::Result<()> {
    let faml_str = r#"
[functions]
negative_value = -12
positive_value = negative_value.abs()
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let evaluated = root.evaluate()?;
    assert_eq!(
        evaluated["functions"]["positive_value"].as_int().unwrap(),
        12
    );
    Ok(())
}

// 测试各种值的方法
#[test]
fn test_value_methods() -> anyhow::Result<()> {
    // 测试空值方法
    let faml_str1 = r#"
[group]
eval = null
eval_str = eval.to_str()
"#;
    let root1 = FamlExpr::from_str(faml_str1)?;
    let evaluated1 = root1.evaluate()?;
    assert_eq!(evaluated1["group"]["eval_str"].as_str(), "null");

    // 测试布尔值方法
    let faml_str2 = r#"
[group]
bval = true
bval_str = bval.to_str()
"#;
    let root2 = FamlExpr::from_str(faml_str2)?;
    let evaluated2 = root2.evaluate()?;
    assert_eq!(evaluated2["group"]["bval_str"].as_str(), "true");

    // 测试浮点数方法
    let faml_str3 = r#"
[group]
num = 12.3
num_ceil = num.ceil()
num_abs = num.abs()
"#;
    let root3 = FamlExpr::from_str(faml_str3)?;
    let evaluated3 = root3.evaluate()?;
    assert_eq!(evaluated3["group"]["num_ceil"].as_float().unwrap(), 13.0);
    assert_eq!(evaluated3["group"]["num_abs"].as_float().unwrap(), 12.3);

    // 测试字符串方法
    let faml_str4 = r#"
[group]
sval = "a b  c"
sval_len = sval.len()
sval_sp1 = sval.split(" ")
"#;
    let root4 = FamlExpr::from_str(faml_str4)?;
    let evaluated4 = root4.evaluate()?;
    assert_eq!(evaluated4["group"]["sval_len"].as_int().unwrap(), 6);
    let array = evaluated4["group"]["sval_sp1"].as_array().unwrap();
    assert_eq!(array.len(), 4);

    // 测试数组方法
    let faml_str5 = r#"
[group]
arr = [1, 2, 3, 4, 5]
arr_len = arr.len()
arr_rev = arr.reverse()
"#;
    let root5 = FamlExpr::from_str(faml_str5)?;
    let evaluated5 = root5.evaluate()?;
    assert_eq!(evaluated5["group"]["arr_len"].as_int().unwrap(), 5);
    let reversed_array = evaluated5["group"]["arr_rev"].as_array().unwrap();
    assert_eq!(reversed_array[0].as_int().unwrap(), 5);

    // 测试映射方法
    let faml_str6 = r#"
[group]
map = { a: 1, b: 2 }
map_len = map.len()
"#;
    let root6 = FamlExpr::from_str(faml_str6)?;
    let evaluated6 = root6.evaluate()?;
    assert_eq!(evaluated6["group"]["map_len"].as_int().unwrap(), 2);

    Ok(())
}

#[test]
fn test_simple_format_string() -> anyhow::Result<()> {
    let faml_str = r#"
[hello]
value = $"hello"
"#;
    let root = FamlExpr::from_str(faml_str)?;
    let evaluated = root.evaluate()?;
    assert_eq!(evaluated["hello"]["value"].as_str(), "hello");
    Ok(())
}
