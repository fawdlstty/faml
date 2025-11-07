# Methods

Null value methods:

```faml
[group]
eval = null
eval_str = eval.to_str()  // "null"
```

Boolean value methods:

```faml
[group]
bval = true
bval_str = bval.to_str()  // "true"
```

Integer value methods, except for methods like round, floor, etc., are the same as floating-point value methods:

```faml
[group]
num = 12.3

// The following methods are exclusive to floating-point types
num_ceil = num.ceil()                          // 13.0
num_ceili = num.ceili()                        // 13
num_floor = num.floor()                        // 12.0
num_floori = num.floori()                      // 12
num_round = num.round()                        // 12.0
num_roundi = num.roundi()                      // 12
num_round_ties_even = num.round_ties_even()    // 12.0
num_round_ties_eveni = num.round_ties_eveni()  // 12
num_trunc = num.trunc()                        // 12.0
num_trunci = num.trunci()                      // 12

// The following methods are shared between floating-point types and integers
num_abs = num.abs()                            // 12.3
num_acos = num.acos()                          // null
num_acosh = num.acosh()                        // 3.2010898763691036
num_asin = num.asin()                          // null
num_asinh = num.asinh()                        // 3.20439481754902
num_atan = num.atan()                          // 1.4896739346939956
num_atanh = num.atanh()                        // null
num_cbrt = num.cbrt()                          // 2.308350239753609
num_cos = num.cos()                            // 0.9647326178866098
num_cosh = num.cosh()                          // 109847.99433834481
num_exp = num.exp()                            // 219695.9886721379
num_exp2 = num.exp2()                          // 5042.76751706078
num_fract = num.fract()                        // 0.3000000000000007
num_gamma = num.gamma()                        // 83385367.89997019
num_is_finite = num.is_finite()                // true
num_is_infinite = num.is_infinite()            // false
num_is_nan = num.is_nan()                      // false
num_is_negative = num.is_negative()            // false
num_is_positive = num.is_positive()            // true
num_ln = num.ln()                              // 2.509599262378372
num_log10 = num.log10()                        // 1.089905111439398
num_log2 = num.log2()                          // 3.6205864104518777
num_next_down = num.next_down()                // 12.299999999999999
num_next_up = num.next_up()                    // 12.300000000000002
num_signum = num.signum()                      // 1.0
num_sin = num.sin()                            // -0.26323179136580094
num_sinh = num.sinh()                          // 109847.99433379307
num_sqrt = num.sqrt()                          // 3.5071355833500366
num_tan = num.tan()                            // -0.2728546609551249
num_tanh = num.tanh()                          // 0.9999999999585633
num_to_quantified = num.to_quantified()        // "12.3 B"
num_to_degrees = num.to_degrees()              // 704.7380880109126
num_to_radians = num.to_radians()              // 0.21467549799530256
num_to_str = num.to_str()                      // "12.3"
```

String value functions:

```faml
[group]
sval = "a b  c"
sval_len = sval.len()                       // 6
sval_sp1 = sval.split(" ")                  // [ "a", "b", "", "c" ]
sval_sp2 = sval.split_once(" ")             // ["a", "b  c"]
sval_sp3 = sval.split_without_empty(" ")    // [ "a", "b", "c" ]

sval2 = "a+b-c/d*e"
sval2_sp1 = sval2.split("+", "-", "*", "/") // [ "a", "b", "c", "d", "e" ]

sval3 = "  a  "
sval3_trim = sval3.trim()                   // "a"
sval3_is_empty = sval3.is_empty()           // false

sval4 = "AaBbCc"
sval4_lower = sval4.to_lowercase()          // "aabbcc"
sval4_upper = sval4.to_uppercase()          // "AABBCC"
sval4_cts = sval4.contains("Bb")            // true
sval4_sw = sval4.starts_with("Aa")          // true
sval4_ew = sval4.ends_with("Cc")            // true
sval4_f = sval4.find("Bb")                  // 2
sval4_rf = sval4.rfind("Bb")                // 2
sval4_rp = sval4.repeat(3)                  // "AaBbCcAaBbCcAaBbCc"

sval5 = "hello"
sval5_replace = sval5.replace("l", "x")     // "hexxo"
sval5_replace_once = sval5.replace_once("l", "x") // "hexlo"
```

Array value functions:

```faml
[group]
arr = [1, 2, 3, 4, 5]
arr_len = arr.len()       // 5
arr_rev = arr.reverse()   // [ 5, 4, 3, 2, 1 ]
arr_str = arr.to_str()    // "[ 1, 2, 3, 4, 5 ]"
arr_join = arr.join(", ") // "1, 2, 3, 4, 5"
```

Map value functions:

```faml
[group]
map = { "a": 1, "b": 2 }
map_len = map.len()       // 2
map_str = map.to_str()    // "{ a: 1, b: 2 }"
```

Duration value functions:

```faml
[group]
dur = 1 seconds
dur_str = dur.to_str()    // "1 seconds"
```

Distance value functions:

```faml
[group]
dist = 1000 meters
dist_km = dist.to_kilometers()  // 1.0
dist_str = dist.to_str()        // "1000 meters"
```