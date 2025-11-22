# Advanced Usage

## Calculation Tracking

Suppose there is a pallet carrier robot with some of its own attributes and some site-related parameters. The robot calculates the actual control distance of the servo through a series of complex formulas based on these parameters. Now, if the result is incorrect, we want to investigate the cause. Sample code is as follows:

```rust
fn main() -> anyhow::Result<()> {
    let faml_str = r#"
[expr.map]
shelf_height                  = 1000 millimeters // Shelf height
pallet_hole_height            = 110 millimeters  // Pallet hole height
lift_pallet_height_with_shelf = 50 millimeters   // Lift pallet height with shelf
pick_height = shelf_height + pallet_hole_height + lift_pallet_height_with_shelf // Recommended pick height

[expr.robot]
fork_base_height = 110 millimeters // Fork base height from ground
chassis_height   = 150 millimeters // Robot chassis height
min_cargo_height = chassis_height + super.map.pallet_hole_height + super.map.lift_pallet_height_with_shelf // Minimum cargo height

[expr]
@if map.pick_height >= robot.min_cargo_height
lift_height = map.pick_height - robot.fork_base_height
@if map.pick_height < robot.min_cargo_height
lift_height = robot.min_cargo_height - robot.fork_base_height
"#;
    let expr = faml::FamlExpr::from_str(faml_str)?;
    let trace = expr["expr"]["lift_height"].trace("lift_height")?;
    println!("{trace}");
    Ok(())
}
```

For complex formulas, calling the trace function will directly output each parameter and its corresponding calculation that contributes to the result. Using it to track down errors will eliminate numerous logging and debugging steps.

The above code outputs the following:

```faml
lift_pallet_height_with_shelf = 50 millimeters
map.pick_height = (shelf_height + pallet_hole_height) + lift_pallet_height_with_shelf // =1.1600000000000001 meters
pallet_hole_height = 110 millimeters
robot.fork_base_height = 110 millimeters
shelf_height = 1000 millimeters // =1 meters
lift_height = map.pick_height - robot.fork_base_height // =1.05 meters
```

## Template Configuration

Suppose there is a standard product model with a series of configuration parameters. Now, based on this standard product, we develop a customized product that shares most configurations with the standard product but has some minor differences. FAML supports template usage, allowing customized parameters to be applied on top of templates. This approach facilitates configuration management. Sample code is as follows:

```rust
fn main() -> anyhow::Result<()> {
    let faml_str1 = r#"
[hello]
a = true
b = 123
c = "hello"
"#;
    let faml_str2 = r#"
[hello]
b = 122
"#;
    let mut expr = faml::FamlExpr::from_str(faml_str1)?;
    expr.apply(faml::FamlExpr::from_str(faml_str2)?)?;
    let val = expr.evaluate()?.to_json();
    println!("{val:?}");
    Ok(())
}
```

The JSON structure corresponding to the `val` in the above code:

```json
{
    "hello": {
        "a": true,
        "b": 122,
        "c": "hello"
    }
}
```

## Composite Application

Assume that the current project already has a complete JSON configuration. Is there a way to directly integrate FAML's expression functionality without modifying the existing configuration? The answer is yes. Sample code:

```rust
fn main() -> anyhow::Result<()> {
    let json_str1 = r#"
{
    "hello": {
        "a": 123,
        "b": 432
    }
}
"#;
    let faml_str2 = r#"
[hello]
expr = a + b
"#;
    let root: serde_json::Value = serde_json::from_str(&json_str1)?;
    let mut expr = faml::FamlExpr::from_json(root)?;
    expr.apply(faml::FamlExpr::from_str(faml_str2)?)?;
    let val = expr["hello"]["expr"].evaluate()?.as_int();
    println!("{val:?}");
    Ok(())
}
```

In the above code, `json_str1` is our original JSON configuration, and `faml_str2` is our FAML formula. As you can see, the formula directly accesses values from the original configuration. The execution result of the above code is:

```
Some(555)
```

## Calling Host Functions

If the value of a configuration item requires complex calculations, host language functions can be called within FAML. Sample code is as follows:

```rust
fn main() -> anyhow::Result<()> {
    faml::Native::add_func("test", |n: i64| n + 10);

    let faml_str = r#"
[hello]
val = native.test(12)
"#;
    let expr = faml::FamlExpr::from_str(faml_str)?;
    let val = expr["hello"]["val"].evaluate()?.as_int();
    println!("{val:?}");
    Ok(())
}
```

First, register the host function through `add_func`, then call it in FAML via `native.function_name`. The result of this code execution is:

```log
Some(22)
```
