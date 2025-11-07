# 表达式

表达式是FAML中用于计算的语法，支持基本的算术运算、逻辑运算、比较运算和条件运算。

## 基本引用

首先是对其他配置项的引用。直接写配置名称即可实现引用：

```faml
[group1]
int1_val = 20

[group2]
int2_val = 22
int3_val = int2_val                                     // 22
int4_val = base.group1.int1_val + super.group1.int1_val // 40
format_str_field = $"hello, {int4_val}"                 // "hello, 40"
```

## 数组和哈希表成员访问

数组或哈希表的成员访问也很简单，类似编程语言：

```faml
[group1]
array_field = [1, 2, 3, 4, 5]        // 数组类型
map_field = { foo: "bar", baz: 123 } // 哈希表类型
array2 = array_field[2]              // 3
mapfoo = map_field.foo               // "bar"
```

## 算术运算符

FAML支持常见的算术运算符：

```faml
[arithmetic]
add = 10 + 5      // 15
sub = 10 - 5      // 5
mul = 10 * 5      // 50
div = 10 / 5      // 2
mod = 10 % 3      // 1
pow = 2 ** 3      // 8 (幂运算)
```

## 比较运算符

支持常见的比较运算符：

```faml
[comparison]
eq = 10 == 5      // false
ne = 10 != 5      // true
lt = 10 < 5       // false
le = 10 <= 10     // true
gt = 10 > 5       // true
ge = 10 >= 15     // false
```

## 逻辑运算符

支持逻辑运算符用于布尔值操作：

```faml
[logic]
and = true && false   // false
or = true || false    // true
not = !true           // false
```

## 条件运算符（三元运算符）

FAML支持三元条件运算符：

```faml
[conditional]
value = 10
result = value > 5 ? "large" : "small"  // "large"
```

## 格式化字符串

使用`$""`语法创建格式化字符串，可以在其中嵌入表达式：

```faml
[format]
name = "Alice"
age = 30
greeting = $"Hello, {name}! You are {age} years old."  // "Hello, Alice! You are 30 years old."
```

## 函数调用

可以调用值的内置方法：

```faml
[functions]
negative_value = -12
positive_value = negative_value.abs()  // 12
```

## 运算符优先级

FAML中的运算符按以下优先级从高到低排列：

1. `**` (幂运算)
2. `*`, `/`, `%` (乘法、除法、取模)
3. `+`, `-` (加法、减法)
4. `<<`, `>>` (位移)
5. `&` (按位与)
6. `^` (按位异或)
7. `|` (按位或)
8. `<`, `<=`, `>`, `>=` (比较)
9. `==`, `!=` (相等性)
10. `&&` (逻辑与)
11. `||` (逻辑或)