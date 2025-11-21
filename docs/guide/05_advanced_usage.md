# 高级用法

## 计算跟踪

现假设有一个托盘运载机器人，它具有一些自身的属性，且还有一些场地相关参数，机器人将这些参数通过一系列复杂的公式计算出伺服器实际控制距离。现发现结果不对，希望排查不对的原因。示例代码如下：

```rust
fn main() -> anyhow::Result<()> {
    let faml_str = r#"
[expr.map]
shelf_height                  = 1000 millimeters // 货位高度
pallet_hole_height            = 110 millimeters  // 托盘插孔高度
lift_pallet_height_with_shelf = 50 millimeters   // 取放货托盘抬升距载货区的高度
pick_height = shelf_height + pallet_hole_height + lift_pallet_height_with_shelf // 建议取放货高度

[expr.robot]
fork_base_height = 110 millimeters // 举升原点位置叉臂上平面距地面距离
chassis_height   = 150 millimeters // 机器人底盘高度
min_cargo_height = chassis_height + super.map.pallet_hole_height + super.map.lift_pallet_height_with_shelf // 最低取放货高度

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

对于复杂公式，调用trace函数将会直接输出结果来源的每一个参数及相应计算，用它跟踪出错原因将会省略大量的打日志、调试等步骤。

上述代码输出如下：

```faml
lift_pallet_height_with_shelf = 50 millimeters
map.pick_height = (shelf_height + pallet_hole_height) + lift_pallet_height_with_shelf // =1.1600000000000001 meters
pallet_hole_height = 110 millimeters
robot.fork_base_height = 110 millimeters
shelf_height = 1000 millimeters // =1 meters
lift_height = map.pick_height - robot.fork_base_height // =1.05 meters
```

## 模板配置

现假设存在一个标准型号的产品，它具有一系列配置参数。现基于此标准产品基础上开发了一款定制产品，它与标准产品大多数配置相同，只存在一些细微的不同。faml支持模板化用法，可以在模板技术上再应用定制化的参数。这样做的好处是便于配置项管理。示例代码如下：

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

上述代码val对应的JSON结构：

```json
{
    "hello": {
        "a": true,
        "b": 122,
        "c": "hello"
    }
}
```

## 组合应用

假设现有项目已经有了完善的json配置，有没办法在不改动现有配置的前提下，直接集成faml的表达式功能呢？答案是肯定的。示例代码：

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

上面代码里，json_str1为我们原先的json配置，faml_str2为我们的faml公式。可见公式直接访问了原配置里的值。上述代码执行结果为：

```
Some(555)
```
