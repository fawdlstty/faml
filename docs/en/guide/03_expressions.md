# Expressions

Expressions are syntax used for calculations in FAML, supporting basic arithmetic operations, logical operations, comparison operations, and conditional operations.

## Basic References

First is the reference to other configuration items. Simply write the configuration name to achieve reference:

```faml
[group1]
int1_val = 20

[group2]
int2_val = 22
int3_val = int2_val                                     // 22
int4_val = base.group1.int1_val + super.group1.int1_val // 40
format_str_field = $"hello, {int4_val}"                 // "hello, 40"
```

## Array and Hash Map Member Access

Accessing members of arrays or hash maps is also simple, similar to programming languages:

```faml
[group1]
array_field = [1, 2, 3, 4, 5]        // Array type
map_field = { foo: "bar", baz: 123 } // Hash map type
array2 = array_field[2]              // 3
mapfoo = map_field.foo               // "bar"
```

## Arithmetic Operators

FAML supports common arithmetic operators:

```faml
[arithmetic]
add = 10 + 5      // 15
sub = 10 - 5      // 5
mul = 10 * 5      // 50
div = 10 / 5      // 2
mod = 10 % 3      // 1
pow = 2 ** 3      // 8 (power operation)
```

## Comparison Operators

Support common comparison operators:

```faml
[comparison]
eq = 10 == 5      // false
ne = 10 != 5      // true
lt = 10 < 5       // false
le = 10 <= 10     // true
gt = 10 > 5       // true
ge = 10 >= 15     // false
```

## Logical Operators

Support logical operators for boolean operations:

```faml
[logic]
and = true && false   // false
or = true || false    // true
not = !true           // false
```

## Conditional Operator (Ternary Operator)

FAML supports ternary conditional operators:

```faml
[conditional]
value = 10
result = value > 5 ? "large" : "small"  // "large"
```

## Formatted Strings

Use the `$""` syntax to create formatted strings, where expressions can be embedded:

```faml
[format]
name = "Alice"
age = 30
greeting = $"Hello, {name}! You are {age} years old."  // "Hello, Alice! You are 30 years old."
```

## Function Calls

You can call built-in methods of values:

```faml
[functions]
negative_value = -12
positive_value = negative_value.abs()  // 12
```

## Operator Precedence

Operators in FAML are arranged in the following precedence from high to low:

1. `**` (power operation)
2. `*`, `/`, `%` (multiplication, division, modulo)
3. `+`, `-` (addition, subtraction)
4. `<<`, `>>` (bit shift)
5. `&` (bitwise AND)
6. `^` (bitwise XOR)
7. `|` (bitwise OR)
8. `<`, `<=`, `>`, `>=` (comparison)
9. `==`, `!=` (equality)
10. `&&` (logical AND)
11. `||` (logical OR)