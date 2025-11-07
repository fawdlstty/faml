# 结构和类型

## 注释

注释使用类似编译语言的注释：
```faml
// 单行注释
/* 多行注释 */
```

## 基本结构

基本结构的写法近似于yaml，但注意标识符类似编程语言，只能为下划线、字母或数字的组合，且只允许下划线或字母开头。

### 单个对象配置
使用方括号 `[]` 定义单个对象：

```faml
[group]
field = 123
```

对应的JSON结构：
```json
{
  "group": { "field": 123 }
}
```

### 对象数组配置
使用双方括号 `[[]]` 定义对象数组：

```faml
[[groups]]
field1 = 111

[[groups]]
field2 = 111
```

对应的JSON结构：
```json
{
  "groups": [
    { "field1": 111 },
    { "field2": 111 }
  ]
}
```

## 核心特性

### 条件判断

使用`@if`指令可以根据条件控制字段是否生效：

```faml
[group]
value = 12

@if value == 12
conditional_field = "this field is active"

@if value != 12
inactive_field = "this field is not active"
```

当 `value` 等于 12 时，生成的JSON结构：
```json
{
  "group": {
    "value": 12,
    "conditional_field": "this field is active"
  }
}
```

### 引用其他配置项

可以直接引用同一配置文件中的其他字段：

```faml
[group]
base_value = 10
derived_value = base_value + 5  // 15
```

对应的JSON结构：
```json
{
  "group": {
    "base_value": 10,
    "derived_value": 15
  }
}
```

## 数据类型

数据类型相对于yaml有极大扩展：

### 基本类型

```faml
[group]
bool_field = true
int_field = 123
float_field = 123.456
str_field = "hello world"
```

对应的JSON结构：
```json
{
  "group": {
    "bool_field": true,
    "int_field": 123,
    "float_field": 123.456,
    "str_field": "hello world"
  }
}
```

### 复杂类型

```faml
[group]
// 数组类型
array_field = [1, 2, 3, 4, 5]

// 映射类型
map_field = { foo: "bar", baz: 123 }
```

对应的JSON结构：
```json
{
  "group": {
    "array_field": [ 1, 2, 3, 4, 5 ],
    "map_field": { "baz": 123, "foo": "bar" }
  }
}
```

### 特殊类型

```faml
[group]
// 量化数字类型，实际值为数字类型
quantified_float_field = 123.456 KB

// 持续时间类型
duration_field = 123.456 seconds

// 距离类型
distance_field = 123.456 meters
```

对应的JSON结构：
```json
{
  "group": {
    "quantified_float_field": 126418.944,
    "duration_field": "2.0576 mins",
    "distance_field": "123.456 meters"
  }
}
```

### 量化数字类型单位

支持以下单位：
- `KB` (千字节)
- `MB` (兆字节)
- `GB` (吉字节)
- `TB` (太字节)

### 持续时间类型单位

支持以下时间单位：
- `nanoseconds` (纳秒)
- `microseconds` (微秒)
- `milliseconds` (毫秒)
- `seconds` (秒)
- `mins` (分钟)
- `hours` (小时)
- `days` (天)
- `weeks` (周)
- `months` (月)
- `years` (年)

### 距离类型单位

支持以下距离单位：
- `nanometers` (纳米)
- `micrometers` (微米)
- `millimeters` (毫米)
- `meters` (米)
- `kilometers` (千米)
- `megameters` (兆米)
