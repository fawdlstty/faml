# 表达式

表达式是FAML中用于计算的语法，支持基本的算术运算、逻辑运算和条件运算。

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

然后是数组或哈希表的成员访问。也很简单，类似编程语言：

```faml
[group1]
array_field = [1, 2, 3, 4, 5]        // 数组类型
map_field = { foo: "bar", baz: 123 } // 哈希表类型
array2 = array_field[2]              // 3
mapfoo = map_field.foo               // "bar"
```

再然后是条件运算符，只有在条件为真时才会生效：

```faml
[group]
@if 1 == 1
var_a = 123    // 123

@if 1 == 2
var_b = 456    // null
```
